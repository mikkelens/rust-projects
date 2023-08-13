use bevy::{
	prelude::*,
	time::{Timer, TimerMode}
};

const ENEMY_SPAWN_DELAY: f32 = 5.0;
#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);
impl Default for EnemySpawnTimer {
	fn default() -> Self { Self(Timer::from_seconds(ENEMY_SPAWN_DELAY, TimerMode::Repeating)) }
}
