use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};
use bevy_turborand::prelude::*;
use std::num::NonZeroU16;
use std::ops::RangeInclusive;
use std::time::Duration;

fn main() {
    App::new()
        .insert_resource(BulletQueue(Vec::new()))
        .insert_resource(ShipSpawner {
            timer: Timer::new(Duration::from_millis(250), TimerMode::Once),
        })
        .add_plugins(RngPlugin::default()) // creates a global rng resource
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_spewer.after(setup))
        .add_systems(Update, update_spewer_fire)
        .add_systems(FixedUpdate, move_transforms_with_velocity)
        .add_systems(
            FixedUpdate,
            spawn_ships.after(move_transforms_with_velocity),
        )
        .add_systems(FixedUpdate, spawn_queued_bullets.after(update_spewer_fire))
        .add_systems(
            FixedUpdate,
            check_bullet_collision.after(spawn_queued_bullets),
        )
        .add_systems(FixedUpdate, mover_cleanup.after(check_bullet_collision))
        .run();
}

const SCREEN_SIZE: Vec2 = Vec2::new(400., 225.);
const SCREEN_EDGE_OFFSET: Vec2 = Vec2::new(SCREEN_SIZE.x / 2., SCREEN_SIZE.y / 2.);

const BULLET_SPEED: RangeInclusive<f32> = 3.315f32..=3.795f32;
const BULLET_RADIUS: f32 = 4.785;

const SHIP_SPEED: RangeInclusive<f32> = 1.115f32..=1.375f32;
const SHIP_RADIUS: f32 = 9.35;

const SHIP_SPAWN_EXTRA_CLEARANCE: f32 = 14.25;
const SHIP_SPAWN_HEIGHT_VARIANCE: f32 = 0.535;
const SHIP_SPAWN_DELAY: RangeInclusive<f32> = 0.2525..=0.695;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    {
        // camera
        commands.spawn((
            Camera2dBundle::default(),
            PixelZoom::FitSize {
                width: SCREEN_SIZE.x as i32,
                height: SCREEN_SIZE.y as i32,
            },
            PixelViewport,
        ));
    }
    {
        // backdrop
        let image = asset_server.load("backdrop_smaller.png");
        commands.spawn(SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(0., 0., -5.),
            ..default()
        });
    }
}

#[derive(Resource)]
struct ShipSpawner {
    timer: Timer,
}

#[derive(Resource)]
struct BulletQueue(Vec<(Transform, Vec2, NonZeroU16)>);

#[derive(Component)]
struct Velocity {
    direction: Vec2,
    speed_magnitude: f32,
}

#[derive(Component)]
struct CircleCollider {
    radius: f32,
    enabled: bool,
}

#[derive(Component)]
struct Bullet {
    spawn_amount: NonZeroU16, // starts at two, increases incrementally
}

#[derive(Component)]
struct SpewerTag;

fn spawn_spewer(mut commands: Commands, asset_server: Res<AssetServer>) {
    eprintln!("spawned spewer!");
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("spewer.png"),
            transform: Transform::from_xyz(0., -SCREEN_EDGE_OFFSET.y, -0.1),
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            ..default()
        },
        SpewerTag,
    ));
}

trait Interpolated {
    fn lerp(&self, t: f32) -> f32;
    fn inv_lerp(&self, v: f32) -> f32;
    fn remap(&self, target: &Self, v: f32) -> f32 {
        target.lerp(self.inv_lerp(v))
    }
}
impl Interpolated for RangeInclusive<f32> {
    fn lerp(&self, t: f32) -> f32 {
        (1. - t) * self.start() + self.end() * t
    }
    fn inv_lerp(&self, v: f32) -> f32 {
        (v - self.start()) / (self.end() - self.start())
    }
}

fn spawn_ships(
    mut commands: Commands,
    mut ship_spawner: ResMut<ShipSpawner>,
    mut rng: ResMut<GlobalRng>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    ship_spawner.timer.tick(time.delta());
    if ship_spawner.timer.just_finished() {
        // eprintln!("spawned ship!");
        {
            // set up future ship spawn delay
            ship_spawner.timer.set_duration(Duration::from_secs_f32(
                SHIP_SPAWN_DELAY.lerp(rng.f32().powi(2)),
            ));
            ship_spawner.timer.reset()
        }
        {
            let sign = rng.f32_normalized();
            let mut transform = Transform::from_xyz(
                (SCREEN_EDGE_OFFSET.x + SHIP_SPAWN_EXTRA_CLEARANCE).copysign(sign),
                SCREEN_EDGE_OFFSET.y * SHIP_SPAWN_HEIGHT_VARIANCE * rng.f32_normalized(),
                0.,
            );
            let direction = Vec2::new(1.0f32.copysign(-sign), 0.);
            // eprintln!(
            //     "spawned ship with sign: {}, translation: {}, direction: {}",
            //     sign, transform.translation, direction
            // );
            transform.rotation = Quat::from_rotation_z(Vec2::NEG_X.angle_between(direction)); // turn towards direction but flipped
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("simple flyer.png"),
                    transform,
                    ..default()
                },
                Velocity {
                    direction,
                    speed_magnitude: SHIP_SPEED.lerp(rng.f32().powi(2)),
                },
                CircleCollider {
                    radius: SHIP_RADIUS,
                    enabled: true,
                },
            ));
        }
    }
}

