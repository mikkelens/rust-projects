use bevy::prelude::*;

use crate::main_menu::{components::*, styles::*};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
	let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
	if let Ok(main_menu_entity) = main_menu_query.get_single() {
		commands.entity(main_menu_entity).despawn_recursive();
	}
}

#[allow(clippy::too_many_lines)]
fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
	commands
		.spawn((
			NodeBundle {
				style: MAIN_MENU_STYLE,
				..default()
			},
			MainMenu
		))
		.with_children(|parent| {
			// title
			parent
				.spawn(NodeBundle {
					style: TITLE_STYLE,
					..default()
				})
				.with_children(|parent| {
					// image 1
					parent.spawn(ImageBundle {
						style: IMAGE_STYLE,
						image: asset_server.load("sprites/ball_blue_large.png").into(),
						..default()
					});
					// text
					parent.spawn(TextBundle {
						text: Text {
							sections: vec![TextSection::new("Bevy Ball Game", title_text_style())],
							alignment: TextAlignment::Center,
							..default()
						},
						..default()
					});
					// image 2
					parent.spawn(ImageBundle {
						style: IMAGE_STYLE,
						image: asset_server.load("sprites/ball_red_large.png").into(),
						..default()
					});
				});

			// play button
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: NORMAL_BUTTON_COLOR.into(),
						..default()
					},
					PlayButton {}
				))
				.with_children(|parent| {
					parent.spawn(TextBundle {
						text: Text {
							sections: vec![TextSection::new("Play", button_text_style())],
							alignment: TextAlignment::Center,
							..default()
						},
						..default()
					});
				});
			// quit button
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: NORMAL_BUTTON_COLOR.into(),
						..default()
					},
					QuitButton {}
				))
				.with_children(|parent| {
					parent.spawn(TextBundle {
						text: Text {
							sections: vec![TextSection::new("Quit", button_text_style())],
							alignment: TextAlignment::Center,
							..default()
						},
						..default()
					});
				});
		})
		.id()
}
