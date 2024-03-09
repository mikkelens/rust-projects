#![feature(is_sorted)]

mod cards;

use bevy::prelude::*;

/// WORKING TITLE: Pokern't ("not poker")
///
/// I spent a little bit tinkering with an advanced module/modification system for some generic form of Texas hold'em, but it seemed too complicated. New/current objective: Reinvent basic poker
fn main() {
    // test
    App::new().add_plugins(DefaultPlugins).run()
}
