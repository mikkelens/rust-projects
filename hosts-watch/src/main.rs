use std::str::FromStr;
use std::time::Duration;

// note: it might be possible to look for a signal instead of naively continuously looking for
// changes
fn main() {
    let mut seconds_to_wait_for = 1;
    loop {
        let html = reqwest::blocking::get("https://hackeve.haaukins.dk/hosts");
        if let Ok(response) = html {
            seconds_to_wait_for = 1;
            let s = response.text().unwrap();
            let entries: HostFileEntries = scraper::Html::parse_fragment(&s).into();
            // todo: manipulate hosts file
        }
        std::thread::sleep(Duration::from_secs(seconds_to_wait_for));
        seconds_to_wait_for *= 2;
    }
}

#[derive(Debug)]
struct HostFileEntries(Vec<Entry>);
impl From<scraper::Html> for HostFileEntries {
    fn from(fragment: scraper::Html) -> Self {
        //    let p_selector = Selector::parse("p").unwrap();
        let li_selector = scraper::Selector::parse("li").unwrap();
        let vec = fragment
            .select(&li_selector)
            .filter_map(|e_r| {
                e_r.text()
                    .try_into()
                    .inspect_err(|e| {
                        eprintln!("Tried and failed to parse element reference:\n{:?}\n", e)
                    })
                    .ok()
            })
            .collect();
        HostFileEntries(vec)
    }
}

// found in https://hackeve.haaukins.dk/hosts as "127.0.0.1 sanity-checks.hkn"
#[derive(Debug)]
struct Entry(IPv4, url::Url);
#[derive(Debug)]
enum EntryParseErr {
    NoTextElements,
    SplitError,
    IPv4(IPv4ParseErr),
    Url(url::ParseError),
    Both {
        ipv4_e: IPv4ParseErr,
        url_e: url::ParseError,
    },
}
impl<'a> TryFrom<scraper::element_ref::Text<'a>> for Entry {
    type Error = EntryParseErr;

    fn try_from(mut text: scraper::element_ref::Text) -> Result<Self, Self::Error> {
        match text.next() {
            None => Err(Self::Error::NoTextElements),
            Some(s) => match s.split_once(' ') {
                None => Err(Self::Error::SplitError),
                Some((ipv4_s, url_s)) => {
                    match (ipv4_s.parse::<IPv4>(), reqwest::Url::parse(url_s)) {
                        (Ok(ipv4), Ok(url)) => Ok(Entry(ipv4, url)),
                        (Ok(ipv4), Err(url_e)) => Err(Self::Error::Url(url_e)),
                        (Err(ipv4_e), Ok(url)) => Err(Self::Error::IPv4(ipv4_e)),
                        (Err(ipv4_e), Err(url_e)) => Err(Self::Error::Both { ipv4_e, url_e }),
                    }
                }
            },
        }
    }
}
// e.g. 192.168.1.255
#[derive(Debug)]
struct IPv4([u8; 4]);
#[derive(Debug)]
enum IPv4ParseErr {
    ByteParseError(std::num::ParseIntError),
    IncorrectLength,
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
                Err(_) => Err(IPv4ParseErr::IncorrectLength),
            },
            Err(int_e) => Err(Self::Err::ByteParseError(int_e)),
        }
    }
}