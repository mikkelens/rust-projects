use bevy::{
	audio::{Volume, VolumeLevel},
	prelude::*,
	sprite::SpriteBundle,
	window::{PrimaryWindow, Window}
};

use super::components::*;
use crate::{
	events::GameOver,
	game::{
		enemy::{components::Enemy, *},
		score::resources::*,
		star::{components::Star, *}
	},
	utils::Vec3Representable
};

const PLAYER_SIZE: f32 = 64.0; // sprite size
const PLAYER_SPEED: f32 = 500.0; // movement speed
pub fn spawn_player(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>
) {
	let window = window_query.get_single().unwrap();

	commands.spawn((
		SpriteBundle {
			transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
			texture: asset_server.load("sprites/ball_blue_large.png"),
			..default()
		},
		Player
	));
}
pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
	if let Ok(player_entity) = player_query.get_single() {
		commands.entity(player_entity).despawn();
	}
}

pub fn player_movement(
	keyboard_input: Res<Input<KeyCode>>,
	mut player_query: Query<&mut Transform, With<Player>>,
	time: Res<Time>
) {
	if let Ok(mut transform) = player_query.get_single_mut() {
		let mut direction = Vec2::ZERO;
		if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
			direction.x -= 1.0;
		}
		if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
			direction.x += 1.0;
		}
		if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
			direction.y += 1.0;
		}
		if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
			direction.y -= 1.0;
		}
		// 1 or 0 length exactly
		if direction.length() > 0.0 {
			direction = direction.normalize();
		}

		transform.translation += direction.to_vec3() * PLAYER_SPEED * time.delta_seconds();
	}
}

// collision/area confinement
pub fn confine_player_movement(
	mut player_query: Query<&mut Transform, With<Player>>,
	window_query: Query<&Window, With<PrimaryWindow>>
) {
	if let Ok(mut player_transform) = player_query.get_single_mut() {
		const HALF_PLAYER_SIZE: f32 = PLAYER_SIZE / 2.0;

		let window = window_query.get_single().unwrap();

		// calculate movable area
		let x_min = 0.0 + HALF_PLAYER_SIZE;
		let x_max = window.width() - HALF_PLAYER_SIZE;
		let y_min = 0.0 + HALF_PLAYER_SIZE;
		let y_max = window.height() - HALF_PLAYER_SIZE;

		// keep translation within area
		let mut translation = player_transform.translation;
		translation.x = translation.x.clamp(x_min, x_max);
		translation.y = translation.y.clamp(y_min, y_max);

		player_transform.translation = translation;
	}
}

pub fn enemy_hit_player(
	mut commands: Commands,
	mut game_over_event_writer: EventWriter<GameOver>,
	mut player_query: Query<(Entity, &Transform), With<Player>>,
	enemy_query: Query<(Entity, &Transform), With<Enemy>>,
	asset_server: Res<AssetServer>,
	score: Res<Score>
) {
	if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
		for (enemy_entity, enemy_transform) in &enemy_query {
			const PLAYER_RADIUS: f32 = PLAYER_SIZE / 2.0;
			const ENEMY_RADIUS: f32 = ENEMY_SIZE / 2.0;
			const NON_OVERLAP_DISTANCE: f32 = PLAYER_RADIUS + ENEMY_RADIUS;
			let distance = player_transform
				.translation
				.distance(enemy_transform.translation);
			if distance < NON_OVERLAP_DISTANCE {
				println!("Enemy hit player! Game Over!");
				commands.entity(enemy_entity).insert(AudioBundle {
					source:   asset_server.load("audio/explosionCrunch_000.ogg"),
					settings: PlaybackSettings::REMOVE
						.with_volume(Volume::Relative(VolumeLevel::new(0.15)))
				});
				commands.entity(player_entity).despawn();
				game_over_event_writer.send(GameOver {
					final_score: score.0
				});
			}
		}
	}
}
pub fn player_hit_star(
	mut commands: Commands,
	mut player_query: Query<(Entity, &Transform), With<Player>>,
	star_query: Query<(Entity, &Transform), With<Star>>,
	asset_server: Res<AssetServer>,
	mut score: ResMut<Score>
) {
	if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
		for (star_entity, star_transform) in &star_query {
			const PLAYER_RADIUS: f32 = PLAYER_SIZE / 2.0;
			const STAR_RADIUS: f32 = STAR_SIZE / 2.0;
			const NON_OVERLAP_DISTANCE: f32 = PLAYER_RADIUS + STAR_RADIUS;
			let distance = player_transform
				.translation
				.distance(star_transform.translation);
			if distance < NON_OVERLAP_DISTANCE {
				println!("Collected star!");
				commands.entity(player_entity).insert(AudioBundle {
					source:   asset_server.load("audio/laserLarge_000.ogg"),
					settings: PlaybackSettings::REMOVE
						.with_volume(Volume::Relative(VolumeLevel::new(0.2)))
				});
				score.0 += 10;
				commands.entity(star_entity).despawn();
			}
		}
	}
}
