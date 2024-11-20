use std::{
	fmt::{Display, Formatter},
	num::NonZero
};

use itertools::Itertools;

use crate::expressions::ExprNode;

#[derive(Debug)]
pub enum Field {
	One,
	Zero,
	X
}
impl Display for Field {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Field::One => '1',
			Field::Zero => '0',
			Field::X => 'X'
		})
	}
}
pub struct Map<'a> {
	grid:       &'a [Field; 2usize.pow(4)],
	wrap_point: NonZero<u8>,
	symbols:    Vec<char>
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Bit {
	One,
	Zero
}
impl Display for Bit {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Bit::One => '1',
			Bit::Zero => '0'
		})
	}
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GrayCode(Vec<Bit>);
impl Display for GrayCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.iter().join(""))
	}
}

/// Given a length `n`, returns a vec of unique graycodes ordered such that
/// every next value is only one bit different from the previous, wrap-around
/// included.
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

impl<'a> Display for Map<'a> {
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

impl<'a> From<ExprNode> for Map<'a> {
	fn from(value: ExprNode) -> Self {
		todo!()
	}
}
