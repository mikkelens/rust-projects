use std::{
	env,
	fs::File,
	io::{stdin, Read, Write},
	ops::{Deref, DerefMut},
	sync::{Arc, Mutex},
	time::Duration
};

use axum::{routing::get, Router};
use ical::{generator::Emitter, parser::ical::component::IcalCalendar, property::Property};
use reqwest::IntoUrl;
use serde::{Deserialize, Serialize};

const CALENDAR_ENV_VAR: &str = "CALENDAR_ICAL_URL";
const ICAL_CACHE_PATH: &str = "latest_processed.ical";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let url = env::var(CALENDAR_ENV_VAR).or_else(|_| {
		let mut s = String::new();
		println!("Please provide a URL (it was not found in environment ");
		print!("> ");
		stdin().read_line(&mut s).map(|_| s)
	})?;

	let calendar: Arc<Mutex<Option<ComparableCalendar>>> = Arc::new(Mutex::new({
		let mut s = String::new();
		match File::open(ICAL_CACHE_PATH) {
			Ok(mut file) => {
				let byte_count = file.read_to_string(&mut s)?;
				println!("Found previous data, read {} bytes from file.", byte_count);
				Some(ComparableCalendar::from(
					ical::IcalParser::new(s.as_bytes())
						.next()
						.ok_or_else(|| anyhow::Error::msg("No first IcalCalendar in file?"))??
				))
			},
			Err(_) => None
		}
	}));
	let writer_calendar = calendar.clone();

	let host = tokio::task::spawn(async move {
		axum::serve(
			tokio::net::TcpListener::bind("127.0.0.1:3000")
				.await
				.unwrap(),
			Router::new().route(
				"/",
				get(|| async move {
					println!("GET requested from server. ");
					match calendar.lock().unwrap().deref() {
						None => "".into(),
						Some(val) => val.source.generate()
					}
				})
			)
		)
		.await
	});

	let writer = tokio::task::spawn(async move {
		let mut calendar_handle = writer_calendar.clone();
		loop {
			match update(&url, &mut calendar_handle).await {
				Ok(_) => {},
				e @ Err(_) => break e
			};
		}
	});

	let (a, b) = tokio::try_join!(host, writer)?;
	a?;
	b?;

	unreachable!()
}

async fn update(
	url: impl IntoUrl,
	calendar_handle: &mut Arc<Mutex<Option<ComparableCalendar>>>
) -> anyhow::Result<()> {
	match reqwest::get(url).await?.error_for_status() {
		Ok(success) => {
			// println!("Success, code {}.", success.status());

			let bytes = success.bytes().await?;
			let new = ComparableCalendar::from(
				ical::IcalParser::new(&bytes[..])
					.next()
					.ok_or_else(|| anyhow::Error::msg("No first IcalCalendar in file?"))??
			);

			// File::create_new(format!(
			//     "timetable_file_{}.ical",
			//     chrono::Utc::now().format("%F_%H.%M.%S")
			// ))
			// .unwrap()
			// .write_all(&bytes[..])
			// .unwrap();

			match calendar_handle.lock().unwrap().deref_mut() {
				// match guard lock may break app
				Some(prev) if *prev == new => {
					println!("Newly fetched ical is identical to previous. No changes needed.")
				},
				should_change => {
					match should_change {
						Some(prev) => {
							*prev = new;
							println!(
								"Updated calendar: Newly parsed ical was different than previous."
							);
						},
						a => {
							*a = Some(new);
							println!("Added initial calendar from parsed ical.")
						}
					}
					File::create(ICAL_CACHE_PATH)?
						.write_all(&bytes[..])
						.unwrap();
					println!(
						"Wrote changes to file of total size: {} bytes.",
						bytes.len()
					)
				}
			}

			let duration = Duration::from_secs(5);
			println!("Waiting {} seconds...", duration.as_secs());
			tokio::time::sleep(duration).await;
		},
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

#[derive(Debug, Serialize, Deserialize)]
struct ComparableCalendar {
	source: IcalCalendar,
	built:  Option<String>
}
impl From<IcalCalendar> for ComparableCalendar {
	fn from(source: IcalCalendar) -> Self {
		Self {
			source,
			built: None
		}
	}
}
impl PartialEq for ComparableCalendar {
	fn eq(&self, other: &Self) -> bool {
		self.cache_relevant_properties()
			.eq(other.cache_relevant_properties())
	}
}
impl ComparableCalendar {
	fn cache_relevant_properties(&self) -> impl Iterator<Item = &Property> {
		// this seems to do what I want
		self.flattened_properties().filter(|&property| {
			!matches!(
				property.name.as_str(),
				"DTSTAMP" | "CREATED" | "LAST-MODIFIED"
			)
		})
	}

	fn flattened_properties(&self) -> impl Iterator<Item = &Property> {
		self.source
			.properties
			.iter()
			.chain(self.source.events.iter().flat_map(|event| {
				event.properties.iter().chain(
					event
						.alarms
						.iter()
						.flat_map(|alarm| alarm.properties.iter())
				)
			}))
			.chain(self.source.todos.iter().flat_map(|todo| {
				todo.properties
					.iter()
					.chain(todo.alarms.iter().flat_map(|alarm| alarm.properties.iter()))
			}))
			.chain(
				self.source
					.alarms
					.iter()
					.flat_map(|alarm| alarm.properties.iter())
			)
			.chain(
				self.source
					.journals
					.iter()
					.flat_map(|journal| journal.properties.iter())
			)
	}

	fn get_built(&mut self) -> String {
		fn prop_filter(property: &&Property) -> bool {
			property
				.value
				.as_ref()
				.is_some_and(|s| s.as_str() == "Beregnelighed og logik")
				&& property.name == "SUMMARY"
		}

		match &self.built {
			None => {
				let new = IcalCalendar {
					properties: self
						.source
						.properties
						.iter()
						.filter(prop_filter)
						.cloned()
						.collect(),
					events:     self
						.source
						.events
						.iter()
						.map(|&event| event.properties.iter().filter(prop_filter).collect())
						.collect(),
					alarms:     vec![],
					todos:      vec![],
					journals:   vec![],
					free_busys: vec![],
					timezones:  vec![]
				};
				let built = new.generate();
				self.built = Some(built.clone());
				built
			},
			Some(val) => val.clone()
		}
	}
}
