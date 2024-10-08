#![feature(assert_matches)]

mod config;

use std::{
	collections::HashSet,
	convert::Infallible,
	fmt::{Display, Formatter},
	io::{BufRead, BufReader, Write},
	str::FromStr,
	time::Duration
};

use clap::{Args::*, *};
use tokio::{self, io::AsyncBufReadExt};
use url::Host;

use crate::config::AppArgs;

enum ProgramErr {
	Config(AppArgs),
	Runtime(std::io::Error),
	FileSystem(std::io::Error)
}

fn main() -> std::process::ExitCode {
	let Err(res) = run(); // infallible
	println!("Program error occurred, {}", match res {
		ProgramErr::Config(config_err) => {
			format!("failed to initialize configuration:\n{:?}", config_err)
		},
		ProgramErr::Runtime(runtime_err) => {
			format!("failed to initialize tokio runtime:\n{}", runtime_err)
		},
		ProgramErr::FileSystem(io_err) => {
			format!("failed to perform filesystem IO.\n{}", io_err)
		}
	});
	std::process::ExitCode::FAILURE
}

fn run() -> Result<Infallible, ProgramErr> {
	let command = Command::new("hosts-watch (builder config)")
		.about("Watching hosts files")
		// 		.subcommand_required(true)
		.arg_required_else_help(true)
		// 		.allow_external_subcommands(true)
		.arg(Arg::new("url"))
		.subcommand(
			Command::new("os")
				.about("What OS to target(windows/linux)")
				.arg_required_else_help(true)
				.args([
					Arg::new("linux")
						.short('l')
						.long("linux")
						.action(ArgAction::Set),
					Arg::new("windows")
						.short('w')
						.long("windows")
						.action(ArgAction::Set)
				])
		);
	//    let config =
	// Config::try_from(std::env::args()).map_err(ProgramErr::Config)?;    let rt =
	tokio::runtime::Runtime::new().map_err(ProgramErr::Runtime)?;
	// 	println!(
	// 		"Welcome to hosts-watch.\nParams:"
	// 	);
	// 	println!("* Target URL: '{}'...", &config.url);
	// 	println!("* Target hosts path: '{:?}'...", &config.hosts_path);
	let mut ms_to_wait = config.min_wait_ms;
	loop {
		println!();
		rt.block_on(async { refresh_state_from_web(&config, &mut ms_to_wait).await })
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
		rt.block_on(async {
			tokio::select! { // race each task, skipping wait from key-press
				_ = &mut interrupt_task => {}
				_ = &mut sleep_task => {}
			}
		})
	}
}

pub async fn refresh_state_from_web(config: &Config, ms_to_wait: &mut u64) -> std::io::Result<()> {
	match reqwest::get(config.url.as_str()).await {
		Ok(response) => {
			let s = response
				.text()
				.await
				.expect("idk why a response wouldn't have text here");
			let entries: HostFileEntries = scraper::Html::parse_fragment(&s).into();
			if entries.0.is_empty() {
				println!(
					"Could not find any entries at the page/url '{}'.",
					&config.url
				);
				*ms_to_wait = u64::max(*ms_to_wait * 2, config.max_wait_ms);
				println!("Doubling wait time...");
			} else {
				let mut file = std::fs::File::open(&config.hosts_path)?;
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
		},
		_ => {
			*ms_to_wait = u64::max(*ms_to_wait * 2, config.max_wait_ms);
			println!("Doubling wait time...");
		}
	}
	Ok(())
}

async fn reset_action() {
	let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
	loop {
		let mut buffer = String::new();
		let _fut = reader.read_line(&mut buffer).await;
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
		//    let p_selector = Selector::parse("p").unwrap();
		let li_selector = scraper::Selector::parse("li").unwrap();
		let vec = fragment
			.select(&li_selector)
			.filter_map(|e_r| {
				e_r.text()
					.next()? // ?: idk if this is the best solution, needs testing
					.parse()
					.inspect_err(|e| {
						eprintln!("Tried and failed to parse element reference:\n{:?}\n", e)
					})
					.ok()
			})
			.collect();
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
		match s.split_once(' ') {
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
