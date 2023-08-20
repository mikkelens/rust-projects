use bevy::prelude::*;

use self::{
	components::{Tile, GRID_SIZE},
	states::AppState
};

pub mod components;
pub mod events;
pub mod resources;
pub mod states;
// pub mod systems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			OnEnter(AppState::Game(states::GameState::Playing)),
			spawn_board
		);
	}
}

#[allow(clippy::cast_precision_loss)]
fn spawn_board(
	mut commands: Commands,
	// window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>
) {
	// let window = window_query.get_single().unwrap();

	for y in 0..GRID_SIZE {
		for x in 0..GRID_SIZE {
			commands.spawn((
				SpriteBundle {
					transform: Transform::from_xyz(x as f32, y as f32, 0.0),
					texture: asset_server.load("sprites/border.png"),
					..default()
				},
				Tile::Empty
			));
		}
	}
}
