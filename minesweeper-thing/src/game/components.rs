#![allow(unused)]

use std::{
	fmt::{self, Display, Formatter},
	ops::{Add, Deref, DerefMut, Sub}
};

use bevy::prelude::Component;
// use bevy::prelude::*;
use rand::{thread_rng, Rng};

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Coordinates {
// 	pub x: usize,
// 	pub y: usize
// }

// impl Add for Coordinates {
// 	type Output = Self;

// 	fn add(self, rhs: Self) -> Self::Output {
// 		Self {
// 			x: self.x + rhs.x,
// 			y: self.y + rhs.y
// 		}
// 	}
// }
// impl Sub for Coordinates {
// 	type Output = Self;

// 	fn sub(self, rhs: Self) -> Self::Output {
// 		Self {
// 			x: self.x.saturating_sub(rhs.x),
// 			y: self.y.saturating_sub(rhs.y)
// 		}
// 	}
// }
// impl Display for Coordinates {
// 	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "({}, {})",
// self.x, self.y) } }
// impl Add<(i8, i8)> for Coordinates {
// 	type Output = Self;

// 	#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
// 	fn add(self, (x, y): (i8, i8)) -> Self::Output {
// 		let x = ((self.x as isize) + x as isize) as usize;
// 		let y = ((self.y as isize) + y as isize) as usize;
// 		Self { x, y }
// 	}
// }

// const SQUARE_COORDS: [(i8, i8); 8] = [
// 	// Bottom left
// 	(-1, -1),
// 	// Bottom
// 	(0, -1),
// 	// Bottom right
// 	(1, -1),
// 	// Left
// 	(-1, 0),
// 	// Right
// 	(1, 0),
// 	// Top Left
// 	(-1, 1),
// 	// Top
// 	(0, 1),
// 	// Top right
// 	(1, 1)
// ];

pub const GRID_SIZE: usize = 16;
// #[derive(Debug, Clone)]
// pub struct TileMap {
// 	// pure 2D tile array
// 	bomb_count: usize,
// 	grid:       Grid
// }
pub type Grid = [[Tile; GRID_SIZE]; GRID_SIZE];

// impl Deref for TileMap {
// 	type Target = Grid;

// 	fn deref(&self) -> &Self::Target { &self.grid }
// }

// impl DerefMut for TileMap {
// 	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.grid }
// }

// impl TileMap {
// 	pub fn empty() -> Self {
// 		Self {
// 			bomb_count: 0,
// 			grid:       [[Tile::Empty; GRID_SIZE]; GRID_SIZE]
// 		}
// 	}

// 	pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item
// = Coordinates> { 		SQUARE_COORDS
// 			.iter()
// 			.copied()
// 			.map(move |tuple| coordinates + tuple)
// 	}

// 	pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
// 		if coordinates.x >= GRID_SIZE || coordinates.y >= GRID_SIZE {
// 			return false;
// 		};
// 		self.grid[coordinates.y][coordinates.x].is_bomb()
// 	}

// 	pub fn bomb_count_at(&self, coordinates: Coordinates) -> usize {
// 		if self.is_bomb_at(coordinates) {
// 			return 0;
// 		}
// 		self.safe_square_at(coordinates)
// 			.filter(|coord| self.is_bomb_at(*coord))
// 			.count()
// 	}

// 	pub fn set_bombs(&mut self, bomb_count: usize) {
// 		self.bomb_count = bomb_count;
// 		let mut remaining_bombs = bomb_count;
// 		let mut rng = thread_rng();

// 		while remaining_bombs > 0 {
// 			let (x, y) = (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
// 			if let Tile::Empty = self.grid[y][x] {
// 				self.grid[y][x] = Tile::Bomb;
// 				remaining_bombs -= 1;
// 			}
// 		}
// 		for y in 0..GRID_SIZE {
// 			for x in 0..GRID_SIZE {
// 				let coords = Coordinates { x, y };
// 				if self.is_bomb_at(coords) {
// 					continue;
// 				}
// 				let num = self.bomb_count_at(coords);
// 				if num == 0 {
// 					continue;
// 				}
// 				let tile = &mut self[y][x];
// 				*tile = Tile::BombNeighbor(
// 					IndicatorNumber::try_from(num).expect("Num must be a number 1-8")
// 				);
// 			}
// 		}
// 	}

// 	#[cfg(feature = "debug")]
// 	pub fn console_output(&self) -> String {
// 		let mut buffer = format!(
// 			"Map ({}, {}) with (?) bombs:\n",
// 			GRID_SIZE, GRID_SIZE /* , self.bomb_count */
// 		);
// 		let line: String = (0..=(GRID_SIZE + 1)).map(|_| '-').collect();
// 		buffer = format!("{}{}\n", buffer, line);
// 		for line in self.iter().rev() {
// 			buffer = format!("{}|", buffer);
// 			for tile in line.iter() {
// 				buffer = format!("{}{}", buffer, tile.console_output());
// 			}
// 			buffer = format!("{}|\n", buffer);
// 		}
// 		format!("{}{}", buffer, line)
// 	}
// }

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
	BombNeighbor(IndicatorNumber),
	Bomb,
	Empty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorNumber {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight
}
impl TryFrom<usize> for IndicatorNumber {
	type Error = String;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		Ok(match value {
			1 => IndicatorNumber::One,
			2 => IndicatorNumber::Two,
			3 => IndicatorNumber::Three,
			4 => IndicatorNumber::Four,
			5 => IndicatorNumber::Five,
			6 => IndicatorNumber::Six,
			7 => IndicatorNumber::Seven,
			8 => IndicatorNumber::Eight,
			_ => Err("Bomb count provided exceeded maximum accounted for".to_string())?
		})
	}
}
impl From<IndicatorNumber> for usize {
	fn from(val: IndicatorNumber) -> Self {
		match val {
			IndicatorNumber::One => 1,
			IndicatorNumber::Two => 2,
			IndicatorNumber::Three => 3,
			IndicatorNumber::Four => 4,
			IndicatorNumber::Five => 5,
			IndicatorNumber::Six => 6,
			IndicatorNumber::Seven => 7,
			IndicatorNumber::Eight => 8
		}
	}
}

// impl Tile {
// 	/// Is the tile a bomb?
// 	pub const fn is_bomb(&self) -> bool { matches!(self, Self::Bomb) }

// 	#[cfg(feature = "debug")]
// 	pub fn console_output(&self) -> String {
// 		(match self {
// 			Tile::Bomb => "*".to_string(),
// 			Tile::BombNeighbor(v) => {
// 				let num: usize = (*v).into();
// 				num.to_string()
// 			},
// 			Tile::Empty => " ".to_string()
// 		})
// 		.to_string()
// 	}
// }
