use bevy::prelude::*;

use self::systems::*;
use super::SimulationState;
use crate::{game::system_sets::*, AppState};

pub mod components;
pub mod resources;
mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			// enter
			.add_systems(
				OnEnter(AppState::Game),
				spawn_player.in_set(SpawningSystemSet)
			)
			// running
			.add_systems(
				Update,
				(
					(player_movement, confine_player_movement)
						.chain()
						.in_set(MovementSystemSet),
					(player_hit_star, enemy_hit_player)
						.chain()
						.in_set(HitSystemSet)
				)
					.run_if(in_state(AppState::Game))
					.run_if(in_state(SimulationState::Running))
			)
			// exit
			.add_systems(OnExit(AppState::Game), despawn_player);
	}
}
