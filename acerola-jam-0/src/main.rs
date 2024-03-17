#![feature(is_sorted)]

mod cards;

use bevy::prelude::*;
use strum::*;

/// WORKING TITLE: Pokern't ("not poker")
///
/// I spent a little bit tinkering with an advanced module/modification system for some generic form of Texas hold'em, but it seemed too complicated. New/current objective: Reinvent basic poker
fn main() {
    // test
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup, spawn_card))
        .add_systems(Update, animate_cards)
        .run()
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct CardAnimationDriver {
    timer: Timer,
    suit_entity: Entity,
}

const SWITCH_DELAY: f32 = 0.35;
const RANKS: usize = cards::Rank::COUNT;
const SUITS: usize = cards::Suit::COUNT;
const RANK_SPRITE_OFFSET: usize = 1;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let background = asset_server.load("sprites/backgrounds.png");
    let panel = asset_server.load("sprites/panel.png");

    // camera
    commands.spawn(Camera2dBundle::default());

    // background
    commands.spawn(SpriteSheetBundle {
        texture: background,
        atlas: TextureAtlas {
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                Vec2::new(300.0, 300.0),
                1,
                1,
                None,
                None,
            )),
            index: 0,
        },
        transform: Transform::from_translation(Vec3::NEG_Z * 10.0),
        ..Default::default()
    });

    // panel
    commands.spawn(SpriteSheetBundle {
        texture: panel,
        atlas: TextureAtlas {
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                Vec2::new(300.0, 300.0),
                1,
                1,
                None,
                None,
            )),
            index: 0,
        },
        transform: Transform::from_translation(Vec3::NEG_Z * 5.0),
        ..Default::default()
    });
}

fn spawn_card(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let cards_texture = asset_server.load("sprites/cards.png");
    let suits_texture = asset_server.load("sprites/suits.png");

    // spawn backdrop
    let _shape = commands.spawn(SpriteSheetBundle {
        texture: cards_texture.clone_weak(),
        atlas: TextureAtlas {
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                Vec2::new(64.0, 64.0),
                SUITS,
                1,
                None,
                None,
            )),
            index: 0,
        },
        transform: Transform::from_translation(Vec3::NEG_Z),
        ..Default::default()
    });

    // spawn the suit display (& get its entity id)
    let suit_entity = {
        let suit_indices = AnimationIndices {
            first: 0,
            last: SUITS - 1,
        };
        commands
            .spawn((
                SpriteSheetBundle {
                    texture: suits_texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                            Vec2::new(16.0, 16.0),
                            SUITS,
                            1,
                            None,
                            None,
                        )),
                        index: suit_indices.first,
                    },
                    transform: Transform::from_translation(Vec3::new(-14.0, 22.0, 0.0)),
                    ..Default::default()
                },
                suit_indices,
            ))
            .id()
    };

    // spawn the rank display and its animation drivers
    {
        let rank_indices = AnimationIndices {
            first: RANK_SPRITE_OFFSET,
            last: RANKS,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture: cards_texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        Vec2::new(64.0, 64.0),
                        RANKS + RANK_SPRITE_OFFSET,
                        1,
                        None,
                        None,
                    )),
                    index: rank_indices.first,
                },
                ..Default::default()
            },
            rank_indices,
            CardAnimationDriver {
                timer: Timer::from_seconds(SWITCH_DELAY, TimerMode::Repeating),
                suit_entity,
            },
        ));
    }
}

fn animate_cards(
    time: Res<Time>,
    mut driver_query: Query<(
        &AnimationIndices,
        &mut CardAnimationDriver,
        &mut TextureAtlas,
    )>,
    mut non_driver_query: Query<
        (&AnimationIndices, &mut TextureAtlas),
        Without<CardAnimationDriver>,
    >,
) {
    for (driver_indices, mut driver, mut driver_atlas) in &mut driver_query {
        driver.timer.tick(time.delta());
        if driver.timer.just_finished() {
            driver_atlas.index = if driver_atlas.index == driver_indices.last {
                // update dependent (suit part of card)
                let (suit_indices, mut suit_atlas) = non_driver_query
                    .get_mut(driver.suit_entity)
                    .expect("card has suit display missing?");
                suit_atlas.index = if suit_atlas.index == suit_indices.last {
                    suit_indices.first
                } else {
                    suit_atlas.index + 1
                };
                driver_indices.first
            } else {
                driver_atlas.index + 1
            }
        }
    }
}