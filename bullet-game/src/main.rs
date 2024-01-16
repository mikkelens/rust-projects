use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_single_flyer))
        .add_systems(FixedUpdate, move_flying)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Flying {
    horizontal_speed: f32,
    position: Vec2,
}

#[derive(Component)]
struct Hittable;

#[derive(Component)]
struct Hitting;

fn spawn_single_flyer(mut commands: Commands) {
    commands.spawn((
        Flying {
            horizontal_speed: 5f32,
            position: Vec2 { x: 10f32, y: 50f32 },
        },
        Hittable,
    ));
}

fn move_flying(mut query: Query<&mut Flying>) {
    for mut flyer in &mut query {
        flyer.position.x += flyer.horizontal_speed;
        dbg!(flyer.position);
    }
}