use bevy::prelude::*;

#[derive(Event)]
pub struct GameOver {
	pub final_score: u32
}
