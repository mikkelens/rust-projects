#![allow(
	clippy::needless_pass_by_value,
	clippy::wildcard_imports,
	clippy::module_name_repetitions
)]

use bevy::prelude::*;

pub mod enemy;
pub mod events;
mod player;
pub mod score;
mod star;
pub mod system_sets;
mod systems;
pub mod utils;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use self::{events::*, system_sets::*, systems::*};

fn main() {
	App::new()
		.configure_set(Startup, SpawningSystemSet)
		.configure_sets(
			Update,
			(
				MovementSystemSet,
				HitSystemSet,
				ScoreSystemSet,
				EvaluateStateSystemSet
			)
				.chain()
		)
		.add_event::<GameOver>()
		.add_plugins((
			DefaultPlugins,
			EnemyPlugin,
			PlayerPlugin,
			ScorePlugin,
			StarPlugin
		))
		.add_systems(Startup, spawn_camera.in_set(SpawningSystemSet))
		.add_systems(
			Update,
			(exit_game, handle_game_over).in_set(EvaluateStateSystemSet)
		)
		.run();
}
