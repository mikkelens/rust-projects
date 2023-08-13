use bevy::{
	audio::{Volume, VolumeLevel},
	prelude::*,
	window::PrimaryWindow
};
use rand::random;

use super::{components::*, *};
use crate::utils::Vec3Representable;

pub fn spawn_enemies_start(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>
) {
	let window = window_query.get_single().unwrap();

	for _ in 0..NUMBER_OF_ENEMIES_AT_START {
		commands.spawn((
			SpriteBundle {
				transform: Transform::from_xyz(
					random::<f32>() * window.width(),
					random::<f32>() * window.height(),
					0.0
				),
				texture: asset_server.load("sprites/ball_red_large.png"),
				..default()
			},
			Enemy {
				direction: Vec2 {
					x: random(),
					y: random()
				}
				.normalize()
			}
		));
	}
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
	for enemy_entity in &enemy_query {
		commands.entity(enemy_entity).despawn();
	}
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
	for (mut transform, enemy) in enemy_query.iter_mut() {
		transform.translation += enemy.direction.to_vec3() * ENEMY_SPEED * time.delta_seconds();
	}
}
pub fn update_enemy_direction(
	mut commands: Commands, // spawn audio
	mut enemy_query: Query<(&Transform, &mut Enemy, Entity)>,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>
) {
	const HALF_ENEMY_SIZE: f32 = ENEMY_SIZE / 2.0;
	let window = window_query.get_single().unwrap();

	// calculate movable area
	let x_min = 0.0 + HALF_ENEMY_SIZE;
	let x_max = window.width() - HALF_ENEMY_SIZE;
	let y_min = 0.0 + HALF_ENEMY_SIZE;
	let y_max = window.height() - HALF_ENEMY_SIZE;

	for (transform, mut enemy, entity) in enemy_query.iter_mut() {
		let translation = transform.translation;
		let mut direction_changed = false;
		if translation.x < x_min || translation.x > x_max {
			enemy.direction.x *= -1.0;
			direction_changed = true;
		}
		if translation.y < y_min || translation.y > y_max {
			enemy.direction.y *= -1.0;
			direction_changed = true;
		}

		if direction_changed {
			commands.entity(entity).insert(AudioBundle {
				source:   asset_server.load(if random() {
					"audio/pluck_001.ogg"
				} else {
					"audio/pluck_002.ogg"
				}),
				settings: PlaybackSettings::REMOVE
					.with_volume(Volume::Relative(VolumeLevel::new(0.2)))
			});
		}
	}
}

pub fn confine_enemy_movement(
	mut enemy_query: Query<&mut Transform, With<Enemy>>,
	window_query: Query<&Window, With<PrimaryWindow>>
) {
	const HALF_ENEMY_SIZE: f32 = ENEMY_SIZE / 2.0;
	let window = window_query.get_single().unwrap();

	// calculate movable area
	let x_min = 0.0 + HALF_ENEMY_SIZE;
	let x_max = window.width() - HALF_ENEMY_SIZE;
	let y_min = 0.0 + HALF_ENEMY_SIZE;
	let y_max = window.height() - HALF_ENEMY_SIZE;

	for mut transform in enemy_query.iter_mut() {
		// keep translation within area
		let mut translation = transform.translation;
		translation.x = translation.x.clamp(x_min, x_max);
		translation.y = translation.y.clamp(y_min, y_max);

		transform.translation = translation;
	}
}

pub fn tick_enemy_spawn_timers(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
	enemy_spawn_timer.0.tick(time.delta());
}
pub fn spawn_enemies_over_time(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
	star_spawn_timer: Res<EnemySpawnTimer>
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
				texture: asset_server.load("sprites/ball_red_large.png"),
				..default()
			},
			Enemy {
				direction: Vec2 {
					x: random(),
					y: random()
				}
				.normalize()
			}
		));
	}
}
