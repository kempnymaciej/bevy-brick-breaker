use bevy::prelude::*;
use crate::game::collectable::{Collectable, CollectableType};
use super::settings::{BallSize, BallSpeed, PaddleSize, PaddleSpeed};
use super::ball::Ball;
use super::collider::BoxCollider;
use super::paddle::Paddle;

pub fn keep_ball_at_paddle_center (
    paddle_query: Query<(&Transform, &BoxCollider), With<Paddle>>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<Paddle>)>,
    ball_size: Res<BallSize>,
)
{
    if let Ok((paddle_transform, paddle_collider)) = paddle_query.get_single() {
        for mut ball in ball_query.iter_mut() {
            ball.translation = Vec3 {
                x: paddle_transform.translation.x,
                y: paddle_transform.translation.y + paddle_collider.extends.y + ball_size.get_radius(),
                z: 0.,
            }
        }
    }
}

pub fn collect_collectables(
    mut commands: Commands,
    collectable_query: Query<(Entity, &Transform, &BoxCollider, &Collectable)>,
    paddle_query: Query<(&Transform, &BoxCollider), With<Paddle>>,
    mut ball_size: ResMut<BallSize>,
    mut ball_speed: ResMut<BallSpeed>,
    mut paddle_size: ResMut<PaddleSize>,
    mut paddle_speed: ResMut<PaddleSpeed>,
)
{
    if let Ok((paddle_transform, paddle_collider)) = paddle_query.get_single() {
        for (entity, transform, collider, collectable) in collectable_query.iter() {
            let overlap = BoxCollider::overlap(
                paddle_transform.translation.xy(), paddle_collider.extends,
                transform.translation.xy(), collider.extends,
            );

            if overlap {
                collect_collectable(&collectable.collectable_type,
                    &mut ball_size, &mut ball_speed, &mut paddle_size, &mut paddle_speed);
                commands.entity(entity).despawn();
            }
        }
    }
}

fn collect_collectable(
    collectable_type: &CollectableType,
    ball_size: &mut ResMut<BallSize>,
    ball_speed: &mut ResMut<BallSpeed>,
    paddle_size: &mut ResMut<PaddleSize>,
    paddle_speed: &mut ResMut<PaddleSpeed>,
)
{
    match collectable_type {
        CollectableType::PaddleSizeUp => {
            paddle_size.change_points(1);
        }
        CollectableType::PaddleSizeDown => {
            paddle_size.change_points(-1);
        }
        CollectableType::PaddleSpeedUp => {
            paddle_speed.change_points(1);
        }
        CollectableType::PaddleSpeedDown => {
            paddle_speed.change_points(-1);
        }
        CollectableType::BallSizeUp => {
            ball_size.change_points(1);
        }
        CollectableType::BallSizeDown => {
            ball_size.change_points(-1);
        }
        CollectableType::BallSpeedUp => {
            ball_speed.change_points(1);
        }
        CollectableType::BallSpeedDown => {
            ball_speed.change_points(-1);
        }
    }
}

pub fn xy0(xy: Vec2) -> Vec3 {
    Vec3 { x: xy.x, y: xy.y, z: 0.0 }
}