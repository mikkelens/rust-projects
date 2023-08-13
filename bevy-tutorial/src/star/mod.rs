use bevy::prelude::*;

use self::{resources::*, systems::*};
use crate::system_sets::SpawningSystemSet;

pub mod components;
pub mod resources;
mod systems;

const NUMBER_OF_STARS_AT_START: usize = 3;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;
impl Plugin for StarPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.init_resource::<StarSpawnTimer>()
			.add_systems(Startup, spawn_stars.in_set(SpawningSystemSet))
			.add_systems(
				Update,
				(tick_star_spawn_timers, spawn_stars_over_time)
					.chain()
					.in_set(SpawningSystemSet)
			);
	}
}
