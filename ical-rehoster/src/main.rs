use std::env;
use std::io::stdin;
use std::time::Duration;

use ical::parser::ical::component::IcalCalendar;
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

    let mut calendar = None;
    loop {
        tokio::try_join!(update_calendar(&url, &mut calendar))?;
    }
}

async fn update_calendar(
    url: impl IntoUrl,
    calendar: &mut Option<IcalCalendar>,
) -> anyhow::Result<()> {
    match reqwest::get(url).await?.error_for_status() {
        Ok(success) => {
            println!("Success, code {}.", success.status());

            let bytes = success.bytes().await?;

            let new = ical::IcalParser::new(&bytes[..])
                .next()
                .ok_or_else(|| anyhow::Error::msg("No first IcalCalendar in file?"))??;

            fn calendar_eq(a: &IcalCalendar, b: &IcalCalendar) -> bool {
                format!("{:?}", a) == format!("{:?}", b)
            }
            match &calendar {
                Some(prev) if calendar_eq(prev, &new) => {
                    println!("Newly fetched ical is identical to previous. No changes needed.")
                }
                Some(_) | None => *calendar = Some(new),
            }

            let duration = Duration::from_secs(10);
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
