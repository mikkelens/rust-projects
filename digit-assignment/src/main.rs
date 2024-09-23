//! calculate how many numbers in `1..=n` have a specific digit `c`

use std::io::Write;

/// Single digit in base 10.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Digit(u32);
impl std::ops::Deref for Digit {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
enum DigitParseErr {
    TooLong,
}
impl TryFrom<u32> for Digit {
    type Error = DigitParseErr;

    fn try_from(num: u32) -> Result<Self, Self::Error> {
        if num.checked_ilog10().unwrap_or(0) == 0 {
            Ok(Self(num))
        } else {
            Err(Self::Error::TooLong)
        }
    }
}

fn get_numbers() -> (u32, Digit) {
    /// Takes CLI args, and if that's not enough, ask user for input.
    /// Input is parsed with `p` before being accepted and returned.
    fn get_desired<T>(
        iter: &mut impl Iterator<Item = String>,
        t: &'static str,
        p: fn(String) -> Option<T>,
    ) -> T {
        iter.chain(
            std::iter::once_with(|| {
                println!("Please provide a number. ({}: u64)", t);
                std::io::stdout().flush().unwrap();
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                s
            })
            .cycle(),
        )
        .filter_map(|s: String| p(s))
        .next()
        .expect("completed by user eventually")
    }
    fn parse_num(s: String) -> Option<u32> {
        s.trim()
            .parse::<u32>()
            .inspect_err(|e| {
                println!("Input '{}' is not a valid number: {:?}", s, e.kind());
            })
            .ok()
    }

    let mut rem_args = std::env::args().skip(1); // first element is path/unrelated
    let n = get_desired::<u32>(&mut rem_args, "n", |s| {
        parse_num(s)
            .map(|num| {
                if num >= 1 {
                    Some(num)
                } else {
                    println!("Number cannot be less than 1.");
                    None
                }
            })
            .flatten()
    });
    let c = get_desired::<Digit>(&mut rem_args, "c", |s| {
        parse_num(s)
            .map(|num| {
                Digit::try_from(num)
                    .inspect_err(|e| {
                        println!("Number '{}' is not a single digit: {:?}", num, e);
                    })
                    .ok()
            })
            .flatten()
    });

    (n, c)
}

fn main() {
    let (n, c) = get_numbers();
    let c_counts = count_contains_in_range_naive(n, c);
    println!("For digit {} appears {} times in 1..={}.", *c, c_counts, n);
}

fn count_contains_in_range_naive(n: u32, c: Digit) -> u32 {
    eprintln!("n={}, c={}", n, *c);
    (1..=n).filter(|i| warmup::contains_c(*i, c)).count() as u32
}

/// # Observation
/// n=7290; i=7200; c=2:
/// the next 90 values of `i` all contain `c`
#[allow(unused)]
fn count_contains_in_range_niche(n: u32, c: Digit) -> u32 {
    let mut count = 0;
    let mut i = 1;
    while i <= n {
        let max_e = i.ilog10(); // i >= 1
        for e in (0..=max_e).rev() {
            let place_worth = 10_u32.pow(e); // n=510; c=5 -> 100
            if i / place_worth % 10 == *c {
                let worth_remaining = u32::min(place_worth, n - i);
                count += worth_remaining;
                i += worth_remaining;
                continue;
            }
        }
        i += 1;
    }

    count
}

/// # General observations
/// `n=30`: `X3` appears 3 times, `3X` appears once.
/// `n=300`: `XX3` appears 30 times, `X3X` appears 3 times, `3XX` appears once.
/// `n=3000`: `XXX3` appears 300 times, `XX3X` appears 30 times, `X3XX` appears 3 times, `3XXX` appears once.
/// We must unfortunately subtract overlapping appearances.
///
/// # Observations of non-filled (from most significant)
/// if n=1223, and c=3, then...
/// ...for 1000..=1099: 10*(1-0) + 1*(10-1)
/// ...for 1100..=1199: 10*(1-0) + 1*(10-1)
/// ...for 1200..=1209: 1
/// ...for 1210..=1219: 1
/// ...for 1220..=1223: 1
/// ## Strategy:
/// - Digits, left to right (most significant to least)
/// - Uncounted starts as value of n's place.
/// - If the most significant digit is c, then add uncounted to count.
/// - Otherwise, treat digit as multiplier to sub_sum of digit (digit is "n" of new sub_sum?).
/// - Add to count the digits we process: n=1234 -> 1200 if digit was 2.
/// ## Example
/// ### `n=1234`, `c=4`:
/// 1234 -> 1 * sub_sum(4) +
/// 234 -> 2 * sub_sum(3) +
/// 34 -> 3 * sub_sum(2) +
/// 4 -> 4 * sub_sum(1)
#[allow(unused)]
fn count_contains_in_range_alternative(n: u32, c: Digit) -> u32 {
    eprintln!("\nCalculating for n={} and c={}.", n, *c);
    let mut remaining = n;
    let mut count = 0;
    for descending_n in ((if *c == 0 { 1 } else { 0 })..=n.ilog10()).rev() {
        let place_weight = 10_u32.pow(descending_n);
        let digit = remaining / place_weight;
        dbg!(descending_n, place_weight, digit);
        if *c == 0 {
            count += dbg!(sub_sum_zero(descending_n));
        } else {
            count += digit * sub_sum_non_zero(descending_n);
        }
        remaining -= dbg!(place_weight * digit);
        eprintln!("count={}, uncounted={}", count, remaining);
        if digit == *c {
            //            count += 1;
            break;
        }
    }
    count += remaining;

    /// # Handling non-zero digits
    /// ## Strategy
    /// The generalization for these values is that count is changed per digit's place according to:
    /// 1. How many times is this place redone? (in 0..=999, X10 happens 10 times)
    /// 2. How many numbers are in this place? (10's place = 10)
    /// 3. How much is already counted by another place? (in 0..=999, 100's overlaps 10's 10 times.)
    /// The answer to question 3 depends on 1 and 2?
    /// ## Example
    /// For *any* non-zero digit:
    /// n>=1000, 1..=999: 100*(1-0-0) + 10*(10-1-0) + 1*(100-10-1)
    /// n>=10000, 1..=9999: 1000*(1-0-0-0) + 100*(10-1-0-0) + 10*(100-10-1-0) + 1*(1000-100-10-1)

    fn sub_sum_non_zero(n_digits: u32) -> u32 {
        eprintln!("Subsum: n_digits={}", n_digits);
        let powers: Vec<_> = (0..n_digits).map(|e| 10_u32.pow(e)).collect();
        let res = powers
            .iter()
            .rev()
            .zip(powers.iter())
            .scan(Vec::new(), |prev_ascending, (&descending, &ascending)| {
                let sum = descending * ascending - prev_ascending.iter().sum::<u32>();
                prev_ascending.push(ascending);

                Some(sum)
            })
            .sum::<u32>();
        dbg!(res)
    }
    /// ## Example
    /// For zero, think the opposite way:
    /// n>=1: 0
    /// n>=10: 1
    /// n>=100: 10
    /// n>=1000: 181?
    /// n>=10000:
    fn sub_sum_zero(n_digits: u32) -> u32 {
        10_u32.pow(n_digits - 1) - (0..(n_digits - 1)).map(|e| 10_u32.pow(e)).sum::<u32>()
    }

    count
}

mod warmup {
    use super::Digit;

