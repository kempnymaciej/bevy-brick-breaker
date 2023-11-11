use bevy::prelude::*;
use rand::prelude::*;
use crate::{utility, WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT};
use crate::game::shared::collider::BoxCollider;

use crate::AppState;
use crate::game::InGameState;

pub const BALL_SIZE: f32 = 22.0;
pub const BALL_RADIUS: f32 = BALL_SIZE / 2.0;
pub const BALL_RADIUS_SQUARED: f32 = BALL_RADIUS * BALL_RADIUS;
pub const BALL_SPEED: f32 = 800.0;

pub enum CollisionType {
    Natural,
    Centric,
}

#[derive(Component)]
pub struct Ball {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct BallObstacle {
    pub collision_type: CollisionType,
    pub hit_flag: bool
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn_ball)
            .add_systems(Update, (
                move_balls,
                bounce_ball_on_obstacles,
                bounce_ball_on_edges).chain().run_if(in_state(InGameState::Play))
            )
            .add_systems(OnExit(AppState::InGame), despawn_balls);
    }
}

fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(WINDOW_USABLE_WORLD_WIDTH / 2.0, WINDOW_WORLD_HEIGHT / 2.0, 0.0),
            texture: asset_server.load("sprites/ballBlue.png"),
            .. default()
        },
        Ball { direction: random_direction_2d() }
    ));
}

pub fn despawn_balls(
    mut commands: Commands,
    balls_query: Query<Entity, With<Ball>>
) {
    for ball in balls_query.iter() {
        commands.entity(ball).despawn();
    }
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
    mut obstacle_query: Query<(&Transform, &BoxCollider, &mut BallObstacle), Without<Ball>>
) {
    for (mut ball_transform, mut ball) in balls_query.iter_mut() {
        let mut ball_position = ball_transform.translation;

        for (obstacle_transform, obstacle_collider, mut obstacle) in obstacle_query.iter_mut() {
            let obstacle_position = obstacle_transform.translation;
            let obstacle_extends = obstacle_collider.extends;

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
                obstacle.hit_flag = true;

                // Collision detected; calculate the collision point
                let collision = obstacle_position.xy() + closest;

                ball_position = xy0(collision + (ball_position.xy() - collision));
                ball.direction = match obstacle.collision_type {
                    CollisionType::Natural => {
                        let reflection = bounce_ball_on_box(collision, obstacle_position.xy(), obstacle_extends);
                        ball.direction.x *= reflection.x;
                        ball.direction.y *= reflection.y;
                        ball.direction
                    }
                    CollisionType::Centric => {
                        xy0((ball_position.xy() - obstacle_position.xy()).normalize())
                    }
                };
            }
            ball_transform.translation = ball_position;
        }
    }
}


pub fn bounce_ball_on_edges(
    mut balls_query: Query<(&mut Transform, &mut Ball)>,
) {
    let min_x = BALL_RADIUS;
    let max_x = WINDOW_USABLE_WORLD_WIDTH - BALL_RADIUS;
    let min_y = BALL_RADIUS;
    let max_y = WINDOW_WORLD_HEIGHT - BALL_RADIUS;

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

fn bounce_ball_on_box(ball_center: Vec2, box_center: Vec2, box_extends: Vec2) -> Vec2 {
    let delta_center = ball_center - box_center;
    let dx = delta_center.x.abs() - box_extends.x;
    let dy = delta_center.y.abs() - box_extends.y;

    if dx.abs() > dy.abs() {
        Vec2::new(1.0, -1.0)
    }
    else {
        Vec2::new(-1.0, 1.0)
    }
}

fn random_direction_2d() -> Vec3 {
    let rand = random();
    let mut result_2d = Vec2::new(rand, 1.0 - rand);
    result_2d = result_2d.normalize();
    Vec3::new(result_2d.x, result_2d.y, 0.0)
}