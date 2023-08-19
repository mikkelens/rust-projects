mod game;
mod states;

use bevy::prelude::*;

use self::{game::*, states::*};

fn main() {
	App::new()
		.add_state::<AppState>()
		.add_plugins(DefaultPlugins)
		.add_plugins(GamePlugin)
		.run();
}
