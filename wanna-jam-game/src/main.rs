#![allow(clippy::needless_pass_by_value)] // disables unhelpful Query reference lint

use bevy::{prelude::*, DefaultPlugins};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, add_numbers)
		.add_systems(Update, timered_number_mentions)
		.run();
}

#[derive(Component)]
struct ID {
	num:   usize,
	timer: Timer
}

fn add_numbers(mut commands: Commands) {
	commands.spawn(ID {
		num:   1,
		timer: Timer::from_seconds(1.0, TimerMode::Repeating)
	});
	commands.spawn(ID {
		num:   2,
		timer: Timer::from_seconds(2.0, TimerMode::Repeating)
	});
	commands.spawn(ID {
		num:   4,
		timer: Timer::from_seconds(4.0, TimerMode::Repeating)
	});
	commands.spawn(ID {
		num:   6,
		timer: Timer::from_seconds(6.0, TimerMode::Repeating)
	});
}
fn timered_number_mentions(time: Res<Time>, mut query: Query<&mut ID>) {
	for mut id in &mut query {
		if id.timer.tick(time.delta()).just_finished() {
			println!(
				"Number {} just finished its {} second timer.",
				id.num,
				id.timer.duration().as_secs()
			);
		}
	}
}

// #[derive(Component)]
// struct Person;
// #[derive(Component)]
// struct Name(String);

// fn add_people(mut commands: Commands) {
// 	commands.spawn((Person, Name("Elaina Proctor".into())));
// 	commands.spawn((Person, Name("Renzo Hume".into())));
// 	commands.spawn((Person, Name("Zayna Nieves".into())));
// }

// fn greet_people(query: Query<&Name, With<Person>>) {
// 	for name in &query {
// 		println!("hello {}!", name.0);
// 	}
// }
