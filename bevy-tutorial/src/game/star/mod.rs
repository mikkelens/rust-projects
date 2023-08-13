use bevy::prelude::*;

use self::{resources::*, systems::*};
use super::SimulationState;
use crate::{game::system_sets::SpawningSystemSet, AppState};

pub mod components;
pub mod resources;
mod systems;

const NUMBER_OF_STARS_AT_START: usize = 3;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;
impl Plugin for StarPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.init_resource::<StarSpawnTimer>()
			// enter
			.add_systems(
				OnEnter(AppState::Game),
				spawn_stars.in_set(SpawningSystemSet)
			)
			// running
			.add_systems(
				Update,
				(tick_star_spawn_timers, spawn_stars_over_time)
					.chain()
					.in_set(SpawningSystemSet)
					.run_if(in_state(AppState::Game))
					.run_if(in_state(SimulationState::Running))
			)
			// exit
			.add_systems(OnExit(AppState::Game), despawn_stars);
	}
}
