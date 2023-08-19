use bevy::{prelude::*, time::Timer};

#[derive(Resource)]
pub struct LevelTimer(pub Timer);
