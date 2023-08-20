#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]

use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::{resources::BoardOptions, BoardPlugin};

fn main() {
	let mut app = App::new();
	// Bevy default plugins
	app.add_plugins(DefaultPlugins);

	#[cfg(feature = "debug")]
	// Debug hierarchy inspector
	app.add_plugins(WorldInspectorPlugin::new());

	app.insert_resource(BoardOptions {
		map_size: (20, 20),
		bomb_count: 40,
		tile_padding: 3.0,
		..default()
	});

	app.add_plugins(BoardPlugin);

	// Run the app
	app.run();
}
