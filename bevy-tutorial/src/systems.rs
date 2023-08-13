use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{events::*, AppState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
	let window = window_query.get_single().unwrap();

	commands.spawn(Camera2dBundle {
		transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
		..default()
	});
}

pub fn transition_to_game_state(
	keyboard_input: Res<Input<KeyCode>>,
	app_state: Res<State<AppState>>,
	mut next_app_state: ResMut<NextState<AppState>>
) {
	if keyboard_input.just_pressed(KeyCode::G) && *app_state.get() != AppState::Game {
		next_app_state.set(AppState::Game);
		println!("Entered {:?}", AppState::Game);
	}
}

pub fn transition_to_main_menu_state(
	keyboard_input: Res<Input<KeyCode>>,
	app_state: Res<State<AppState>>,
	mut next_app_state: ResMut<NextState<AppState>>
) {
	if keyboard_input.just_pressed(KeyCode::M) && *app_state.get() != AppState::MainMenu {
		next_app_state.set(AppState::MainMenu);
		println!("Entered {:?}", AppState::MainMenu);
	}
}

pub fn exit_game(
	keyboard_input: Res<Input<KeyCode>>,
	mut app_exit_event_writer: EventWriter<AppExit>
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		app_exit_event_writer.send(AppExit);
	}
}

pub fn handle_game_over(
	mut game_over_event_reader: EventReader<GameOver>,
	mut next_app_state: ResMut<NextState<AppState>>
) {
	for event in game_over_event_reader.iter() {
		println!("Your final score is: {}", event.final_score);
		next_app_state.set(AppState::GameOver);
	}
}
