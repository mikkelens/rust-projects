use bevy::prelude::*;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) { app.add_systems(Startup, main_menu); }
}

fn main_menu() {
	println!("You are on the main menu.");
}
