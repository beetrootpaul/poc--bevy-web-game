use bevy::ecs::schedule::ShouldRun;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

use crate::game::animation::AnimationFrames;
#[cfg(debug_assertions)]
use crate::game::collision::HitCircle;
use crate::game::collision_debug::create_hit_circle_debug;
use crate::game::gui::TOPBAR_H;
use crate::game::sprites::{SpriteDimensions, SpriteSheet};
use crate::z_layer::Z_LAYER_SPRITES_COINS;

#[derive(Component)]
pub struct Coin;

#[derive(Bundle)]
struct CoinBundle {
    coin: Coin,
    sprite_sheet_bundle: SpriteSheetBundle,
    animation_frames: AnimationFrames,
    sprite_dimensions: SpriteDimensions,
    hit_circle: HitCircle,
}

pub fn create_coin_spawn_systems() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(there_is_no_coin)
        .with_system(spawn_coin)
}

fn there_is_no_coin(query: Query<&Coin>) -> ShouldRun {
    if query.iter().count() > 0 {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn spawn_coin(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let animation_frames = AnimationFrames { first: 0, last: 31 };
    let hit_circle = HitCircle {
        r: 3.5,
        offset: vec3(0., 0., 0.),
    };
    let mut parent_command = commands.spawn(CoinBundle {
        coin: Coin,
        sprite_sheet_bundle: SpriteSheetBundle {
            // TODO: reorganize game area position calculations
            // TODO: add helpers for translating from window-centered coors to game area coords
            // TODO: TEMPORARY COORDS
            transform: Transform::from_xyz(
                rng.gen_range(-40.0..40.0),
                -TOPBAR_H / 2. + rng.gen_range(-40.0..40.0),
                Z_LAYER_SPRITES_COINS,
            ),
            texture_atlas: sprite_sheet.texture_atlas_handle.clone().unwrap(),
            sprite: TextureAtlasSprite {
                index: animation_frames.first,
                anchor: Anchor::Center,
                ..default()
            },
            ..default()
        },
        animation_frames,
        sprite_dimensions: SpriteDimensions {
            width: 6.,
            height: 6.,
            ..default()
        },
        hit_circle: hit_circle.clone(),
    });

    #[cfg(debug_assertions)]
    parent_command.with_children(|parent| {
        parent.spawn(create_hit_circle_debug(
            &hit_circle,
            Z_LAYER_SPRITES_COINS,
            meshes,
            materials,
        ));
    });
}