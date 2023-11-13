use bevy::prelude::*;
use rand::prelude::random;
use crate::game::collider::BoxCollider;
use crate::game::events::{BrickDestroyed};
use crate::game::spark::SparkBundle;

const COLLECTABLE_WIDTH: f32 = 148.;
const COLLECTABLE_HEIGHT: f32 = 148.;

#[derive(Clone, Copy)]
pub enum CollectableType {
    PaddleSizeUp,
    PaddleSizeDown,
    PaddleSpeedUp,
    PaddleSpeedDown,
    BallSizeUp,
    BallSizeDown,
    BallSpeedUp,
    BallSpeedDown,
}

#[derive(Component)]
pub struct Collectable {
    pub collectable_type: CollectableType,
}

impl Collectable {
    const COLLECTABLE_TYPES: &'static [CollectableType] = &[
        CollectableType::PaddleSizeUp,
        CollectableType::PaddleSizeDown,
        CollectableType::PaddleSpeedUp,
        CollectableType::PaddleSpeedDown,
        CollectableType::BallSizeUp,
        CollectableType::BallSizeDown,
        CollectableType::BallSpeedUp,
        CollectableType::BallSpeedDown,
    ];

    pub fn random() -> Self {
        Self {
            collectable_type: Self::COLLECTABLE_TYPES[random::<usize>() % Self::COLLECTABLE_TYPES.len()]
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
    for brick_destroyed_event in brick_destroyed_events.read() {
        if random::<f32>() > 1.1 {
            continue;
        }

        let scale = 0.3;

        commands.spawn(
            (
                SparkBundle {
                    sprite_bundle: SpriteBundle {
                        transform: Transform {
                            translation: Vec3 {
                                x: brick_destroyed_event.brick_position.x,
                                y: brick_destroyed_event.brick_position.y,
                                z: 1.,
                            },
                            scale: scale * Vec3::ONE,
                            ..default()
                        },
                        texture: asset_server.load("sprites/coin_10.png"),
                        ..default()
                    },
                    box_collider: BoxCollider {
                        extends: scale * Vec2::new(COLLECTABLE_WIDTH / 2., COLLECTABLE_HEIGHT / 2.),
                    },
                    ..default()
                },
                Collectable::random(),
            )
        );
    }
}
