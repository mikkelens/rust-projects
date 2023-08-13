use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use systems::layout::*;

use crate::AppState;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app
			// enter
			.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
			// exit
			.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
	}
}
