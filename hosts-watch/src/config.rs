use std::{path::PathBuf, str::FromStr};

use clap;

#[derive(Debug)]
pub struct AppArgs {
	pub url_target:   url::Url,
	pub file_target:  Option<FileTarget>,
	pub min_wait_ms:  Option<u64>,
	pub mid_wait_ms:  Option<u64>,
	pub max_wait_ms:  Option<u64>,
	pub target_begin: Option<String>,
	pub target_end:   Option<String>
}

#[derive(Debug)]
pub struct Config {
	pub url:          url::Url,
	pub hosts_path:   PathBuf,
	pub min_wait_ms:  u64,
	pub mid_wait_ms:  u64,
	pub max_wait_ms:  u64,
	pub target_begin: String,
	pub target_end:   String
}
impl Config {
	const DEFAULT_MAX_WAIT_MS: u64 = Self::DEFAULT_MIN_WAIT_MS * 2_u64.pow(8);
	const DEFAULT_MID_WAIT_MS: u64 = Self::DEFAULT_MIN_WAIT_MS * 2_u64.pow(6);
	const DEFAULT_MIN_WAIT_MS: u64 = 50;
	const DEFAULT_TARGET_BEGIN: &'static str = "/// ctf_top ///";
	const DEFAULT_TARGET_END: &'static str = "/// ctf_bottom ///";
	const DEFAULT_URL: &'static str = "https://hackeve.haaukins.dk/hosts";
}

// impl TryFrom<AppArgs> for Config {
// 	type Error = OsSupportErr;
//
// 	fn try_from(args: AppArgs) -> Result<Self, Self::Error> {
// 		Ok(Self {
// 			url:          args.url_target,
// 			hosts_path:   args
// 				.file_target
// 				.unwrap_or(FileTarget::Os(Os::from_str(std::env::consts::OS)?))
// 				.to_path()?,
// 			min_wait_ms:  args.min_wait_ms.unwrap_or(Self::DEFAULT_MIN_WAIT_MS),
// 			mid_wait_ms:  args.mid_wait_ms.unwrap_or(Self::DEFAULT_MID_WAIT_MS),
// 			max_wait_ms:  args.max_wait_ms.unwrap_or(Self::DEFAULT_MAX_WAIT_MS),
// 			target_begin: args
// 				.target_begin
// 				.unwrap_or(Self::DEFAULT_TARGET_BEGIN.to_string()),
// 			target_end:   args
// 				.target_end
// 				.unwrap_or(Self::DEFAULT_TARGET_END.to_string())
// 		})
// 	}
//}
#[derive(Debug)]
pub enum FileTarget {
	Os(Os),
	SpecificPath(PathBuf)
}
impl FileTarget {
	fn to_path(self) -> PathBuf {
		match self {
			FileTarget::Os(os) => os.to_path(),
			FileTarget::SpecificPath(path) => path
		}
	}
}
pub struct OsSupportErr;
#[derive(Debug, clap::ValueEnum, Copy, Clone)]
pub enum Os {
	Linux,
	Windows
}
impl Os {
	const LINUX_HOST_FILE_LOCATION: &'static str = "/etc/hosts";
	const WINDOWS_HOST_FILE_LOCATION: &'static str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

	fn to_path(&self) -> PathBuf {
		match self {
			Os::Linux => PathBuf::from(Self::LINUX_HOST_FILE_LOCATION),
			Os::Windows => PathBuf::from(Self::WINDOWS_HOST_FILE_LOCATION)
		}
	}
}
impl FromStr for Os {
	type Err = OsSupportErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"linux" => Os::Linux,
			"windows" => Os::Windows,
			_ => Err(OsSupportErr)?
		})
	}
}

