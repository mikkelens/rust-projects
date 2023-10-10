use bevy::prelude::Event;

use crate::components::Coordinates;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent(pub Coordinates);
