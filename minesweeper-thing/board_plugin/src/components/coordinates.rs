use std::{
	fmt::{self, Display, Formatter},
	ops::{Add, Sub}
};

use bevy::{prelude::Component, reflect::Reflect};
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectorOptions;

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
	pub x: u16,
	pub y: u16
}

impl Display for Coordinates {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "({}, {})", self.x, self.y) }
}

// We want to be able to make coordinates sums..
impl Add for Coordinates {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}
// ..and subtractions
impl Sub for Coordinates {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x.saturating_sub(rhs.x),
			y: self.y.saturating_sub(rhs.y)
		}
	}
}

impl Add<(i8, i8)> for Coordinates {
	type Output = Self;

	#[allow(
		clippy::cast_sign_loss,
		clippy::cast_lossless,
		clippy::cast_possible_wrap
	)]
	fn add(self, (x, y): (i8, i8)) -> Self::Output {
		let x = ((self.x as i16) + x as i16) as u16;
		let y = ((self.y as i16) + y as i16) as u16;
		Self { x, y }
	}
}