    #[allow(unused)]
    pub(super) fn ends_with_c(n: u32, c: Digit) -> bool {
        n % 10 == *c
    }

    #[allow(unused)]
    pub(super) fn contains_c(i: u32, c: Digit) -> bool {
        (0..=i.ilog10()).rev().any(|e| {
            let digit = i / 10_u32.pow(e);
            let a = digit % 10;
            let res = a == *c;
            if res {
                println!("TRUE on {} with digit {} & a {}", i, digit, a);
            }
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        count_contains_in_range_alternative, count_contains_in_range_naive,
        count_contains_in_range_niche, warmup::contains_c, warmup::ends_with_c, Digit,
    };

    mod warmup {
        use super::{contains_c, ends_with_c, Digit};
        #[ignore]
        #[test]
        fn warmup_1() {
            assert!(ends_with_c(19, Digit(9)));
            assert!(ends_with_c(1999, Digit(9)));
            assert!(ends_with_c(1111119, Digit(9)));
        }

        #[ignore]
        #[test]
        fn warmup_2() {
            assert!(contains_c(7264, Digit(2)));
            assert!(contains_c(72, Digit(2)));
            assert!(contains_c(264, Digit(2)));
        }
    }

    struct TestCase {
        n: u32,
        c: Digit,
        result: u32,
    }
    fn generate_cases() -> impl Iterator<Item = TestCase> {
        (2..=9).map(|d| TestCase {
            n: 100,
            c: Digit(d),
            result: 19,
        })
    }

    const BASE_CASES: &[TestCase] = &[TestCase {
        n: 100,
        c: Digit(5),
        result: 19,
    }];
    const EDGE_CASES: &[TestCase] = &[
        TestCase {
            n: 10,
            c: Digit(0),
            result: 1, // not 2: n>=1, never 0
        },
        TestCase {
            n: 100,
            c: Digit(0),
            result: 10, // "leading" zeroes are not counted, unlike other digits
        },
        TestCase {
            n: 1000,
            c: Digit(0),
            result: 181, // "leading" zeroes are not counted, unlike other digits
        },
        TestCase {
            n: 100,
            c: Digit(1),
            result: 20,
        },
    ];

    mod naive {
        use super::{count_contains_in_range_naive, generate_cases, BASE_CASES, EDGE_CASES};

        //        #[ignore]
        #[test]
        fn naive_base_case() {
            for test in BASE_CASES {
                assert_eq!(count_contains_in_range_naive(test.n, test.c), test.result);
            }
        }

        //        #[ignore]
        #[test]
        fn naive_edge_case() {
            for edge in EDGE_CASES {
                assert_eq!(count_contains_in_range_naive(edge.n, edge.c), edge.result);
            }
        }

        //        #[ignore]
        #[test]
        fn naive_generated_case() {
            for generated in generate_cases() {
                assert_eq!(
                    count_contains_in_range_naive(generated.n, generated.c),
                    generated.result
                );
            }
        }
    }

    mod niche {
        use super::{count_contains_in_range_niche, generate_cases, BASE_CASES, EDGE_CASES};

        #[ignore]
        #[test]
        fn niche_base_case() {
            for base in BASE_CASES {
                assert_eq!(count_contains_in_range_niche(base.n, base.c), base.result);
            }
        }

        #[ignore]
        #[test]
        fn niche_edge_case() {
            for edge in EDGE_CASES {
                assert_eq!(count_contains_in_range_niche(edge.n, edge.c), edge.result);
            }
        }

        #[ignore]
        #[test]
        fn niche_generated_case() {
            for edge in generate_cases() {
                assert_eq!(count_contains_in_range_niche(edge.n, edge.c), edge.result);
            }
        }
    }

    mod alternative {
        use super::{count_contains_in_range_alternative, generate_cases, BASE_CASES, EDGE_CASES};

        #[test]
        fn alternative_base_case() {
            for test in BASE_CASES {
                assert_eq!(
                    count_contains_in_range_alternative(test.n, test.c),
                    test.result
                );
            }
        }

        #[test]
        fn alternative_edge_case() {
            for edge in EDGE_CASES {
                assert_eq!(
                    count_contains_in_range_alternative(edge.n, edge.c),
                    edge.result
                );
            }
        }

        #[test]
        fn alternative_generated_case() {
            for generated in generate_cases() {
                assert_eq!(
                    count_contains_in_range_alternative(generated.n, generated.c),
                    generated.result
                );
            }
        }
    }
}
