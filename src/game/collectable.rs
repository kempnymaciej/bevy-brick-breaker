use bevy::prelude::*;
use rand::prelude::random;
use crate::game::collider::BoxCollider;
use crate::game::events::{BrickDestroyed};

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

#[derive(Component)]
pub struct Spark {
    velocity: Vec2,
    rotation_sign: f32,
}

impl Spark {
    const MIN_INITIAL_SPEED: f32 = 200.;
    const MAX_INITIAL_SPEED: f32 = 400.;
    const TARGET_VELOCITY: Vec2 = Vec2::new(0., -900.);
    const VELOCITY_LERP_SPEED: f32 = 0.3;
    const VELOCITY_LENGTH_TO_ROTATION_SPEED: f32 = 0.02;

    pub fn update(&mut self, transform: &mut Transform, delta_time: f32) {
        self.velocity = self.velocity.lerp(
            Self::TARGET_VELOCITY, delta_time * Self::VELOCITY_LERP_SPEED);

        transform.translation.x += delta_time * self.velocity.x;
        transform.translation.y += delta_time * self.velocity.y;

        transform.rotate_z(delta_time
            * self.rotation_sign * self.velocity.length() * Self::VELOCITY_LENGTH_TO_ROTATION_SPEED);
    }

    pub fn random() -> Self {
        let initial_speed = Self::MIN_INITIAL_SPEED
            - random::<f32>() * (Self::MAX_INITIAL_SPEED - Self::MIN_INITIAL_SPEED);
        let initial_velocity = initial_speed
            * Vec2::new((random::<f32>() - 0.5) * 0.5, random::<f32>() * 0.5).normalize_or_zero();
        Self {
            velocity: initial_velocity,
            rotation_sign: if random::<bool>() { 1.0 } else { -1.0 },
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
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3{
                            x: brick_destroyed_event.brick_position.x,
                            y: brick_destroyed_event.brick_position.y,
                            z: 1.,
                        },
                        scale: scale* Vec3::ONE,
                        ..default()
                    },
                    texture: asset_server.load("sprites/coin_10.png"),
                    ..default()
                },
                BoxCollider {
                    extends: scale * Vec2::new(COLLECTABLE_WIDTH / 2., COLLECTABLE_HEIGHT / 2.),
                },
                Spark::random(),
                Collectable::random(),
            )
        );
    }
}

pub fn move_collectables(
    mut spark_query: Query<(&mut Spark, &mut Transform)>,
    time: Res<Time>,
)
{
    let delta_time = time.delta_seconds();
    for (mut spark, mut transform) in spark_query.iter_mut() {
        spark.update(&mut transform, delta_time);
    }
}

pub fn keep_despawning_collectables(
    mut commands: Commands,
    collectable_query: Query<(Entity, &Transform, &BoxCollider)>,
)
{
    for (entity, transform, collider) in collectable_query.iter() {
        if transform.translation.y + collider.extends.y < 0. {
            commands.entity(entity).despawn();
        }
    }
}
