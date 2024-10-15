use std::{
	env::args,
	fmt::{Display, Formatter},
	str::FromStr
};

use itertools::Itertools;

#[derive(Debug)]
enum MapField {
	One,
	Zero,
	X
}
impl Display for MapField {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			MapField::One => '1',
			MapField::Zero => '0',
			MapField::X => 'X'
		})
	}
}
struct Map4([[MapField; 4]; 4]);

impl Display for Map4 {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "| AB\\CD | 00 | 01 | 11 | 10 |")?;
		write!(
			f,
			"|    00 | {} | {} | {} | {} |",
			self.0[0][0], self.0[0][1], self.0[0][2], self.0[0][3]
		)?;
		write!(
			f,
			"|    01 | {} | {} | {} | {} |",
			self.0[1][0], self.0[1][1], self.0[1][2], self.0[1][3]
		)?;
		write!(
			f,
			"|    11 | {} | {} | {} | {} |",
			self.0[2][0], self.0[2][1], self.0[2][2], self.0[2][3]
		)?;
		write!(
			f,
			"|    10 | {} | {} | {} | {} |",
			self.0[3][0], self.0[3][1], self.0[3][2], self.0[3][3]
		)?;
		Ok(())
	}
}

enum TermOperation {
	And,
	Or
}

/// Tree of terms
enum TermExpression {
	Leaf {
		label: char,
		prime: bool
	},
	Subterms {
		elements:  Vec<TermExpression>,
		operation: TermOperation
	}
}
enum TermParseErr {
	UnknownSymbol
}
impl FromStr for TermExpression {
	type Err = TermParseErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut iter = s.split('+').peekable();
		Ok(if iter.peek().is_some() {
			// iterator is OR-splittable
			Self::Subterms {
				elements:  iter
					.map(|sub_expr| sub_expr.parse::<TermExpression>())
					.collect::<Result<Vec<_>, _>>()?,
				operation: TermOperation::Or
			}
		} else {
			// reached AND(?)
			let mut chars = s.chars();
			let res = chars.peeking_take_while(|a| *a == '\'');
			todo!()
		})
	}
}

fn main() {
	let arg = args().nth(1).unwrap();
	let (name, expression) = arg.split_once("=").unwrap();
	let (_name, _expression) = (name.trim(), expression.trim());
	println!("Hello, world!");
}
