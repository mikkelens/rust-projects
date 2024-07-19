use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
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
	commands.spawn(Camera2dBundle::default()); // todo: test this

	// background
	commands.spawn(SpriteBundle {
		texture: asset_server.load("sketch_backdrop.png"),
		..default()
	});

	// todo: initialize endless generation?
}
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		Player {
			angular_speed: 0f32,
			speed:         1f32
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
	time: Res<Time>,
	kb_buttons: Res<ButtonInput<KeyCode>>,
	gp_buttons: Res<ButtonInput<GamepadButton>>,
	gp_axes: Res<Axis<GamepadAxis>>,
	gamepads: ResMut<Gamepads>
) {
	let (mut player, mut player_transform) = player.get_single_mut().expect("no player?");
	player.speed -= 0.1 * time.delta_seconds(); // should be fixed?
	player.angular_speed = {
		let max_angular_speed = 0.5f32 * 360f32; // degrees per second?
		let angular_delta = max_angular_speed * 0.3;
		let delta_dir = {
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
				|val| val < -0.1f32
			);
			let anything_right = look_for(
				&[KeyCode::KeyD, KeyCode::ArrowRight],
				GamepadButtonType::DPadRight,
				|val| val > 0.1f32
			);
			match (anything_left, anything_right) {
				(true, true) | (false, false) => 0f32, // both or neither means no turning
				(true, false) => -1f32,                // turn left
				(false, true) => 1f32                  // turn right
			}
		};
		let accelerated = player.angular_speed + delta_dir * angular_delta;
		// todo: player input modifies velocity but should directly map direction
		f32::clamp(accelerated, -max_angular_speed, max_angular_speed)
	};
	player_transform.translation.y -= player.speed; // positive Y downwards in screen space?
	player_transform.rotate_z(player.angular_speed.to_radians());
}