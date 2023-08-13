use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct Score(pub u32);
#[derive(Resource, Default)]
pub struct HighScores(pub Vec<(String, u32)>);
