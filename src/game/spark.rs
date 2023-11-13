use std::ops::Range;
use bevy::prelude::*;
use rand::prelude::random;
use crate::game::collider::BoxCollider;

#[derive(Bundle, Default)]
pub struct SparkBundle {
    pub sprite_bundle: SpriteBundle,
    pub spark: Spark,
    pub box_collider: BoxCollider,
}

#[derive(Component)]
pub struct Spark {
    velocity: Vec2,
    target_velocity: Vec2,
    rotation_sign: f32,
    velocity_lerp_speed: f32,
    speed_to_rotation_speed: f32,
}

impl Default for Spark {
    fn default() -> Self {
        Self::random(
            200.0..400.0,
            850.0..950.0,
            0.3,
            0.02,
        )
    }
}

impl Spark {
    const TARGET_DIRECTION: Vec2 = Vec2::new(0., -1.);

    pub fn update(&mut self, transform: &mut Transform, delta_time: f32) {
        self.velocity = self.velocity.lerp(
            self.target_velocity, delta_time * self.velocity_lerp_speed);

        transform.translation.x += delta_time * self.velocity.x;
        transform.translation.y += delta_time * self.velocity.y;

        transform.rotate_z(delta_time
            * self.rotation_sign * self.velocity.length() * self.speed_to_rotation_speed);
    }

    pub fn random(
        initial_speed: Range<f32>,
        target_speed: Range<f32>,
        velocity_lerp_speed: f32,
        speed_to_rotation_speed: f32,
    ) -> Self
    {
        let initial_speed = initial_speed.start
            - random::<f32>() * (initial_speed.end - initial_speed.start);
        let initial_velocity = initial_speed
            * Vec2::new((random::<f32>() - 0.5) * 0.5, random::<f32>() * 0.5).normalize_or_zero();
        let target_speed = target_speed.start
            - random::<f32>() * (target_speed.end - target_speed.start);
        Self {
            velocity: initial_velocity,
            target_velocity: target_speed * Self::TARGET_DIRECTION,
            rotation_sign: if random::<bool>() { 1.0 } else { -1.0 },
            velocity_lerp_speed,
            speed_to_rotation_speed,
        }
    }
}

pub fn move_sparks(
    mut spark_query: Query<(&mut Spark, &mut Transform)>,
    time: Res<Time>,
)
{
    let delta_time = time.delta_seconds();
    for (mut spark, mut transform) in spark_query.iter_mut() {
        spark.update(&mut transform, delta_time);
    }
}

pub fn keep_despawning_sparks(
    mut commands: Commands,
    collectable_query: Query<(Entity, &Transform, &BoxCollider), With<Spark>>,
)
{
    for (entity, transform, collider) in collectable_query.iter() {
        if transform.translation.y + collider.extends.y < 0. {
            commands.entity(entity).despawn();
        }
    }
}