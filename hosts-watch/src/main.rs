#![feature(assert_matches)]

mod config;

use std::{
	collections::HashSet,
	convert::Infallible,
	fmt::{Display, Formatter},
	fs::OpenOptions,
	io::{BufRead, BufReader, Write},
	str::FromStr,
	time::Duration
};

use headless_chrome::{
	protocol::cdp::Network::{CookieParam, CookieSameSite},
	Browser, LaunchOptions
};
use itertools::Itertools;
use tokio::{self};
use url::Host;

use crate::config::{Config, ConfigErr};

enum ProgramErr {
	Config(ConfigErr),
	// 	Runtime(std::io::Error),
	FileSystem(std::io::Error)
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
	let Err(res) = run().await; // infallible
	println!("Program error occurred, {}", match res {
		ProgramErr::Config(config_err) => {
			format!("failed to initialize configuration:\n{}", config_err)
		},
		// 		ProgramErr::Runtime(runtime_err) => {
		// 			format!("failed to initialize tokio runtime:\n{}", runtime_err)
		// 		},
		ProgramErr::FileSystem(io_err) => {
			format!("failed to perform filesystem IO.\n{}", io_err)
		}
	});
	std::process::ExitCode::FAILURE
}

async fn run() -> Result<Infallible, ProgramErr> {
	let config = Config::try_from(std::env::args()).map_err(ProgramErr::Config)?;
	println!("Welcome to hosts-watch.\nParams:");
	println!("* Target URL: '{}'...", &config.url);
	println!("* Target hosts path: '{:?}'...", &config.hosts_path);
	let mut ms_to_wait = config.min_wait_ms;
	let browser = &mut Browser::new(LaunchOptions {
		headless: false,
		..Default::default()
	})
	.unwrap();
	let tab = {
		let tab = browser.new_tab().unwrap();
		// Type "Tab" is effectively a mutable reference, but rust doesn't know that
		// 		tab.navigate_to("https://prosa.haaukins.dk").unwrap();
		tab.navigate_to(format!("https://{}", config.url.domain().unwrap()).as_str())
			.unwrap();
		tab.wait_until_navigated().unwrap();
		// Set a value in local storage
		let set_local_storage_script = format!("localStorage.setItem('token', '{}')", config.token);
		tab.evaluate(set_local_storage_script.as_str(), false)
			.unwrap();
		eprintln!("Configured storage (token=...)");
		// 		tab.navigate_to(config.url.as_str()).unwrap();
		// 		tab.reload(true, None).unwrap();
		tab.navigate_to(config.url.as_str()).unwrap();
		tab.wait_until_navigated().unwrap();
		tab
	};

	loop {
		println!();
		refresh_state_from_web(
			tab.get_content().unwrap().as_str(),
			&config,
			&mut ms_to_wait
		)
		.await
		.map_err(ProgramErr::FileSystem)?;
		let duration = Duration::from_millis(ms_to_wait);
		println!(
			"Waiting for {} seconds unless interrupted ('r').",
			duration.as_secs_f32()
		);
		let sleep_task = tokio::time::sleep(duration);
		tokio::pin!(sleep_task);
		let interrupt_task = reset_action();
		tokio::pin!(interrupt_task);
		tokio::select! { // race each task, skipping wait from key-press
			_ = &mut interrupt_task => {}
			_ = &mut sleep_task => {}
		}
	}
}

pub async fn refresh_state_from_web(
	fragment: &str,
	config: &Config,
	ms_to_wait: &mut u64
) -> std::io::Result<()> {
	{
		let entries: HostFileEntries = scraper::Html::parse_fragment(&fragment).into();
		if entries.0.is_empty() {
			println!(
				"Could not find any entries at the page/url '{}'.",
				&config.url
			);
			*ms_to_wait = u64::max(*ms_to_wait * 2, config.max_wait_ms);
			println!("Doubling wait time...");
		} else {
			let mut file = OpenOptions::new().write(true).open(&config.hosts_path)?;
			// TODO: below line crashes even with an administrator shell!
			let mut reader_lines = BufReader::new(&file).lines().map(|l| l.unwrap());
			let mut other_content = (&mut reader_lines)
				.take_while(|l| *l != config.target_begin)
				.collect::<HashSet<_>>();
			let prev_entries = HostFileEntries(
				(&mut reader_lines)
					.take_while(|l| *l != config.target_end)
					.filter_map(|l| l.parse::<Entry>().ok())
					.collect()
			);
			other_content.extend(reader_lines);

			if entries != prev_entries {
				*ms_to_wait = config.min_wait_ms; // reset
				other_content.extend(entries.0.iter().map(|entry| entry.to_string()));
				file.write_all(
					other_content
						.into_iter()
						.collect::<Vec<_>>()
						.join("\n")
						.as_bytes()
				)?;
				println!("Updated hosts file:");
				for entry in entries.0.difference(&prev_entries.0) {
					println!("+ '{}'", entry);
				}
			} else {
				*ms_to_wait = config.mid_wait_ms;
				println!("No difference between last check.");
			}
		}
		// 		_ => {
		// 			*ms_to_wait = u64::max(*ms_to_wait * 2, config.max_wait_ms);
		// 			println!("Doubling wait time...");
		// 		}
	}
	Ok(())
}

