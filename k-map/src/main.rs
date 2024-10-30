extern crate core;

use itertools::Itertools;
use std::{
    env::args,
    fmt::{Display, Formatter},
    num::NonZero,
    str::FromStr,
};

#[derive(Debug)]
enum Field {
    One,
    Zero,
    X,
}
impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Field::One => '1',
                Field::Zero => '0',
                Field::X => 'X',
            }
        )
    }
}
struct Map<'a> {
    grid: &'a [Field; 2usize.pow(4)],
    wrap_point: NonZero<u8>,
    symbols: Vec<char>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Bit {
    One,
    Zero,
}
impl Display for Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Bit::One => '1',
                Bit::Zero => '0',
            }
        )
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
struct GrayCode(Vec<Bit>);
impl Display for GrayCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().join(""))
    }
}

#[test]
fn graycode_works() {
    for bit_count in 1..=2u8.pow(3) {
        let graycodes = graycode_recursive(bit_count.try_into().unwrap());
        assert_eq!(
            graycodes.len(),
            2usize.pow(bit_count as u32),
            "the amount of gray codes is 2^n"
        );
        assert!(
            graycodes.iter().all_unique(),
            "every generated graycode is unique"
        );
        for graycode in graycodes {
            assert_eq!(
                graycode.0.len() as u8,
                bit_count,
                "graycode {} is length {}",
                graycode,
                bit_count
            );
        }
    }
}

/// Given a length `n`, returns a vec of unique graycodes ordered such that every next value
/// is only one bit different from the previous, wrap-around included.
fn graycode_recursive(n: NonZero<u8>) -> Vec<GrayCode> {
    match n.get() {
        0 => unreachable!("n cannot be zero"),
        1 => vec![GrayCode(vec![Bit::Zero]), GrayCode(vec![Bit::One])],
        _ => {
            let lower = graycode_recursive(NonZero::new(n.get() - 1).unwrap());
            let zero_prefixed = lower
                .iter()
                .cloned()
                .map(|code| GrayCode([vec![Bit::Zero], code.0].concat()))
                .collect::<Vec<_>>();
            let one_prefixed = lower
                .into_iter()
                .map(|code| GrayCode([vec![Bit::One], code.0].concat()))
                .collect::<Vec<_>>();

            [zero_prefixed, one_prefixed].concat()
        }
    }
}

impl<'a> Display for Map<'a> {
    #[allow(clippy::erasing_op, clippy::write_literal, clippy::identity_op)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let midpoint = self.symbols.len() / 2;
        let down_symbols = &self.symbols[..midpoint]; // does *not* include middle
        let right_symbols = &self.symbols[midpoint..]; // does include middle

        // write top bar: `| AB\CD | 00 | 01 | 11 | 10 |`
        write!(
            f,
            "| {}\\{} |",
            down_symbols.iter().join(""),
            right_symbols.iter().join("")
        )?;
        let right = graycode_recursive((right_symbols.len() as u8).try_into().unwrap());
        for code in right {
            write!(f, " {} |", code)?;
        }
        writeln!(f)?;

        let space = right_symbols.len() + 1; // equivalent to `width - down_symbols.len()`

        // write rest: `|    00 |  0 |  1 |  0 |  X |`
        let down = graycode_recursive((down_symbols.len() as u8).try_into().unwrap());
        for (i, code) in down.into_iter().enumerate() {
            write!(f, "| {}{} |", " ".repeat(space), code)?;
            let wrap_point = self.wrap_point.get() as usize;
            for field in &self.grid[(wrap_point * i)..(wrap_point * i + wrap_point)] {
                write!(f, " {}{} |", " ".repeat(right_symbols.len() - 1), field)?;
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

/// Tree of terms
#[derive(Debug)]
enum TermExpr {
    Symbol(char),
    Primed(Box<TermExpr>),
    Parentheses(Box<TermExpr>),
    Or(Box<TermExpr>, Box<TermExpr>),
    And(Box<TermExpr>, Box<TermExpr>),
}
impl Display for TermExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TermExpr::Symbol(c) => format!("{}", c),
                TermExpr::Primed(t) => format!("{}'", t),
                TermExpr::Parentheses(t) => format!("({})", t),
                TermExpr::Or(a, b) => format!("{}+{}", a, b),
                TermExpr::And(a, b) => format!("{}{}", a, b),
            }
        )
    }
}
#[derive(Debug)]
enum ExprParseErr {
    IllegalCharacter(char),
    MissingExprBeforeOr,
    DoublePlus,
    EarlyParenthesisClose,
    UnmatchedOpenParenthesis(u64),
}
impl FromStr for TermExpr {
    type Err = ExprParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
	    /// This exists to solve the problem of "what should the parenthesis surround?"
	    /// The problem with this is that exiting from this function naturally is hard.
        fn parse_sub_exprs(mut iter: impl Iterator<Item = char>) -> Result<Self, Self::Err> {
            let mut prev_expr: Option<TermExpr> = None;
            let mut unbonded_plus = false;
            for c in iter.take_while_inclusive(|&c| c != 'c') {
                match c {
                    '+' => match prev_expr {
                        None => Err(ExprParseErr::MissingExprBeforeOr)?,
                        Some(_) => {
                            if unbonded_plus {
                                Err(ExprParseErr::DoublePlus)?
                            }
                            unbonded_plus = true;
                        }
                    },
                    '(' => {
                        todo!("parse subexpression")
                    }
                    ')' => match prev_expr.take() {
                        None => Err(ExprParseErr::EarlyParenthesisClose)?,
                        Some(_) if unbonded_plus => Err(ExprParseErr::EarlyParenthesisClose)?,
                        Some(p) => return Ok(p),
                    },
                    '\'' => todo!(),
                    c if c.is_alphabetic() => match prev_expr.take() {
                        None => prev_expr = Some(TermExpr::Symbol(c)),
                        Some(a) => {
                            prev_expr =
                                Some(TermExpr::And(Box::new(a), Box::new(TermExpr::Symbol(c))))
                        }
                    },
                    c => Err(ExprParseErr::IllegalCharacter(c))?,
                }
            }
            todo!()
        }

        if parenthesis > 0 {
            Err(ExprParseErr::UnmatchedOpenParenthesis(parenthesis))?
        }
        todo!()
    }
}

fn main() {
    let arg = args().nth(1).unwrap();
    let expression = arg.parse::<TermExpr>();
    println!("Exp:\n\n{:?}", expression)
}
