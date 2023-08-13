use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
use resources::*;
use systems::*;

use crate::system_sets::*;

pub const NUMBER_OF_ENEMIES_AT_START: usize = 2;
pub const ENEMY_SIZE: f32 = 64.0; // sprite size
pub const ENEMY_SPEED: f32 = 200.0; // movement speed

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<EnemySpawnTimer>()
			.add_systems(Startup, spawn_enemies.in_set(SpawningSystemSet))
			.add_systems(
				Update,
				(
					(
						enemy_movement,
						update_enemy_direction,
						confine_enemy_movement
					)
						.chain()
						.in_set(MovementSystemSet),
					(tick_enemy_spawn_timers, spawn_enemies_over_time)
						.chain()
						.in_set(SpawningSystemSet)
				)
			);
	}
}
