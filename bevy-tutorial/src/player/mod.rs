use bevy::prelude::*;

use self::systems::*;
use crate::system_sets::*;

pub mod components;
pub mod resources;
mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_player.in_set(SpawningSystemSet))
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
			);
	}
}
