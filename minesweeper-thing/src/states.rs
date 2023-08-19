use bevy::prelude::*;

use crate::game::states::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
	#[default]
	Menu,
	Game(GameState)
}
impl States for AppState {
	type Iter = Box<dyn Iterator<Item = AppState>>;

	fn variants() -> Self::Iter {
		Box::new(
			[AppState::Menu]
				.into_iter()
				.chain(GameState::variants().map(AppState::Game))
		)
	}
}
