#![allow(unused)]

use itertools::Itertools;
use std::array::TryFromSliceError;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use strum::{EnumCount, EnumIter, FromRepr, IntoEnumIterator};

impl Ord for SortedHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match HandVariants::from(self).cmp(&HandVariants::from(other)) {
            unequal @ Ordering::Less | unequal @ Ordering::Greater => unequal,
            Ordering::Equal => self.ranks().cmp(&other.ranks()),
        }
    }
}
impl PartialOrd for SortedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for SortedHand {}
impl PartialEq for SortedHand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HandVariants {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    // FlushHouse, // assume impossible
    StraightFlush,
    RoyalFlush,
    // FiveOfAKind // assume impossible
}
impl From<&SortedHand> for HandVariants {
    fn from(hand: &SortedHand) -> Self {
        match (hand.is_flush(), hand.is_straight()) {
            (true, true) => {
                if hand.ranks() == SortedHand::ROYAL {
                    HandVariants::RoyalFlush
                } else {
                    HandVariants::StraightFlush
                }
            }
            (true, false) => HandVariants::Flush, // 4-K/F-H + flush is considered impossible
            (false, true) => HandVariants::Straight,
            (false, false) => match &hand.duplicate_numbers()[..] {
                // group assumes suit invariant
                [1, 4] => HandVariants::FourOfAKind, // could be overshadowed by the "lesser" flush
                [2, 3] => HandVariants::FullHouse,   // same as above
                [1, 1, 3] => HandVariants::ThreeOfAKind,
                [1, 2, 2] => HandVariants::TwoPair,
                [1, 1, 1, 2] => HandVariants::Pair,
                [1, 1, 1, 1, 1] => HandVariants::HighCard,
                [.., (5..)] => unreachable!("4 suits, and cards should be unique"),
                [..] => unreachable!("impossible assuming hand integrity"),
            },
        }
    }
}

#[derive(Debug)]
pub struct Hand([Card; 5]);

pub struct SortedHand([Card; 5]);
impl From<Hand> for SortedHand {
    fn from(hand: Hand) -> Self {
        Self(
            hand.0
                .into_iter()
                .sorted_by_key(|card| card.rank)
                .collect::<Vec<_>>()[..]
                .try_into()
                .unwrap(),
        )
    }
}

