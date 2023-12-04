use bevy::prelude::*;

use crate::{events::TileTriggerEvent, Board, Bomb, BombNeighbor, Coordinates, Uncover};

pub fn trigger_event_handler(
	mut commands: Commands,
	board: Res<Board>,
	mut tile_trigger_evr: EventReader<TileTriggerEvent>
) {
	for trigger_event in tile_trigger_evr.iter() {
		if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
			commands.entity(*entity).insert(Uncover);
		}
	}
}
