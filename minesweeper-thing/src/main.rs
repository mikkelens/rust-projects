#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]

use bevy::{prelude::*, window::PrimaryWindow};
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::{components::Coordinates, resources::BoardOptions, BoardPlugin};

fn main() {
	let mut app = App::new();
	// Bevy default plugins
	app.add_plugins(DefaultPlugins.set(WindowPlugin {
		primary_window: Some(Window {
			title: "Mine Sweeper!".to_string(),
			resolution: (700.0, 800.0).into(),
			..default()
		}),
		..default()
	}));

	#[cfg(feature = "debug")]
	// Debug hierarchy inspector
	app.add_plugins(WorldInspectorPlugin::new());

	app.insert_resource(BoardOptions {
		map_size: (20, 20),
		bomb_count: 40,
		tile_padding: 3.0,
		..default()
	});

	app.add_systems(Startup, spawn_camera);
	app.add_plugins(BoardPlugin);
	app.register_type::<Coordinates>();

	// Run the app
	app.run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
	let window = window_query.get_single().expect("No primary window?");

	commands.spawn(Camera2dBundle {
		transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
		..default()
	});
}
