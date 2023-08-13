use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::{components::*, *};

pub fn spawn_stars(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>
) {
	let window = window_query.get_single().unwrap();
	for _ in 0..NUMBER_OF_STARS_AT_START {
		commands.spawn((
			SpriteBundle {
				transform: Transform::from_xyz(
					random::<f32>() * window.width(),
					random::<f32>() * window.height(),
					0.0
				),
				texture: asset_server.load("sprites/star.png"),
				..default()
			},
			Star
		));
	}
}

pub fn tick_star_spawn_timers(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
	star_spawn_timer.0.tick(time.delta());
}

pub fn spawn_stars_over_time(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
	star_spawn_timer: Res<StarSpawnTimer>
) {
	if star_spawn_timer.0.finished() {
		let window = window_query.get_single().unwrap();

		commands.spawn((
			SpriteBundle {
				transform: Transform::from_xyz(
					random::<f32>() * window.width(),
					random::<f32>() * window.height(),
					0.0
				),
				texture: asset_server.load("sprites/star.png"),
				..default()
			},
			Star
		));
	}
}
