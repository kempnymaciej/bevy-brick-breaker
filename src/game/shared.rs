use bevy::prelude::*;
use super::settings::BallSettings;
use super::ball::Ball;
use super::collider::BoxCollider;
use super::paddle::Paddle;

pub fn keep_ball_at_paddle_center (
    paddle_query: Query<(&Transform, &BoxCollider), With<Paddle>>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<Paddle>)>,
    ball_settings: Res<BallSettings>,
)
{
    if let Ok((paddle_transform, paddle_collider)) = paddle_query.get_single() {
        for mut ball in ball_query.iter_mut() {
            ball.translation = Vec3 {
                x: paddle_transform.translation.x,
                y: paddle_transform.translation.y + paddle_collider.extends.y + ball_settings.get_radius(),
                z: 0.,
            }
        }
    }
}