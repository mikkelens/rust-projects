use std::fmt::{Display, Formatter};
use std::io::BufRead;

fn main() {
    let num = efficient_get();
    println!("{}", WeekDuration(num));
}

#[allow(unused)]
fn efficient_get() -> u64 {
    fn inform_invalid_clear(container: &mut String) {
        println!("'{}' is not a valid u64.", container);
        container.clear();
    }
    match std::env::args().next().map(|s| (s.parse(), s)) {
        Some((Ok(num), _)) => num,
        invalid => {
            let mut input_container = match invalid {
                None => {
                    println!("No number/argument provided.");
                    String::new() // new container
                }
                Some(mut a) => {
                    inform_invalid_clear(&mut a.1);
                    a.1 // reuse
                }
            };
            loop {
                println!("Please provide a number (u64).");
                print!("> ");
                let _ = std::io::stdin()
                    .lock()
                    .read_line(&mut input_container)
                    .expect("Could not read from stdin?");
                if let Ok(num) = input_container.trim().parse() {
                    break num; // valid input received
                }
                inform_invalid_clear(&mut input_container);
                // try again
            }
        }
    }
}

#[allow(unused)]
fn funny_get() -> u64 {
    std::env::args()
        .chain(
            std::iter::once_with(|| {
                println!("Please provide a number that is a u64.");
                print!("> ");
                let mut s = String::new();
                std::io::stdin().lock().read_line(&mut s);
                s
            })
            .cycle(),
        )
        .filter_map(|s| s.trim().parse().ok())
        .next()
        .expect("cyclic iterator never ends")
}

struct WeekDuration(u64);
impl WeekDuration {
    const SECONDS_IN_MINUTE: u64 = 60;
    const SECONDS_IN_HOUR: u64 = Self::SECONDS_IN_MINUTE * 60;
    const SECONDS_IN_DAY: u64 = Self::SECONDS_IN_HOUR * 24;
    const SECONDS_IN_WEEK: u64 = Self::SECONDS_IN_DAY * 7;
}
impl Display for WeekDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut seconds = self.0;
        let weeks = seconds / Self::SECONDS_IN_WEEK;
        seconds %= Self::SECONDS_IN_WEEK;
        let days = seconds / Self::SECONDS_IN_DAY;
        seconds %= Self::SECONDS_IN_DAY;
        let hours = seconds / Self::SECONDS_IN_HOUR;
        seconds %= Self::SECONDS_IN_HOUR;
        let minutes = seconds / Self::SECONDS_IN_MINUTE;
        seconds %= Self::SECONDS_IN_MINUTE;
        write!(
            f,
            "{} weeks, {} days, {} hours, {} minutes and {} seconds.",
            weeks, days, hours, minutes, seconds
        )
    }
}