impl SortedHand {
    const ROYAL: [Rank; 5] = [Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
    fn ranks(&self) -> [Rank; 5] {
        self.0.map(|card| card.rank)
    }
    fn is_straight(&self) -> bool {
        let ranks = self.ranks();
        debug_assert!(ranks.is_sorted());
        match (ranks.first(), ranks.last()) {
            (Some(&Rank::Two), Some(&Rank::Ace)) // low ace
              => ranks[0..4].iter(), // last two do not need to be checked together, state doesn't/can't need it
            _ => ranks.iter()
        }
        .tuple_windows()
        .all(|(a, b)| a.is_next_to(b))
    }
    fn is_flush(&self) -> bool {
        self.0.iter().map(|card| card.suit).all_equal()
    }
    fn duplicate_numbers(&self) -> Vec<u8> {
        {
            let ranks = self.ranks();
            debug_assert!(ranks.is_sorted());
            ranks
        }
        .iter()
        .group_by(|&card| card)
        .into_iter()
        .map(|(&key, group)| group.count() as u8)
        .sorted()
        .collect()
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}
impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}
impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self) // using display
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, EnumCount, FromRepr, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Rank {
    fn is_face_card(&self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }
    fn is_next_to(&self, other: &Self) -> bool {
        self != other && {
            const WRAP_AROUND_DIFF: u8 = Rank::COUNT as u8 - 1; // between Two and Ace
            let self_index = *self as u8;
            let other_index = *other as u8;
            let diff = self_index.abs_diff(other_index);
            matches!(diff, 1 | WRAP_AROUND_DIFF)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, EnumIter, EnumCount)]
pub enum Suit {
    #[default]
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Res;
    use parsing::*;

    #[test]
    fn straight_works() {
        for starting in 0..=(Rank::COUNT as u8 - 5) {
            let end = starting + 4;
            let cards = (starting..=end)
                .map(|repr| Card {
                    rank: Rank::from_repr(repr).unwrap(),
                    suit: Default::default(),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let hand = SortedHand::from(Hand(cards));
            assert!(hand.is_straight())
        }
    }

    #[test]
    fn flush_works() {
        const FLUSH_HAND: Hand = Hand(
            [Card {
                rank: Rank::Five,
                suit: Suit::Hearts,
            }; 5],
        );
        assert!(SortedHand::from(FLUSH_HAND).is_flush())
    }

    mod parsing {
        use super::*;

        pub(super) fn variant_from_str_fn(
            s: &str,
            f: fn(&str) -> Result<Hand, HandParseError>,
        ) -> HandVariants {
            HandVariants::from(&SortedHand::from(f(s).unwrap()))
        }

        #[derive(Debug)]
        pub(super) enum HandParseError {
            Slice(TryFromSliceError),
            Char(&'static str),
        }

        impl Hand {
            /// produces non-flush hands
            pub(super) fn from_str_mixed(s: &str) -> Result<Self, HandParseError> {
                Ok(Hand(
                    s.chars()
                        .zip(Suit::iter().cycle())
                        .map(|(c, suit)| c.try_into().map(|rank: Rank| Card { rank, suit }))
                        .collect::<Result<Vec<Card>, _>>()
                        .map_err(|e| HandParseError::Char(e))?[..]
                        .try_into()
                        .map_err(|e: TryFromSliceError| HandParseError::Slice(e))?,
                ))
            }
            pub(super) fn from_str_flush(s: &str) -> Result<Self, HandParseError> {
                Ok(Hand(
                    s.chars()
                        .map(|c| {
                            c.try_into().map(|rank: Rank| Card {
                                rank,
                                suit: Default::default(),
                            })
                        })
                        .collect::<Result<Vec<Card>, _>>()
                        .map_err(|e| HandParseError::Char(e))?[..]
                        .try_into()
                        .map_err(|e: TryFromSliceError| HandParseError::Slice(e))?,
                ))
            }
        }

        impl TryFrom<char> for Rank {
            type Error = &'static str;

            fn try_from(c: char) -> Result<Self, Self::Error> {
                Ok(match c {
                    '2' => Rank::Two,
                    '3' => Rank::Three,
                    '4' => Rank::Four,
                    '5' => Rank::Five,
                    '6' => Rank::Six,
                    '7' => Rank::Seven,
                    '8' => Rank::Eight,
                    '9' => Rank::Nine,
                    'T' => Rank::Ten,
                    'J' => Rank::Jack,
                    'Q' => Rank::Queen,
                    'K' => Rank::King,
                    'A' => Rank::Ace,
                    _ => Err("incomprehensible rank char")?,
                })
            }
        }
    }

    #[test]
    fn non_flush_hand_piecing_works() {
        let cases = [
            ("AKQJT", HandVariants::Straight),
            ("AAAA2", HandVariants::FourOfAKind),
            ("AAA22", HandVariants::FullHouse),
            ("AAA32", HandVariants::ThreeOfAKind),
            ("AA432", HandVariants::Pair),
            ("A5432", HandVariants::Straight),
            ("T9768", HandVariants::Straight),
            ("K5432", HandVariants::HighCard),
        ];
        for (s, result) in cases {
            assert_eq!(variant_from_str_fn(s, Hand::from_str_mixed), result);
        }
    }

    #[test]
    fn flush_hand_piecing_works() {
        let cases = [
            ("AKQJT", HandVariants::RoyalFlush),
            ("A5432", HandVariants::StraightFlush),
            ("T9768", HandVariants::StraightFlush),
            ("K5432", HandVariants::Flush),
        ];
        for (s, result) in cases {
            assert_eq!(variant_from_str_fn(s, Hand::from_str_flush), result)
        }
    }

    #[test]
    fn compare_variants() {
        let cases = [
            ("AA693", "A683T", Ordering::Greater),
            ("AAA93", "AA83T", Ordering::Greater),
            ("AA693", "AAA3T", Ordering::Less),
            ("AA693", "AA693", Ordering::Equal),
            ("AA692", "AA693", Ordering::Less),
            ("AKQTJ", "AKTQJ", Ordering::Equal),
            ("AKQTJ", "KQTJ9", Ordering::Greater),
        ];
        for (s_a, s_b, result) in cases {
            let a = variant_from_str_fn(s_a, Hand::from_str_mixed);
            let b = variant_from_str_fn(s_b, Hand::from_str_mixed);
        }
    }
}
