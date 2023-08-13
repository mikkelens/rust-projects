use bevy::prelude::*;

use self::{resources::*, systems::*};
use crate::system_sets::ScoreSystemSet;

pub mod components;
pub mod resources;
mod systems;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<HighScores>()
			.init_resource::<Score>()
			.add_systems(
				Update,
				(
					notice_score_change,
					notice_high_scores_change,
					add_score_to_high_score
				)
					.chain()
					.in_set(ScoreSystemSet)
			);
	}
}
