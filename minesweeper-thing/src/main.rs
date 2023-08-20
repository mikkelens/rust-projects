#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]

mod game;
mod states;

use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;

// use self::{game::*, states::*};

fn main() {
	let mut app = App::new();

	#[cfg(feature = "debug")] // debug hierachy inspector
	app.add_plugins(WorldInspectorPlugin::new());

	app.add_plugins(DefaultPlugins)
		.add_plugins(GamePlugin)
		.run();
}
