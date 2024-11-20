use std::fmt::{Display, Formatter};

const MIN_INNER_SYMBOLS: &[char] = &['4', 'w', 'x', 'v', 'b', 'T', 'Y', 'H', 'J', 'K', 'N', '-'];
const EXTRA: &str = "HKN";

struct Flag<'a>(&'a str);
impl Display for Flag<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{{{}}}", EXTRA, self.0)
	}
}

fn main() {
	println!("Hello, world!");
}

/// POST target: `https://aubergine.hkn/cupon
/// payload: `cupon=<data>`
/// HTTP-status responses:
/// - 303 (partial)
/// - 301 (none)
/// - 302 (complete)
fn send() {}
