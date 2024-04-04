use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::routing::get;
use axum::Router;
use reqwest::{IntoUrl, Response};
use tokio::time::Sleep;

use calendar::ComparableCalendar;

mod calendar;

const CALENDAR_ENV_VAR: &str = "CALENDAR_ICAL_URL";
const ICAL_CACHE_PATH: &str = "latest_processed.ical";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // get data handle
    let calendar_handle: Arc<Mutex<Option<ComparableCalendar>>> = Arc::new(Mutex::new({
        let mut s = String::new();
        match File::open(ICAL_CACHE_PATH) {
            Ok(mut file) => {
                // use pre-lived calendar data if possible
                let byte_count = file.read_to_string(&mut s)?;
                println!("Found previous data, read {} bytes from file.", byte_count);
                Some(ComparableCalendar::from(
                    ical::IcalParser::new(s.as_bytes())
                        .next()
                        .ok_or_else(|| anyhow::Error::msg("No (first) IcalCalendar in file?"))??,
                ))
            }
            Err(_) => None, // no previous file
        }
    }));

    println!();

    // get the target/update url
    let url = match env::var(CALENDAR_ENV_VAR) {
        Ok(s) => {
            println!("URL `{}` found from environment variable", s);
            s
        }
        Err(_) => {
            println!(
                "URL not found in environment variable `{}`... ",
                CALENDAR_ENV_VAR
            );
            match env::args().nth(1 /* skip directory argument */) {
                Some(s) => {
                    println!("Found passed argument `{}`, using it as fetch URL.", s);
                    s
                }
                None => {
                    println!("Please provide a fetch URL.");
                    print!("> ");
                    let mut s = String::new();
                    stdin().read_line(&mut s).map(|_| s)? // read line into string
                }
            }
        }
    };

    println!();
    // create tasks to poll
    let (fetch_write_task, rehost_task) = (
        // fetching
        tokio::task::spawn({
            // make copy of handle for writing task (cannot move pre-clone)
            let calendar_write_handle = calendar_handle.clone();
            async move {
                let mut calendar_handle = calendar_write_handle;
                loop {
                    match fetch_ical(&url, &mut calendar_handle).await {
                        Ok(_) => {} // continue
                        Err(e) => {
                            eprintln!("Error in fetch task:\n{}", e);
                            break e;
                        }
                    };
                }
            }
        }),
        // providing
        tokio::task::spawn(async move {
            axum::serve(
                tokio::net::TcpListener::bind("127.0.0.1:1234")
                    .await
                    .unwrap(),
                Router::new().route(
                    "/",
                    get(|| async move {
                        println!("GET requested from server. ");
                        match calendar_handle.lock().unwrap().deref() {
                            None => "".into(),
                            Some(val) => val.filtered.clone(),
                        }
                    }),
                ),
            )
            .await
        }),
    );

    let (a, b) = tokio::try_join!(rehost_task, fetch_write_task)?;
    a?;
    Err(b)
}

async fn fetch_ical(
    url: impl IntoUrl,
    calendar_handle: &mut Arc<Mutex<Option<ComparableCalendar>>>,
) -> anyhow::Result<()> {
    match reqwest::get(url)
        .await
        .and_then(|response| response.error_for_status())
    {
        Ok(valid_response) => {
            if let Err(e) = try_update(calendar_handle, valid_response).await {
                eprintln!("Response was valid, but failed update - {}", e)
            };

            let valid_delay = Duration::from_secs(10);
            wait_delay("delay before next update...", valid_delay).await;
        }
        Err(e) => {
            eprintln!("Request error - {}", e);
            if let Some(code) = e.status() {
                eprintln!("Status code for request: {}", code)
            }
            let err_delay = Duration::from_secs(5);
            wait_delay("delay before trying again...", err_delay).await;
        }
    };
    Ok(())
}

fn wait_delay(msg: impl Display, duration: Duration) -> Sleep {
    println!("WAIT {} SECONDS: {}", duration.as_secs(), msg);
    tokio::time::sleep(duration)
}

async fn try_update(
    calendar_handle: &mut Arc<Mutex<Option<ComparableCalendar>>>,
    success: Response,
) -> anyhow::Result<()> {
    let bytes = success.bytes().await?;
    let new = ComparableCalendar::from(
        ical::IcalParser::new(&bytes[..])
            .next()
            .ok_or_else(|| anyhow::Error::msg("No first IcalCalendar in fetched bytes?"))??,
    );

    // try to add/change calendar
    match calendar_handle.lock().unwrap().deref_mut() {
        // match guard lock may break app(?)
        Some(prev) if *prev == new => {
            println!("Newly fetched ical is identical to previous, no changes needed.")
        }
        should_change => {
            match should_change {
                Some(prev) => {
                    *prev = new;
                    println!("Updated calendar: Newly parsed ical was different than previous.");
                }
                none_ref @ None => {
                    *none_ref = Some(new);
                    println!("Added initial calendar from parsed ical.")
                }
            }
            File::create(ICAL_CACHE_PATH)?.write_all(&bytes[..])?;
            println!(
                "Wrote changes to file of total size: {} bytes.",
                bytes.len()
            )
        }
    }
    Ok(())
}
