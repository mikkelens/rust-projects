use bevy::prelude::*;

use crate::main_menu::components::MainMenu;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
	let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
	if let Ok(main_menu_entity) = main_menu_query.get_single() {
		commands.entity(main_menu_entity).despawn_recursive();
	}
}

fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
	commands
		.spawn((
			NodeBundle {
				style: Style {
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					..default()
				},
				background_color: Color::RED.into(),
				..default()
			},
			MainMenu
		))
		.id()
}
