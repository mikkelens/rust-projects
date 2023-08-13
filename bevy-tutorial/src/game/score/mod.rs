use bevy::prelude::*;

use self::{resources::*, systems::*};
use super::SimulationState;
use crate::{game::system_sets::ScoreSystemSet, AppState};

pub mod components;
pub mod resources;
mod systems;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
	fn build(&self, app: &mut App) {
		app
			// init
			.init_resource::<HighScores>()
			// enter
			.add_systems(OnEnter(AppState::Game), insert_score)
			// running
			.add_systems(
				Update,
				(
					notice_score_change,
					notice_high_scores_change,
					add_score_to_high_score
				)
					.chain()
					.in_set(ScoreSystemSet)
					.run_if(in_state(AppState::Game))
					.run_if(in_state(SimulationState::Running))
			)
			// exit
			.add_systems(OnExit(AppState::Game), remove_score);
	}
}
