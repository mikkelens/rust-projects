#![allow(
	clippy::needless_pass_by_value,
	clippy::wildcard_imports,
	clippy::module_name_repetitions,
	clippy::type_complexity
)]

use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

pub mod events;
mod systems;
pub mod utils;

use self::systems::*;

mod game;
mod main_menu;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_state::<AppState>()
		.add_plugins((MainMenuPlugin, GamePlugin))
		.add_systems(PreStartup, spawn_camera)
		.add_systems(
			PostUpdate,
			(
				exit_game,
				handle_game_over,
				transition_to_game_state,
				transition_to_main_menu_state
			)
		)
		.run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
	#[default]
	MainMenu,
	Game,
	GameOver
}
