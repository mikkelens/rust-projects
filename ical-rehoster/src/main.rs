use std::env;
use std::fs::File;
use std::io::{stdin, Write};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::routing::get;
use axum::Router;
use ical::generator::Emitter;
use ical::parser::ical::component::IcalCalendar;
use ical::property::Property;
use reqwest::IntoUrl;

const CALENDAR_ENV_VAR: &str = "CALENDAR_ICAL_URL";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = env::var(CALENDAR_ENV_VAR).or_else(|_| {
        let mut s = String::new();
        println!("Please provide a URL (it was not found in environment ");
        print!("> ");
        stdin().read_line(&mut s).map(|_| s)
    })?;

    let calendar: Arc<Mutex<Option<ComparableCalendar>>> = Arc::new(Mutex::new(None));
    let writer_calendar = calendar.clone();

    let host = tokio::task::spawn(async move {
        axum::serve(
            tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
            Router::new().route(
                "/",
                get(|| async move {
                    println!("GET requested from server. ");
                    match calendar.lock().unwrap().deref() {
                        None => "".into(),
                        Some(val) => val.0.generate(),
                    }
                }),
            ),
        )
        .await
    });

    let writer = tokio::task::spawn(async move {
        let mut calendar_handle = writer_calendar.clone();
        loop {
            match update(&url, &mut calendar_handle).await {
                Ok(_) => {}
                e @ Err(_) => break e,
            };
        }
    });

    let (_, _) = tokio::join!(host, writer);

    unreachable!()
}

async fn update(
    url: impl IntoUrl,
    calendar_handle: &mut Arc<Mutex<Option<ComparableCalendar>>>,
) -> anyhow::Result<()> {
    match reqwest::get(url).await?.error_for_status() {
        Ok(success) => {
            println!("Success, code {}.", success.status());

            let bytes = success.bytes().await?;
            let new = ComparableCalendar(
                ical::IcalParser::new(&bytes[..])
                    .next()
                    .ok_or_else(|| anyhow::Error::msg("No first IcalCalendar in file?"))??,
            );

            File::create_new(format!(
                "timetable_file_{}.ical",
                chrono::Utc::now().format("%F_%H.%M.%S")
            ))
            .unwrap()
            .write_all(&bytes[..])
            .unwrap();

            match calendar_handle.lock().unwrap().deref_mut() {
                // match guard lock may break app
                Some(prev) if *prev == new => {
                    println!("Newly fetched ical is identical to previous. No changes needed.")
                }
                Some(prev) => {
                    *prev = new;
                    println!("Updated calendar from parsed ical.");
                }
                a @ None => {
                    *a = Some(new);
                    println!("Added initial calendar from parsed ical.")
                }
            }

            let duration = Duration::from_secs(3);
            println!("Waiting {} seconds...", duration.as_secs());
            tokio::time::sleep(duration).await;
        }
        Err(error) => {
            println!("{}", error);
            if let Some(code) = error.status() {
                println!("Status code for request: {}", code)
            }
            let duration = Duration::from_secs(5);
            println!("Trying again in {} seconds...", duration.as_secs());
            tokio::time::sleep(duration).await;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct ComparableCalendar(IcalCalendar);
impl PartialEq for ComparableCalendar {
    fn eq(&self, other: &Self) -> bool {
        self.counted_properties()
            .eq(other.counted_properties())
    }
}
impl ComparableCalendar {
    fn counted_properties(&self) -> impl Iterator<Item = &Property> {
        self.flattened_properties()
            .filter(|property| !matches!(property.name.as_str(), "DTSTAMP" | "CREATED" | "LAST-MODIFIED"))
    }
    fn flattened_properties(&self) -> impl Iterator<Item = &Property> {
        self.0
            .properties
            .iter()
            .chain(self.0.events.iter().flat_map(|event| {
                event.properties.iter().chain(
                    event
                        .alarms
                        .iter()
                        .flat_map(|alarm| alarm.properties.iter()),
                )
            }))
    }
}
