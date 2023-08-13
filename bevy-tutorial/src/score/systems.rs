use bevy::prelude::*;

use super::resources::*;
use crate::events::GameOver;

pub fn notice_score_change(score: Res<Score>) {
	if score.is_changed() {
		println!("Score: {}", score.0);
	}
}

pub fn notice_high_scores_change(high_scores: Res<HighScores>) {
	if high_scores.is_changed() {
		println!("High Scores:\n{:?}", high_scores.0);
	}
}

pub fn add_score_to_high_score(
	mut game_over_event_reader: EventReader<GameOver>,
	mut high_scores: ResMut<HighScores>
) {
	for event in game_over_event_reader.iter() {
		high_scores
			.0
			.push(("Player".to_string(), event.final_score));
	}
}
