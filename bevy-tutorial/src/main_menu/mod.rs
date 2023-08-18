use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use systems::{
	interactions::{interact_with_play_button, interact_with_quit_button},
	layout::*
};

use crate::AppState;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app
			// enter
			.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
			// systems
			.add_systems(
				Update,
				(interact_with_play_button, interact_with_quit_button)
					.run_if(in_state(AppState::MainMenu))
			)
			// exit
			.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
	}
}
