#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]

pub mod components;
pub mod resources;

use bevy::{log, prelude::*, window::PrimaryWindow};
use resources::{tile_map::TileMap, BoardOptions};

use crate::{
	components::Coordinates,
	resources::{BoardPosition, TileSize}
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, create_board);
		log::info!("Loaded Board Plugin");
	}
}

// impl BoardPlugin {
// 	/// System to generate the complete board
// 	pub fn create_board() {
// 		let mut tile_map = TileMap::empty(20, 20);
// 		tile_map.set_bombs(40);
// 		#[cfg(feature = "debug")]
// 		log::info!("{}", tile_map.console_output());
// 	}
// }

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn create_board(
	mut commands: Commands,
	board_options: Option<Res<BoardOptions>>,
	window_query: Query<&Window, With<PrimaryWindow>>
) {
	let window = window_query.get_single().expect("No primary window?");

	let options = match board_options {
		None => BoardOptions::default(), // If no options is set we use the default one
		Some(o) => o.clone()
	};

	// Tilemap generation
	let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
	tile_map.set_bombs(options.bomb_count);

	// Tilemap debugging
	#[cfg(feature = "debug")]
	log::info!("{}", tile_map.console_output());

	let tile_size = match options.tile_size {
		TileSize::Fixed(v) => v,
		TileSize::Adaptive { min, max } => {
			adaptative_tile_size(window, (min, max), (tile_map.width(), tile_map.height()))
		},
	};

	// We deduce the size of the complete board
	let board_size = Vec2::new(
		f32::from(tile_map.width()) * tile_size,
		f32::from(tile_map.height()) * tile_size
	);
	log::info!("board size: {}", board_size);

	// We define the board anchor position (bottom left)
	let board_position = match options.position {
		BoardPosition::Centered { offset } => {
			Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
		},
		BoardPosition::Custom(p) => p
	};

	commands
		.spawn((
			Name::new("Board"),
			Transform::from_translation(board_position),
			GlobalTransform::default()
		))
		.with_children(|parent| {
			// We spawn the board background sprite at the center of the board, since the
			// sprite pivot is centered
			parent.spawn((
				SpriteBundle {
					sprite: Sprite {
						color: Color::WHITE,
						custom_size: Some(board_size),
						..Default::default()
					},
					transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
					..Default::default()
				},
				Name::new("Background")
			));

			// Tiles
			for (y, line) in tile_map.iter().enumerate() {
				for (x, _tile) in line.iter().enumerate() {
					parent.spawn((
						SpriteBundle {
							sprite: Sprite {
								color: Color::GRAY,
								custom_size: Some(Vec2::splat(tile_size - options.tile_padding)),
								..Default::default()
							},
							transform: Transform::from_xyz(
								(x as f32 * tile_size) + (tile_size / 2.0),
								(y as f32 * tile_size) + (tile_size / 2.0),
								1.0
							),
							..Default::default()
						},
						Name::new(format!("Tile ({}, {})", x, y)),
						// We add the `Coordinates` component to our tile entity
						Coordinates {
							x: x as u16,
							y: y as u16
						}
					));
				}
			}
		});
}

/// Computes a tile size that matches the window according to the tile map size
fn adaptative_tile_size(
	window: &Window,
	(min, max): (f32, f32),      // Tile size constraints
	(width, height): (u16, u16)  // Tile map dimensions
) -> f32 {
	let max_width = window.width() / f32::from(width);
	let max_heigth = window.height() / f32::from(height);
	max_width.min(max_heigth).clamp(min, max)
}
