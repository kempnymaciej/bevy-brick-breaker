use bevy::math::{Vec3Swizzles};
use bevy::prelude::*;
use rand::prelude::*;
use crate::utility;

use super::{BALL_RADIUS, BALL_RADIUS_SQUARED, BALL_SPEED};
use super::components::*;

pub fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ballBlue.png"),
            .. default()
        },
        Ball { direction: random_direction_2d() }));
}

fn random_direction_2d() -> Vec3 {
    let rand = random();
    let mut result_2d = Vec2::new(rand, 1.0 - rand);
    result_2d = result_2d.normalize();
    Vec3::new(result_2d.x, result_2d.y, 0.0)
}

pub fn move_balls(
    mut balls_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>
) {
    for (mut ball_transform, ball) in balls_query.iter_mut() {
        ball_transform.translation += BALL_SPEED * time.delta_seconds() * ball.direction;
    }
}

pub fn bounce_ball_on_obstacles(
    mut balls_query: Query<(&mut Transform, &mut Ball)>,
    mut obstacle_query: Query<(&Transform, &mut BallObstacle), Without<Ball>>
) {
    for (mut ball_transform, mut ball) in balls_query.iter_mut() {
        let mut ball_position = ball_transform.translation;

        for (obstacle_transform, obstacle_box) in obstacle_query.iter_mut() {
            let obstacle_position = obstacle_transform.translation;
            let obstacle_extends = obstacle_box.extends;

            // Calculate the distance between the centers of the rectangle and the circle
            let d = ball_position - obstacle_position;

            // Find the point on the rectangle closest to the circle
            let closest = Vec2 {
                x: utility::math::clamp(d.x, -obstacle_extends.x, obstacle_extends.x),
                y: utility::math::clamp(d.y, -obstacle_extends.y, obstacle_extends.y),
            };

            // Check if the distance between the circle's center and the closest point on the rectangle is less than or equal to the radius
            let distance = (d.x - closest.x) * (d.x - closest.x) + (d.y - closest.y) * (d.y - closest.y);

            if distance <= BALL_RADIUS_SQUARED
            {
                // Collision detected; calculate the collision point
                let collision = obstacle_position.xy() + closest;
                ball_position = xy0(collision + (ball_position.xy() - collision));
                ball.direction = xy0((ball_position.xy() - obstacle_position.xy()).normalize());
            }
            ball_transform.translation = ball_position;
        }
    }
}

pub fn bounce_ball_on_edges(
    window_query: Query<&Window>,
    mut balls_query: Query<(&mut Transform, &mut Ball)>,
) {
    let window = window_query.get_single().unwrap();
    let min_x = BALL_RADIUS;
    let max_x = window.width() - BALL_RADIUS;
    let min_y = BALL_RADIUS;
    let max_y = window.height() - BALL_RADIUS;

    for (mut ball_transform, mut ball) in balls_query.iter_mut() {
        let mut ball_position = ball_transform.translation;
        if ball_position.x < min_x {
            ball_position.x = min_x;
            ball.direction.x *= -1.0;
        } else if ball_position.x > max_x {
            ball_position.x = max_x;
            ball.direction.x *= -1.0;
        }

        if ball_position.y < min_y {
            ball_position.y = min_y;
            ball.direction.y *= -1.0;
        } else if ball_position.y > max_y {
            ball_position.y = max_y;
            ball.direction.y *= -1.0;
        }

        ball_transform.translation = ball_position;
    }
}

fn xy0(xy: Vec2) -> Vec3 {
    Vec3 { x: xy.x, y: xy.y, z: 0.0 }
}