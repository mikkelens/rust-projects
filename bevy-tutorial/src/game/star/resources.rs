use bevy::{
	prelude::*,
	time::{Timer, TimerMode}
};
const STAR_SPAWN_DELAY: f32 = 1.25;
#[derive(Resource)]
pub struct StarSpawnTimer(pub Timer);
impl Default for StarSpawnTimer {
	fn default() -> Self { Self(Timer::from_seconds(STAR_SPAWN_DELAY, TimerMode::Repeating)) }
}
