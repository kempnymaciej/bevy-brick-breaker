use bevy::prelude::*;
use rand::prelude::random;
use crate::game::collider::BoxCollider;
use crate::game::events::{BrickDestroyed};
use crate::game::spark::SparkBundle;

const COIN_META_INDEX: usize = 0;

const COLLECTABLE_METAS:  &'static [(CollectableType, CollectableMeta)] = &[
    (CollectableType::Coin, CollectableMeta {
        texture_path: "sprites/collectables/element_blue_square.png",
        z_order: 0.5,
        drop_weight: 50,
        scale: 0.6,
        extends: Vec2 {
            x: 32.0,
            y: 32.0,
        },
        ..CollectableMeta::default()
    }),
    (CollectableType::BallClone, CollectableMeta {
        texture_path: "sprites/collectables/ball_clone.png",
        drop_weight: 4,
        ..CollectableMeta::default()
    }),
    (CollectableType::BallSizeUp, CollectableMeta {
        texture_path: "sprites/collectables/ball_size_up.png",
        drop_weight: 1,
        ..CollectableMeta::default()
    }),
    (CollectableType::BallSizeDown, CollectableMeta {
        texture_path: "sprites/collectables/ball_size_down.png",
        drop_weight: 1,
        ..CollectableMeta::default()
    }),
    (CollectableType::BallSpeedUp, CollectableMeta {
        texture_path: "sprites/collectables/ball_speed_up.png",
        drop_weight: 3,
        ..CollectableMeta::default()
    }),
    (CollectableType::BallSpeedDown, CollectableMeta {
        texture_path: "sprites/collectables/ball_speed_down.png",
        drop_weight: 3,
        ..CollectableMeta::default()
    }),
    (CollectableType::GhostUp, CollectableMeta {
        texture_path: "sprites/collectables/ghost_up.png",
        drop_weight: 1,
        ..CollectableMeta::default()
    }),
    (CollectableType::GhostDown, CollectableMeta {
        texture_path: "sprites/collectables/ghost_down.png",
        drop_weight: 1,
        ..CollectableMeta::default()
    }),
    (CollectableType::PaddleSizeUp, CollectableMeta {
        texture_path: "sprites/collectables/paddle_size_up.png",
        drop_weight: 2,
        ..CollectableMeta::default()
    }),
    (CollectableType::PaddleSizeDown, CollectableMeta {
        texture_path: "sprites/collectables/paddle_size_down.png",
        drop_weight: 2,
        ..CollectableMeta::default()
    }),
    (CollectableType::PaddleSpeedUp, CollectableMeta {
        texture_path: "sprites/collectables/paddle_speed_up.png",
        drop_weight: 3,
        ..CollectableMeta::default()
    }),
    (CollectableType::PaddleSpeedDown, CollectableMeta {
        texture_path: "sprites/collectables/paddle_speed_down.png",
        drop_weight: 3,
        ..CollectableMeta::default()
    }),
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectableType {
    BallClone,
    BallSizeUp,
    BallSizeDown,
    BallSpeedUp,
    BallSpeedDown,
    Coin,
    GhostUp,
    GhostDown,
    PaddleSizeUp,
    PaddleSizeDown,
    PaddleSpeedUp,
    PaddleSpeedDown,
}

#[derive(Component)]
pub struct Collectable {
    pub collectable_type: CollectableType,
}

struct CollectableMeta<'s> {
    pub texture_path: &'s str,
    pub z_order: f32,
    pub extends: Vec2,
    pub scale: f32,
    pub drop_weight: usize,
}

impl CollectableMeta<'_> {
    const fn default() -> Self {
        Self {
            texture_path: "",
            z_order: 0.0,
            extends: Vec2 {
                x: 78.,//0.5 * 148.,
                y: 78.,//0.5 * 148.,
            },
            scale: 0.3,
            drop_weight: 1,
        }
    }
}

pub fn despawn_collectables(
    mut commands: Commands,
    collectable_query: Query<Entity, With<Collectable>>
)
{
    for collectable in collectable_query.iter() {
        commands.entity(collectable).despawn();
    }
}

pub fn keep_spawning_collectables(
    mut commands: Commands,
    mut brick_destroyed_events: EventReader<BrickDestroyed>,
    asset_server: Res<AssetServer>,
)
{
    let total_drop_weight = COLLECTABLE_METAS
        .iter()
        .fold(0, |sum, i| sum + i.1.drop_weight);

    for brick_destroyed_event in brick_destroyed_events.read() {
        let number_of_coins = 2 + random::<usize>() % 7;
        for _ in 0..number_of_coins {
            spawn_collectable(&mut commands, brick_destroyed_event.brick_position, COIN_META_INDEX, &asset_server);
        }

        let mut rand = random::<usize>() % (total_drop_weight + 1);
        let mut meta_index = 0;
        for potential_index in 0..COLLECTABLE_METAS.len() {
            if COLLECTABLE_METAS[potential_index].1.drop_weight == 0 {
                continue;
            }

            if rand < COLLECTABLE_METAS[potential_index].1.drop_weight {
                meta_index = potential_index;
                break;
            }

            rand -= COLLECTABLE_METAS[potential_index].1.drop_weight;
        }

        spawn_collectable(&mut commands, brick_destroyed_event.brick_position, meta_index, &asset_server);
    }
}

fn spawn_collectable(
    commands: &mut Commands,
    position: Vec2,
    meta_index: usize,
    asset_server: &Res<AssetServer>
)
{
    let (collectable_type, collectable_meta) = &COLLECTABLE_METAS[meta_index];

    commands.spawn(
        (
            SparkBundle {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: position.x,
                            y: position.y,
                            z: collectable_meta.z_order,
                        },
                        scale: collectable_meta.scale * Vec3::ONE,
                        ..default()
                    },
                    texture: asset_server.load(collectable_meta.texture_path),
                    ..default()
                },
                box_collider: BoxCollider {
                    extends: collectable_meta.scale * collectable_meta.extends,
                },
                ..default()
            },
            Collectable {
                collectable_type: collectable_type.clone(),
            },
        )
    );
}