async fn reset_action() {
	let reader = tokio::io::BufReader::new(tokio::io::stdin());
	loop {
		let mut buffer = String::new();
		let _bytes_count = reader.buffer().read_line(&mut buffer);
		if buffer.contains("r") {
			println!("Pressed 'r' to continue.");
			break;
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
struct HostFileEntries(HashSet<Entry>);
impl From<scraper::Html> for HostFileEntries {
	fn from(fragment: scraper::Html) -> Self {
		eprintln!("HTML: {}\n\n\n", fragment.html());
		//    let p_selector = Selector::parse("p").unwrap();
		let selector = scraper::Selector::parse("p").unwrap();
		let vec = fragment
			.select(&selector)
			.filter_map(|e_r| {
				let inner = e_r.inner_html();
				println!("Finding e_r:\n - dbg: {:?}\n - {}", e_r, inner);
				inner
					.parse()
					.inspect_err(|e| {
						eprintln!("Tried and failed to parse element reference:\n{:?}\n", e)
					})
					.ok()
			})
			.collect();
		eprintln!();
		HostFileEntries(vec)
	}
}

/// Found in https://hackeve.haaukins.dk/hosts as `127.0.0.1 sanity-checks.hkn`
#[derive(Debug, Eq, PartialEq, Hash)]
struct Entry(IPv4, Host);
#[derive(Debug, Eq, PartialEq)]
enum EntryParseErr {
	SplitError,
	IPv4(IPv4ParseErr),
	Url(url::ParseError),
	Both {
		ipv4_e: IPv4ParseErr,
		host_e: url::ParseError
	}
}
impl FromStr for Entry {
	type Err = EntryParseErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.split_whitespace().tuples().next() {
			None => Err(Self::Err::SplitError),
			Some((ipv4_s, url_s)) => match (ipv4_s.parse::<IPv4>(), Host::parse(url_s)) {
				(Ok(ipv4), Ok(host)) => Ok(Entry(ipv4, host)),
				(Ok(_), Err(host_e)) => Err(Self::Err::Url(host_e)),
				(Err(ipv4_e), Ok(_)) => Err(Self::Err::IPv4(ipv4_e)),
				(Err(ipv4_e), Err(host_e)) => Err(Self::Err::Both { ipv4_e, host_e })
			}
		}
	}
}
impl Display for Entry {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {}", self.0, self.1)
	}
}

// e.g. 192.168.1.255
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct IPv4([u8; 4]);
#[derive(Debug, Eq, PartialEq)]
enum IPv4ParseErr {
	ByteParseError(std::num::ParseIntError),
	IncorrectLength
}
impl FromStr for IPv4 {
	type Err = IPv4ParseErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s
			.split('.')
			.map(|s| s.parse::<u8>())
			.collect::<Result<Vec<_>, _>>()
		{
			Ok(nums) => match nums.try_into() {
				Ok(array) => Ok(IPv4(array)),
				Err(_) => Err(IPv4ParseErr::IncorrectLength)
			},
			Err(int_e) => Err(Self::Err::ByteParseError(int_e))
		}
	}
}
impl Display for IPv4 {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
	}
}

#[cfg(test)]
mod tests {
	use super::{Entry, EntryParseErr, IPv4, IPv4ParseErr};
	mod ipv4 {
		use std::assert_matches::assert_matches;

		use super::{IPv4, IPv4ParseErr};

		#[test]
		fn parsing_works() {
			let ips = [
				("192.168.1.255", [192, 168, 1, 255]),
				("192.168.1.23", [192, 168, 1, 23]),
				("127.0.0.1", [127, 0, 0, 1])
			];
			for (s, bytes) in ips {
				let ip = IPv4(bytes);
				// test parsing works
				assert_eq!(s.parse(), Ok(ip));
				// display works
				assert_eq!(ip.to_string(), s);
				// round-trip
				assert_eq!(ip.to_string().parse(), Ok(ip));
				assert_eq!(
					s.parse::<IPv4>().map(|ipv4| ipv4.to_string()),
					Ok(s.to_owned())
				);
			}
		}

		#[test]
		fn parsing_fails() {
			assert_matches!(
				"192.168.255".parse::<IPv4>(),
				Err(IPv4ParseErr::IncorrectLength)
			);
			assert_matches!(
				"192.168.1.256".parse::<IPv4>(),
				Err(IPv4ParseErr::ByteParseError(_))
			);
		}
	}

	mod entry {
		use std::assert_matches::assert_matches;

		use url::Host;

		use super::{Entry, EntryParseErr};
		use crate::{IPv4, IPv4ParseErr};
		#[test]
		fn parsing_works() {
			let string = "192.168.1.255 test.haaukins.hkn";
			let entry = Entry(
				IPv4([192, 168, 1, 255]),
				Host::parse("test.haaukins.hkn").unwrap()
			);
			assert_eq!(entry.to_string(), string);
			assert_eq!(string.parse(), Ok(entry));
		}
		#[test]
		fn parsing_fails() {
			assert_eq!(
				"192.168.1.255test.haaukins.hkn".parse::<Entry>(),
				Err(EntryParseErr::SplitError)
			);
			assert_matches!(
				"192.168.1. test.haaukins.hkn".parse::<Entry>(),
				Err(EntryParseErr::IPv4(IPv4ParseErr::ByteParseError(_)))
			);
			assert_matches!(
				"192.168.1 1test.haaukins.hkn".parse::<Entry>(),
				Err(EntryParseErr::IPv4(IPv4ParseErr::IncorrectLength))
			);
			assert_matches!(
				"192.168.1.255 test:hkn".parse::<Entry>(),
				Err(EntryParseErr::Url(_))
			);
			assert_matches!(
				"192.168.125 haaukins<test.hkn".parse::<Entry>(),
				Err(EntryParseErr::Both {
					ipv4_e: IPv4ParseErr::IncorrectLength,
					host_e: _
				})
			);
		}
	}
}
