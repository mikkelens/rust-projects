use bevy::prelude::Component;

/// Uncover component, indicates a covered tile that should be uncovered
#[cfg_attr(
	feature = "debug",
	derive(bevy_inspector_egui::InspectorOptions, bevy::prelude::Reflect)
)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncover;