fn move_transforms_with_velocity(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        let velocity = velocity.direction * velocity.speed_magnitude;
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn mover_cleanup(mut commands: Commands, query: Query<(&Transform, Entity), With<Velocity>>) {
    for (transform, id) in query.iter() {
        if transform.translation.xy().distance(Vec2::ZERO) > SCREEN_SIZE.x {
            // eprintln!("despawned moving entity (out of bounds)!");
            commands.entity(id).despawn();
        }
    }
}

fn update_spewer_fire(
    query: Query<&Transform, With<SpewerTag>>,
    keys: Res<Input<KeyCode>>,
    mut spawner: ResMut<BulletQueue>,
) {
    let spewer_transform = query.get_single().expect("singleton");
    if keys.any_just_pressed([KeyCode::Space, KeyCode::X, KeyCode::Return]) {
        eprintln!("player pressed a button to fire");
        spawner
            .0
            .push((*spewer_transform, Vec2::Y, 2.try_into().unwrap()));
    }
}

fn spawn_queued_bullets(
    mut commands: Commands,
    mut spawner: ResMut<BulletQueue>,
    mut rng: ResMut<GlobalRng>,
    asset_server: Res<AssetServer>,
) {
    if !spawner.0.is_empty() {
        eprintln!("spawned {} bullets!", spawner.0.len())
    }
    for (mut transform, direction, power) in spawner.0.drain(..) {
        transform.rotation = Quat::from_rotation_z(Vec2::X.angle_between(direction));
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("simple bullet.png"),
                transform,
                ..default()
            },
            Velocity {
                direction,
                speed_magnitude: BULLET_SPEED.lerp(rng.f32().powi(2)),
            },
            CircleCollider {
                radius: BULLET_RADIUS,
                enabled: true,
            },
            Bullet {
                spawn_amount: power,
            },
        ));
    }
}

trait Snapping {
    fn snapped_horizontal(&self) -> Self;
    fn snapped_cardinal(&self) -> Self;
    fn snapped_eight_way(&self) -> Self;
}
impl Snapping for Vec2 {
    fn snapped_horizontal(&self) -> Self {
        Self::new(self.x.signum(), 0.)
    }
    fn snapped_cardinal(&self) -> Self {
        if self.x.abs() >= self.y.abs() {
            Self::new(1f32.copysign(self.x), 0.)
        } else {
            Self::new(0., 1f32.copysign(self.y))
        }
    }
    fn snapped_eight_way(&self) -> Self {
        Self::from_angle(
            ((self.angle_between(Self::X).to_degrees() / 45.).round() * 45.).to_radians(),
        )
        .rotate(Self::X)
    }
}

fn check_bullet_collision(
    mut commands: Commands,
    mut bullet_spawner: ResMut<BulletQueue>,
    mut flyer_query: Query<(&Transform, &mut CircleCollider, Entity), Without<Bullet>>,
    mut bullet_query: Query<(&Transform, &Velocity, &mut CircleCollider, Entity, &Bullet)>,
) {
    for (flyer_transform, mut flyer_collider, flyer_id) in flyer_query.iter_mut() {
        if !flyer_collider.enabled {
            continue;
        }
        for (bullet_transform, bullet_velocity, mut bullet_collider, bullet_id, bullet_data) in
            bullet_query.iter_mut()
        {
            if !bullet_collider.enabled {
                continue;
            }
            let distance = flyer_transform
                .translation
                .xy()
                .distance(bullet_transform.translation.xy());
            if distance <= (flyer_collider.radius + bullet_collider.radius) {
                flyer_collider.enabled = false;
                bullet_collider.enabled = false;
                let current_power = bullet_data.spawn_amount.get();
                let new_bullet_power = bullet_data.spawn_amount.saturating_add(1);
                let per_offset = 0.5 * ((current_power) % 2) as f32;
                bullet_spawner.0.append(
                    &mut (0..current_power)
                        .map(|index| {
                            (
                                *flyer_transform,
                                Vec2::from_angle(
                                    (((per_offset + index as f32) / current_power as f32) * 360.)
                                        .to_radians(),
                                )
                                .rotate(bullet_velocity.direction.snapped_horizontal()),
                                new_bullet_power,
                            )
                        })
                        .collect::<Vec<_>>(),
                );
                eprintln!("bullet collision!");
                commands.entity(flyer_id).despawn();
                commands.entity(bullet_id).despawn();
                break; // skip all other projectile checks for this flyer
            }
        }
    }
}