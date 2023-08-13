pub mod enemy;
mod player;
pub mod score;
mod star;
pub mod system_sets;
mod systems;

use bevy::prelude::*;

use self::{
	enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin, system_sets::*,
	systems::*
};
use crate::{events::GameOver, AppState};

pub struct GamePlugin;
impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<SimulationState>()
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
			// enter
			.add_systems(OnEnter(AppState::Game), pause_simulation)
			// plugins
			.add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
			// running
			.add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
			// exit
			.add_systems(OnExit(AppState::Game), resume_simulation);
	}
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
	#[default]
	Running,
	Paused
}
