use bevy::prelude::*;

#[derive(Component)]
pub struct Grid([[Tile; GRID_SIZE]; GRID_SIZE]); // pure 2D tile array

const GRID_SIZE: usize = 16;

impl Grid {
	pub fn empty() -> Self { Self([[Tile::Empty; GRID_SIZE]; GRID_SIZE]) }

	#[cfg(feature = "debug")]
	pub fn console_output(&self) -> String {
		let mut buffer = format!(
			"Map ({}, {}) with (?) bombs:\n",
			GRID_SIZE, GRID_SIZE /* , self.bomb_count */
		);
		let line: String = (0..=(GRID_SIZE + 1)).into_iter().map(|_| '-').collect();
		buffer = format!("{}{}\n", buffer, line);
		for line in self.iter().rev() {
			buffer = format!("{}|", buffer);
			for tile in line.iter() {
				buffer = format!("{}{}", buffer, tile.console_output());
			}
			buffer = format!("{}|\n", buffer);
		}
		format!("{}{}", buffer, line)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
	Number(IndicatorNumber),
	Bomb,
	Empty
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndicatorNumber {
	// 1-8
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight
}
