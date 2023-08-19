use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod states;
pub mod systems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		// app.add_systems(schedule, systems)
	}
}
