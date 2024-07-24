use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
		.add_systems(Startup, (spawn_dig_scene, spawn_player))
		.add_systems(FixedUpdate, fixed_update)
		.run();
}

/// player drill facing downwards
#[derive(Component, Debug)]
struct Player {
	angular_speed: f32,
	speed:         f32
}

fn spawn_dig_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
	// camera
	commands.spawn({
		let mut cam = Camera2dBundle::default();
		cam.projection.scaling_mode = ScalingMode::FixedHorizontal(320.0);
		cam
	});
	// background
	commands.spawn(SpriteBundle {
		texture: asset_server.load("sketch_backdrop.png"),
		transform: Transform::from_translation(Vec3::NEG_Z * 10.0),
		..default()
	});

	// todo: initialize endless generation?
}
const PLAYER_SPAWN_SPEED: f32 = 14.0;
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		Player {
			angular_speed: 0f32,
			speed:         PLAYER_SPAWN_SPEED
		},
		SpriteBundle {
			texture: asset_server.load("drill.png"),
			// transform: Transform::IDENTITY, // todo: adjust size?
			..default()
		}
	));
}

// just fucking do everything in one loop, refactor later if needed
fn fixed_update(
	mut player: Query<(&mut Player, &mut Transform)>,
	time: Res<Time<Fixed>>,
	kb_buttons: Res<ButtonInput<KeyCode>>,
	gp_buttons: Res<ButtonInput<GamepadButton>>,
	gp_axes: Res<Axis<GamepadAxis>>,
	gamepads: ResMut<Gamepads>
) {
	let (mut player, mut player_transform) = player.get_single_mut().expect("no player?");
	// --- rotation aka direction ---
	const MAX_ANGULAR_SPEED: f32 = 0.55 * 360f32; // degrees per second?
	const TURN_ACCEL: f32 = MAX_ANGULAR_SPEED / 0.08;
	const TURN_DECEL: f32 = TURN_ACCEL * 1.85;
	const TURN_STOP: f32 = TURN_DECEL * 0.8;
	player.angular_speed = {
		let turn_dir = {
			const MIN_AXIS: f32 = 0.1;
			// Horizontal direction input check:
			// first check keyboard, then gamepad d-pad, then gamepad axis.
			let look_for = |kb_keys: &[KeyCode],
			                gp_button_type: GamepadButtonType,
			                axis_cmp: fn(f32) -> bool| {
				kb_buttons.any_pressed(kb_keys.iter().copied())
					|| gamepads.iter().any(|gamepad| {
						gp_buttons.pressed(GamepadButton {
							gamepad,
							button_type: gp_button_type
						}) || axis_cmp(
							gp_axes
								.get(GamepadAxis {
									gamepad,
									axis_type: GamepadAxisType::LeftStickX
								})
								.unwrap_or(0f32)
						)
					})
			};
			let anything_left = look_for(
				&[KeyCode::KeyA, KeyCode::ArrowLeft],
				GamepadButtonType::DPadLeft,
				|val| val < -MIN_AXIS
			);
			let anything_right = look_for(
				&[KeyCode::KeyD, KeyCode::ArrowRight],
				GamepadButtonType::DPadRight,
				|val| val > MIN_AXIS
			);
			match (anything_left, anything_right) {
				(true, true) | (false, false) => None, // both or neither means no turning
				(true, false) => Some(-1f32),          // turn left
				(false, true) => Some(1f32)            // turn right
			}
		};
		info!("turn_dir: {:?}", turn_dir);
		match turn_dir {
			None => {
				let current_dir = player.angular_speed.signum();
				let decelerated =
					player.angular_speed - current_dir * TURN_STOP * time.timestep().as_secs_f32();
				if decelerated.signum() != current_dir {
					0.0
				} else {
					decelerated
				}
			},
			Some(delta_dir) => {
				let delta_speed =
					if player.angular_speed == 0.0 || player.angular_speed.signum() == delta_dir {
						TURN_ACCEL
					} else {
						TURN_DECEL
					};
				let accelerated =
					player.angular_speed + delta_dir * delta_speed * time.timestep().as_secs_f32();
				f32::clamp(accelerated, -MAX_ANGULAR_SPEED, MAX_ANGULAR_SPEED)
			}
		}
	};
	player_transform.rotate_z(player.angular_speed.to_radians() * time.timestep().as_secs_f32());
	// limit angle
	let z_angles = player_transform
		.rotation
		.to_euler(EulerRot::XYZ)
		.2
		.to_degrees();
	if z_angles.abs() > 90.0 {
		player_transform.rotation = Quat::from_rotation_z(90.0f32.to_radians() * z_angles.signum());
		player.angular_speed = 0.0; // stop turning at limit
	}

	// --- position ---
	const DRILL_DECEL: f32 = 0.2 * PLAYER_SPAWN_SPEED;
	player.speed -= DRILL_DECEL * time.delta_seconds();
	// positive Y downwards in screen space? or is it local transform space?
	player_transform.translation.y -= player.speed * time.timestep().as_secs_f32();
}