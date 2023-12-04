#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]

pub mod components;
pub mod resources;

mod bounds;
mod events;
mod systems;

use bevy::{log, math::Vec3Swizzles, prelude::*, utils::HashMap, window::PrimaryWindow};
use bounds::Bounds2;
use resources::{board::Board, tile::Tile, tile_map::TileMap, BoardOptions};

use self::{
	components::{Bomb, BombNeighbor, Coordinates, Uncover},
	resources::{BoardPosition, TileSize}
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, Self::create_board)
			.add_systems(PreUpdate, systems::input::input_handling);
		log::info!("Loaded Board Plugin");

		#[cfg(feature = "debug")]
		{
			// registering custom component to be able to edit it in inspector
			app.register_type::<Coordinates>();
			app.register_type::<BombNeighbor>();
			app.register_type::<Bomb>();
			app.register_type::<Uncover>();
		}
	}
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
impl BoardPlugin {
	pub fn create_board(
		mut commands: Commands,
		board_options: Option<Res<BoardOptions>>,
		window_query: Query<&Window, With<PrimaryWindow>>,
		asset_server: Res<AssetServer>
	) {
		let window = window_query.get_single().expect("No primary window?");

		let options = match board_options {
			None => BoardOptions::default(), // If no options is set we use the default one
			Some(o) => o.clone()
		};

		let bomb_image = asset_server.load("sprites/bomb.png");

		// Tilemap generation
		let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
		tile_map.set_bombs(options.bomb_count);

		let mut covered_tiles =
			HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

		// Tilemap debugging
		#[cfg(feature = "debug")]
		log::info!("{}", tile_map.console_output());

		let tile_size = match options.tile_size {
			TileSize::Fixed(v) => v,
			TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
				window,
				(min, max),
				(tile_map.width(), tile_map.height())
			)
		};

		#[cfg(feature = "debug")]
		log::info!("{}", tile_size);

		// We deduce the size of the complete board
		let board_size = Vec2::new(
			f32::from(tile_map.width()) * tile_size,
			f32::from(tile_map.height()) * tile_size
		);
		log::info!("board size: {}", board_size);

		// We define the board anchor position (bottom left)
		let board_position = match options.position {
			BoardPosition::Centered { offset } => {
				Vec3::new(board_size.x / 2.0, board_size.y / 2.0, 0.0) + offset
			},
			BoardPosition::Custom(p) => p
		};

		#[cfg(feature = "debug")]
		log::info!("board position: {}", board_position);

		commands
			.spawn((
				Name::new("Board"),
				Transform::default(),
				GlobalTransform::default(),
				Visibility::default(),
				ComputedVisibility::default()
			))
			.with_children(|parent| {
				// We spawn the board background sprite at the center of the board, since the
				// sprite pivot is centered
				parent.spawn((
					SpriteBundle {
						sprite: Sprite {
							color: Color::WHITE,
							custom_size: Some(board_size),
							..default()
						},
						transform: Transform::from_xyz(board_size.x / 2.0, board_size.y / 2.0, 0.0),
						..default()
					},
					Name::new("Background")
				));

				Self::spawn_tiles(
					parent,
					&tile_map,
					tile_size,
					options.tile_padding,
					Color::GRAY,
					bomb_image,
					Color::DARK_GRAY,
					&mut covered_tiles
				);
			});
		commands.insert_resource(Board {
			tile_map,
			bounds: Bounds2 {
				position: board_position.xy(),
				size:     board_size
			},
			tile_size,
			covered_tiles
		});
	}

	#[allow(clippy::too_many_arguments)]
	fn spawn_tiles(
		parent: &mut ChildBuilder,
		tile_map: &TileMap,
		size: f32,
		padding: f32,
		_color: Color,
		bomb_image: Handle<Image>,
		covered_tile_color: Color,
		covered_tiles: &mut HashMap<Coordinates, Entity>
	) {
		// Tiles
		for (y, line) in tile_map.iter().enumerate() {
			for (x, tile) in line.iter().enumerate() {
				let coordinates = Coordinates {
					x: x as u16,
					y: y as u16
				};
				let mut cmd = parent.spawn((
					SpriteBundle {
						sprite: Sprite {
							color: Color::GRAY,
							custom_size: Some(Vec2::splat(size - padding)),
							..default()
						},
						transform: Transform::from_xyz(
							(x as f32 * size) + (size / 2.0),
							(y as f32 * size) + (size / 2.0),
							1.0
						),
						..default()
					},
					Name::new(format!("Tile ({}, {})", x, y)),
					// We add the `Coordinates` component to our tile entity
					coordinates
				));
				// We add the cover sprites
				cmd.with_children(|parent| {
					let entity = parent
						.spawn(SpriteBundle {
							sprite: Sprite {
								custom_size: Some(Vec2::splat(size - padding)),
								color: covered_tile_color,
								..Default::default()
							},
							transform: Transform::from_xyz(0., 0., 2.),
							..Default::default()
						})
						.insert(Name::new("Tile Cover"))
						.id();
					covered_tiles.insert(coordinates, entity);
				});
				match tile {
					// If the tile is a bomb we add the matching component and a sprite child
					Tile::Bomb => {
						cmd.insert(Bomb);
						cmd.with_children(|parent| {
							parent.spawn(SpriteBundle {
								sprite: Sprite {
									custom_size: Some(Vec2::splat(size - padding)),
									..default()
								},
								transform: Transform::from_xyz(0.0, 0.0, 1.0),
								texture: bomb_image.clone(),
								..default()
							});
						});
					},
					// If the tile is a bomb neighbour we add the matching component and a text
					// child
					Tile::BombNeighbor(v) => {
						cmd.insert(BombNeighbor { count: *v });
						cmd.with_children(|parent| {
							parent.spawn(Self::bomb_count_text_bundle(*v, size - padding));
						});
					},
					Tile::Empty => {}
				}
			}
		}
	}

	/// Computes a tile size that matches the window according to the tile map
	/// size
	fn adaptative_tile_size(
		window: &Window,
		(min, max): (f32, f32),      // Tile size constraints
		(width, height): (u16, u16)  // Tile map dimensions
	) -> f32 {
		let max_width = window.width() / f32::from(width);
		let max_heigth = window.height() / f32::from(height);
		max_width.min(max_heigth).clamp(min, max)
	}

	/// Generates the bomb counter text 2D Bundle for a given value
	fn bomb_count_text_bundle(count: u8, /* font: Handle<Font>, */ size: f32) -> Text2dBundle {
		// We retrieve the text and the correct color
		let (text, color) = (count.to_string(), match count {
			1 => Color::WHITE,
			2 => Color::GREEN,
			3 => Color::YELLOW,
			4 => Color::ORANGE,
			_ => Color::PURPLE
		});
		// We generate a text bundle
		Text2dBundle {
			text: Text {
				sections: vec![TextSection {
					value: text,
					style: TextStyle {
						color,
						font_size: size,
						..default()
					}
				}],
				alignment: TextAlignment::Center,
				..default()
			},
			transform: Transform::from_xyz(0.0, 0.0, 1.0),
			..default()
		}
	}
}
