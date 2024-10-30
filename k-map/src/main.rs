use itertools::Itertools;
use std::num::NonZero;
use std::{
    env::args,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug)]
enum Field {
    One,
    Zero,
    DontCare,
}
impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Field::One => '1',
                Field::Zero => '0',
                Field::DontCare => 'X',
            }
        )
    }
}
struct Map4 {
    horizontal_slice: [Field; 2usize.pow(4)],
}

impl Display for Map4 {
    #[allow(clippy::erasing_op, clippy::identity_op)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "| AB\\CD | {:2b} | {:2b} | {:2b} | {:2b} |",
            0b00, 0b10, 0b10, 0b10
        )?;
        writeln!(
            f,
            "|    {:2b} |  {} |   {} |   {} |  {} |",
            0b00,
            self.horizontal_slice[4 * 0 + 0],
            self.horizontal_slice[4 * 0 + 1],
            self.horizontal_slice[4 * 0 + 2],
            self.horizontal_slice[4 * 0 + 3]
        )?;
        writeln!(
            f,
            "|    {:2b} |  {} |  {} |  {} |  {} |",
            0b01,
            self.horizontal_slice[4 * 1 + 0],
            self.horizontal_slice[4 * 1 + 1],
            self.horizontal_slice[4 * 1 + 2],
            self.horizontal_slice[4 * 1 + 3]
        )?;
        writeln!(
            f,
            "|    {:2b} |  {} |  {} |  {} |  {} |",
            0b11,
            self.horizontal_slice[4 * 2 + 0],
            self.horizontal_slice[4 * 2 + 1],
            self.horizontal_slice[4 * 2 + 2],
            self.horizontal_slice[4 * 2 + 3]
        )?;
        writeln!(
            f,
            "|    {:2b} |  {} |  {} |  {} |  {} |",
            0b10,
            self.horizontal_slice[4 * 3 + 0],
            self.horizontal_slice[4 * 3 + 1],
            self.horizontal_slice[4 * 3 + 2],
            self.horizontal_slice[4 * 3 + 3]
        )?;
        Ok(())
    }
}

enum TermOperation {
    And,
    Or,
}

/// Tree of terms
enum TermExpression {
    Symbol(char),
    Primed(Box<TermExpression>),
    Parentheses(Box<TermExpression>),
    Or(Vec<TermExpression>),
    AND(Vec<TermExpression>),
}
#[derive(Debug)]
enum TermParseErr {
    UnknownSymbol,
}
impl FromStr for TermExpression {
    type Err = TermParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('+').peekable();
        todo!()
    }
}

fn main() {
    let arg = args().nth(1).unwrap();
    let (name, expression) = arg.split_once("=").unwrap();
    let (_name, _expression) = (name.trim(), expression.trim());
    println!("Hello, world!");
}
