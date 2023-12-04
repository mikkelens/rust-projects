use bevy::prelude::Component;

/// Bomb neighbor component
#[cfg_attr(
	feature = "debug",
	derive(bevy_inspector_egui::InspectorOptions, bevy::prelude::Reflect)
)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct BombNeighbor {
	/// Number of neighbor bombs
	pub count: u8
}
