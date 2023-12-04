use std::ops::{Deref, DerefMut};

use rand::{thread_rng, Rng};

use crate::{components::Coordinates, resources::tile::Tile};

/// Base tile map
#[derive(Debug, Clone)]
pub struct TileMap {
	bomb_count: u16,
	height:     u16,
	width:      u16,
	map:        Vec<Vec<Tile>>
}

impl TileMap {
	/// Generates an empty map
	pub fn empty(width: u16, height: u16) -> Self {
		let map = (0..height)
			.map(|_| (0..width).map(|_| Tile::Empty).collect())
			.collect();
		Self {
			bomb_count: 0,
			height,
			width,
			map
		}
	}

	#[cfg(feature = "debug")]
	pub fn console_output(&self) -> String {
		let mut buffer = format!(
			"Map ({}, {}) with {} bombs:\n",
			self.width, self.height, self.bomb_count
		);
		let line: String = (0..=(self.width + 1)).map(|_| '-').collect();
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

	// Getter for `width`
	pub fn width(&self) -> u16 { self.width }

	// Getter for `height`
	pub fn height(&self) -> u16 { self.height }

	// Getter for `bomb_count`
	pub fn bomb_count(&self) -> u16 { self.bomb_count }

	pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
		SQUARE_COORDINATES
			.iter()
			.copied()
			.map(move |tuple| coordinates + tuple)
	}

	pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
		if coordinates.x >= self.width || coordinates.y >= self.height {
			return false;
		};
		self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
	}

	#[allow(clippy::cast_possible_truncation)]
	pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
		if self.is_bomb_at(coordinates) {
			return 0;
		}
		self.safe_square_at(coordinates)
			.filter(|coord| self.is_bomb_at(*coord))
			.count() as u8
	}

	/// Places bombs and bomb neighbor tiles
	pub fn set_bombs(&mut self, bomb_count: u16) {
		self.bomb_count = bomb_count;
		let mut remaining_bombs = bomb_count;
		let mut rng = thread_rng();
		// Place bombs
		while remaining_bombs > 0 {
			let (x, y) = (
				rng.gen_range(0..self.width) as usize,
				rng.gen_range(0..self.height) as usize
			);
			if let Tile::Empty = self[y][x] {
				self[y][x] = Tile::Bomb;
				remaining_bombs -= 1;
			}
		}
		// Place bomb neighbors
		for y in 0..self.height {
			for x in 0..self.width {
				let coords = Coordinates { x, y };
				if self.is_bomb_at(coords) {
					continue;
				}
				let num = self.bomb_count_at(coords);
				if num == 0 {
					continue;
				}
				let tile = &mut self[y as usize][x as usize];
				*tile = Tile::BombNeighbor(num);
			}
		}
	}
}

impl Deref for TileMap {
	type Target = Vec<Vec<Tile>>;

	fn deref(&self) -> &Self::Target { &self.map }
}

impl DerefMut for TileMap {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.map }
}

/// Delta coordinates for all 8 square neighbors
const SQUARE_COORDINATES: [(i8, i8); 8] = [
	// Bottom left
	(-1, -1),
	// Bottom
	(0, -1),
	// Bottom right
	(1, -1),
	// Left
	(-1, 0),
	// Right
	(1, 0),
	// Top Left
	(-1, 1),
	// Top
	(0, 1),
	// Top right
	(1, 1)
];
