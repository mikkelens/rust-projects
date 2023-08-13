use bevy::prelude::*;

pub trait Vec3Representable {
	fn to_vec3(self) -> Vec3;
}
impl Vec3Representable for Vec2 {
	fn to_vec3(self) -> Vec3 {
		Vec3 {
			x: self.x,
			y: self.y,
			z: 0.0
		}
	}
}
