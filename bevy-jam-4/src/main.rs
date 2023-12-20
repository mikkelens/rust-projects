use std::ops::RangeInclusive;
use bevy::prelude::*;

struct Rect<T> {
    width: RangeInclusive<T>,
    height: RangeInclusive<T>
}

const CANVAS_SIZE: Rect<f32> = Rect { width: 0.0..=300.0, height: 0.0..=200.0 };

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}