//#[derive(Debug)]
// pub enum ConfigErr {
// 	InvalidOS(String),
// 	InvalidUrl(String, url::ParseError),
// 	InvalidFlag(FlagAddErr)
//}
// impl Display for ConfigErr {
// 	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			ConfigErr::InvalidOS(name) => write!(f, "Operating System '{}' is
// unsupported.", name), 			ConfigErr::InvalidUrl(s, e) => write!(f, "'{}' could
// not be parsed as URL: {}", s, e), 			ConfigErr::InvalidFlag(s) => match s {
// 				FlagAddErr::AlreadySet(s) => write!(f, "The flag '{}' was already set.",
// s), 				FlagAddErr::ExpectedFlag(s) => write!(f, "Expected flag, found
// '{}'.", s), 				FlagAddErr::InvalidFlag(s) => write!(f, "'{}' is not
// recognized as a flag.", s), 				FlagAddErr::InvalidNum(e) => e.fmt(f)
// 			}
// 		}
// 	}
//}
// impl TryFrom<Args> for Config {
// 	type Error = ConfigErr;
//
// 	fn try_from(args: Args) -> Result<Self, Self::Error> {
// 		let mut args = args.skip(1);
//
// 		// if any args are passed, assume that the first is for the target URL
// 		// maybe we should just divide arguments up using `--<flag>` syntax for
// flags 		// so that order is irrelevant
// 		let url = match args.next() {
// 			Some(maybe_url) => match maybe_url.parse::<url::Url>() {
// 				Ok(valid) => valid,
// 				Err(e) => Err(Self::Error::InvalidUrl(maybe_url, e))?
// 			},
// 			None => {
// 				eprintln!(
// 					"Url was not provided, using default ('{}') instead.",
// 					Self::DEFAULT_URL
// 				);
// 				Self::DEFAULT_URL.parse().expect("default url is parsable")
// 			}
// 		};
//
// 		// parse flags
// 		let flags = args
// 			.tuples()
// 			.map(|(key, value)| ConfigFlag::try_from((key.as_str(), value.as_str())))
// 			.collect::<Result<Vec<ConfigFlag>, FlagAddErr>>()
// 			.map_err(Self::Error::InvalidFlag)?;
//
// 		// windows or linux
// 		let hosts_path = match std::env::consts::OS {
// 			"linux" => Self::LINUX_HOST_FILE_LOCATION,
// 			"windows" => Self::WINDOWS_HOST_FILE_LOCATION,
// 			other => Err(Self::Error::InvalidOS(other.to_string()))?
// 		}
// 		.parse()
// 		.expect("OS constants are valid paths");
//
// 		Ok(Self::new(
// 			url,
// 			hosts_path,
// 			ConfigOptions::new(flags).map_err(Self::Error::InvalidFlag)?
// 		))
// 	}
//}
///// Basically a manually state-checked builder.
///// Probably should be type-stated and/or generated?
// struct ConfigOptions {
// 	min_wait_ms:  Option<u64>,
// 	mid_wait_ms:  Option<u64>,
// 	max_wait_ms:  Option<u64>,
// 	target_begin: Option<String>,
// 	target_end:   Option<String>
//}
// impl ConfigOptions {
// 	pub fn new(options: impl IntoIterator<Item = ConfigFlag>) -> Result<Self,
// FlagAddErr> { 		options.into_iter().try_fold(
// 			Self {
// 				min_wait_ms:  None,
// 				mid_wait_ms:  None,
// 				max_wait_ms:  None,
// 				target_begin: None,
// 				target_end:   None
// 			},
// 			|mut acc, flag| {
// 				fn assign_none_or<T, E>(prev: &mut Option<T>, new: T, e: E) -> Result<(),
// E> { 					if prev.is_none() {
// 						*prev = Some(new);
// 						Ok(())
// 					} else {
// 						Err(e)
// 					}
// 				}
// 				// assign option if not already present
// 				// this could maybe be abstracted/generated as type-states
// 				match flag {
// 					ConfigFlag::MinWait(num) => assign_none_or(
// 						&mut acc.min_wait_ms,
// 						num,
// 						FlagAddErr::AlreadySet(ConfigFlag::MIN_WAIT_FLAG.to_string())
// 					)?,
// 					ConfigFlag::MidWait(num) => assign_none_or(
// 						&mut acc.mid_wait_ms,
// 						num,
// 						FlagAddErr::AlreadySet(ConfigFlag::MID_WAIT_FLAG.to_string())
// 					)?,
// 					ConfigFlag::MaxWait(num) => assign_none_or(
// 						&mut acc.max_wait_ms,
// 						num,
// 						FlagAddErr::AlreadySet(ConfigFlag::MAX_WAIT_FLAG.to_string())
// 					)?,
// 					ConfigFlag::TargetBegin(pat) => assign_none_or(
// 						&mut acc.target_begin,
// 						pat,
// 						FlagAddErr::AlreadySet(ConfigFlag::TARGET_BEGIN_FLAG.to_string())
// 					)?,
// 					ConfigFlag::TargetEnd(pat) => assign_none_or(
// 						&mut acc.target_end,
// 						pat,
// 						FlagAddErr::AlreadySet(ConfigFlag::TARGET_END_FLAG.to_string())
// 					)?
// 				}
// 				Ok(acc)
// 			}
// 		)
// 	}
//}
//#[derive(Debug)]
// enum ConfigFlag {
// 	MinWait(u64),
// 	MidWait(u64),
// 	MaxWait(u64),
// 	TargetBegin(String),
// 	TargetEnd(String)
//}
// impl ConfigFlag {
// 	const FLAG_PATTERN: &'static str = "--";
// 	const MAX_WAIT_FLAG: &'static str = "max_wait";
// 	const MID_WAIT_FLAG: &'static str = "mid_wait";
// 	// waiting
// 	const MIN_WAIT_FLAG: &'static str = "min_wait";
// 	// searching
// 	const TARGET_BEGIN_FLAG: &'static str = "target_begin";
// 	const TARGET_END_FLAG: &'static str = "target_end";
//}
//#[derive(Debug)]
// pub enum FlagAddErr {
// 	AlreadySet(String),
// 	ExpectedFlag(String),
// 	InvalidFlag(String),
// 	InvalidNum(ParseIntError)
//}
// impl TryFrom<(&str, &str)> for ConfigFlag {
// 	type Error = FlagAddErr;
//
// 	fn try_from((flag_raw, value): (&str, &str)) -> Result<Self, Self::Error> {
// 		if !flag_raw.starts_with(Self::FLAG_PATTERN) {
// 			Err(Self::Error::ExpectedFlag(flag_raw.to_string()))?
// 		};
// 		let parse = |value: &str| value.parse().map_err(Self::Error::InvalidNum);
// 		Ok(match flag_raw.trim_start_matches(Self::FLAG_PATTERN) {
// 			Self::MIN_WAIT_FLAG => Self::MinWait(parse(value)?),
// 			Self::MID_WAIT_FLAG => Self::MidWait(parse(value)?),
// 			Self::MAX_WAIT_FLAG => Self::MaxWait(parse(value)?),
// 			Self::TARGET_BEGIN_FLAG => Self::TargetBegin(value.to_string()),
// 			Self::TARGET_END_FLAG => Self::TargetEnd(value.to_string()),
// 			unrecognized => Err(Self::Error::InvalidFlag(unrecognized.to_string()))?
// 		})
// 	}
//}
