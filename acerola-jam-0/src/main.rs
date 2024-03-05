use bevy::prelude::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use strum::{EnumIter, IntoEnumIterator};

/// WORKING TITLE: Pokern't ("not poker")
///
/// Structure/genre is a casual one-game / rogue-like system. You play against computer opponent(s).
///
/// Texas hold 'em, but every round the rules change (inspired by drinking game Buffalo).
/// Practically/could be endless, but is balanced around a certain round limit as the ending.
/// To play effectively, you have to keep many arbitrary rules in your head at the same time.
/// To keep things managable, a default game mode could show all the rules once per round (start?).
///
/// Standard poker rules means the game is kind of just like poker.
/// I want to make some pretty simple rules to start. System can be rewritten if need be.
///
/// Simplest rule modifications:
/// Cards, hands or suits can be suddenly "voided" (not worth anything), or offset in value
///
/// Rules are set in play either by random or by player pick (think Hades' Pact of Punishment).
///
/// Many of these modifications to the rules may be bad. Testing and system sanity will tell me.
///
/// An important detail is that the final hand comparison should not be instantly decided,
/// but rather slowly revealed to let the players have a chance at guessing if they won or lost.
fn main() {
    // test
    App::new().add_plugins(DefaultPlugins).run()
}

type ScoringOffset = i8; // +-128

#[derive(Debug)]
struct AddedRules {
    hand_modifiers: HashMap<Hand, Modifier>,
    rank_modifiers: HashMap<Rank, Modifier>,
    suit_modifiers: HashMap<Suit, Modifier>,
}

#[derive(Debug, Copy, Clone)]
enum Modifier {
    /// "Voiding" could mean that the scoring here is nullified (not counted) somehow.
    // Voided,
    /// Value is modified by some amount.
    Offset(ScoringOffset), // hands
}
impl Into<ScoringOffset> for Modifier {
    fn into(self) -> ScoringOffset {
        match self {
            Modifier::Offset(offset) => offset,
        }
    }
}

#[derive(Debug)]
struct DrawnCards(Vec<Card>);

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>, // 52 cards at start, shared between players
}

type DealtHand = [Card; 2];
// type ShowdownHand = [Card; 5];

#[repr(u8)] // use as value, offsetable
#[derive(Copy, Clone, Debug, EnumIter, Eq, PartialEq, Hash)]
enum Hand {
    // normal poker hand types //
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    // cover the possibility of a fifth identical //
    FiveOfAKind,
    FiveOfAKindFlush,
}

// type Kicker = [Card];

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Card {
    rank: Rank,
    suit: Suit,
}
impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit) // Ace of Clubs
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug, EnumIter)]
enum Rank {
    // 2..=14
    Two = 2,
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
    Ace = 14,
}

#[derive(Copy, Clone, Debug, EnumIter, Eq, PartialEq, Hash)]
enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

fn compare_showdown(
    added_rules: AddedRules,
    drawn: DrawnCards,
    a: DealtHand,
    b: DealtHand,
) -> Ordering {
    // create ordered list of best hands
    let ordered_hands: Vec<_> = Hand::iter()
        .sorted_by(|a, b| {
            let offset_a = *a as i16 + added_rules
                .hand_modifiers
                .get(a)
                .map(|modifier| (*modifier).into())
                .unwrap_or(0) as i16;
            let offset_b = *b as i16 + added_rules
                .hand_modifiers
                .get(b)
                .map(|modifier| (*modifier).into())
                .unwrap_or(0) as i16;
            offset_a.cmp(&offset_b)
        })
        .collect();

    // create ordered list of best ranks
    let ordered_ranks: Vec<_> = Rank::iter()
        .sorted_by(|a, b| {
            let offset_a = *a as i16 + added_rules
                .rank_modifiers
                .get(&a)
                .map(|modifier| (*modifier).into())
                .unwrap_or(0) as i16;
            let offset_b = *b as i16 + added_rules
                .rank_modifiers
                .get(&b)
                .map(|modifier| (*modifier).into())
                .unwrap_or(0) as i16;
            offset_a.cmp(&offset_b)
        })
        .collect();
    
    // create ordered list of best suits (if possible)
    let ordered_suits = if added_rules.suit_modifiers.is_empty() {
        None
    } else {
        Some(Suit::iter().filter_map(|suit| {
            added_rules
                .suit_modifiers
                .get(&suit)
                .map(|&modifier| (suit, modifier.into()))
        })
            .sorted_by_key(|(_, level): &(_, ScoringOffset)| *level)
            .collect::<Vec<_>>())
    };
    
    for best_hand_candidate in ordered_hands {
        fn try_create_hand(target: Hand, dealt: DealtHand) -> (bool, [Card]) {
            match target {
                Hand::HighCard => (true, dealt.into_iter().sorted_by()),
                Hand::Pair => {}
                Hand::TwoPair => {}
                Hand::ThreeOfAKind => {}
                Hand::Straight => {}
                Hand::Flush => {}
                Hand::FullHouse => {}
                Hand::FourOfAKind => {}
                Hand::StraightFlush => {}
                Hand::RoyalFlush => {}
                Hand::FiveOfAKind => {}
                Hand::FiveOfAKindFlush => {}
            }
        }
        let (a_created, a_kicker) = try_create_hand(best_hand_candidate, a);
        if a_created {
            
        }
    }

    todo!()
}

struct ModifiedRank {
    internal: Rank,
}