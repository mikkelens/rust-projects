use bevy::prelude::Component;

/// Bomb component
#[cfg_attr(
	feature = "debug",
	derive(bevy_inspector_egui::InspectorOptions, bevy::prelude::Reflect)
)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Bomb;
