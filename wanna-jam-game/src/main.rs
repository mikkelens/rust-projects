#![allow(clippy::needless_pass_by_value)] // disables unhelpful Query reference lint

use bevy::{
	prelude::{App, Commands, Component, Query, Startup, Update, With},
	DefaultPlugins
};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, add_people)
		.add_systems(Update, greet_people)
		.run();
}

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
	commands.spawn((Person, Name("Elaina Proctor".into())));
	commands.spawn((Person, Name("Renzo Hume".into())));
	commands.spawn((Person, Name("Zayna Nieves".into())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
	for name in &query {
		println!("hello {}!", name.0);
	}
}
