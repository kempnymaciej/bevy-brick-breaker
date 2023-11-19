use bevy::prelude::*;
use crate::{WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT};
use crate::game::events::LastBallDestroyed;
use crate::game::shared::xy0;
use super::collider::BoxCollider;
use super::resources::{BallSize, BallSpeed};

pub const BALL_SIZE: f32 = 22.0;

#[derive(Copy, Clone)]
pub enum BallObstacleType {
    Ghost,
    Natural,
    Centric,
}

#[derive(Component)]
pub struct Ball {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct BallObstacle {
    pub obstacle_type: BallObstacleType,
    pub hit_flag: bool,
}

impl BallObstacle {
    pub fn new(obstacle_type: BallObstacleType) -> Self {
        BallObstacle {
            obstacle_type,
            hit_flag: false,
        }
    }
}

pub fn spawn_first_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ball_size: Res<BallSize>,
)
{
    spawn_ball(
        &mut commands,
        &asset_server,
        Vec2::new(WINDOW_USABLE_WORLD_WIDTH / 2.0, WINDOW_WORLD_HEIGHT / 2.0),
        Vec2::new(0., 1.),
        &ball_size,
    )
}

pub fn spawn_ball(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
    direction: Vec2,
    ball_size: &BallSize,
)
{
    commands.spawn((
        SpriteBundle {
            transform: Transform{
                translation: Vec3::new(position.x, position.y, 0.),
                scale: ball_size.get_scale3(),
                ..default()
            },
            texture: asset_server.load("sprites/ballBlue.png"),
            .. default()
        },
        Ball { direction: Vec3::new(direction.x, direction.y, 0.) }
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

pub fn keep_ball_synced_with_settings(
    ball_size: Res<BallSize>,
    mut ball_query: Query<&mut Transform, With<Ball>>
)
{
    if ball_size.is_changed() {
        let ball_scale = ball_size.get_scale3();

        for mut ball_transform in ball_query.iter_mut() {
            ball_transform.scale = ball_scale;
        }
    }
}

pub fn move_balls(
    mut balls_query: Query<(&mut Transform, &mut Ball)>,
    mut obstacle_query: Query<(&Transform, &BoxCollider, &mut BallObstacle), Without<Ball>>,
    time: Res<Time>,
    ball_speed: Res<BallSpeed>,
    ball_size: Res<BallSize>,
)
{
    let ball_speed = ball_speed.get_speed();
    for (mut ball_transform, ball) in balls_query.iter_mut() {
        ball_transform.translation += ball_speed * time.delta_seconds() * ball.direction;
    }

    let ball_radius = ball_size.get_radius();
    bounce_ball_on_obstacles(ball_radius, &mut balls_query, &mut obstacle_query);
    bounce_ball_on_edges(ball_radius, &mut balls_query);
}

pub fn keep_destroying_balls(
    mut commands: Commands,
    mut last_ball_destroyed_events: EventWriter<LastBallDestroyed>,
    ball_query: Query<(Entity, &Transform), With<Ball>>,
    ball_size: Res<BallSize>,
)
{
    let destruction_level = -2. * ball_size.get_radius();
    let mut balls = 0;
    for (entity, transform) in ball_query.iter() {
        if transform.translation.y <= destruction_level
        {
            commands.entity(entity).despawn();
        }
        else
        {
            balls += 1;
        }
    }

    if balls == 0
    {
        last_ball_destroyed_events.send_default();
    }
}

fn bounce_ball_on_obstacles(
    ball_radius: f32,
    balls_query: &mut Query<(&mut Transform, &mut Ball)>,
    obstacle_query: &mut Query<(&Transform, &BoxCollider, &mut BallObstacle), Without<Ball>>
)
{
    let ball_radius_squared = ball_radius * ball_radius;

    for (ball_transform, mut ball) in balls_query.iter_mut() {
        let ball_position = ball_transform.translation.xy();
        let mut ball_flip_direction_x = false;
        let mut ball_flip_direction_y = false;
        let mut ball_override_direction: Option<Vec2> = None;

        for (obstacle_transform, obstacle_collider, mut obstacle) in obstacle_query.iter_mut() {
            let obstacle_position = obstacle_transform.translation.xy();
            let obstacle_extends = obstacle_collider.extends;

            let obstacle_to_ball = ball_position - obstacle_position;
            let obstacle_to_closest = Vec2 {
                x: obstacle_to_ball.x.clamp(-obstacle_extends.x, obstacle_extends.x),
                y: obstacle_to_ball.y.clamp(-obstacle_extends.y, obstacle_extends.y),
            };

            let potential_collision = obstacle_position + obstacle_to_closest;
            if (potential_collision - ball_position).length_squared() <= ball_radius_squared
            {
                obstacle.hit_flag = true;

                match obstacle.obstacle_type {
                    BallObstacleType::Ghost => { /* do nothing */ }
                    BallObstacleType::Natural => {
                        if ball_flip_direction_x && ball_flip_direction_y {
                            continue;
                        }

                        let offset = 1.;

                        if potential_collision.x > obstacle_position.x - obstacle_extends.x + offset
                            && potential_collision.x < obstacle_position.x + obstacle_extends.x - offset
                        {
                            ball_flip_direction_y = true;
                            continue;
                        }
                        if potential_collision.y > obstacle_position.y - obstacle_extends.y + offset
                            && potential_collision.y < obstacle_position.y + obstacle_extends.y - offset
                        {
                            ball_flip_direction_x = true;
                            continue;
                        }

                        ball_flip_direction_y = true;
                        ball_flip_direction_x = true;
                    }
                    BallObstacleType::Centric => {
                        ball_override_direction = Some(ball_position - obstacle_position);
                        break;
                    }
                };
            }
        }

        if let Some(direction) = ball_override_direction {
            ball.direction = xy0(direction.normalize());
        }
        else {
            if ball_flip_direction_x {
                ball.direction.x *= -1.;
            }
            if ball_flip_direction_y {
                ball.direction.y *= -1.;
            }
        }
    }
}


fn bounce_ball_on_edges(
    ball_radius: f32,
    balls_query: &mut Query<(&mut Transform, &mut Ball)>,
)
{
    let min_x = ball_radius;
    let max_x = WINDOW_USABLE_WORLD_WIDTH - ball_radius;
    let max_y = WINDOW_WORLD_HEIGHT - ball_radius;

    for (mut ball_transform, mut ball) in balls_query.iter_mut() {
        let mut ball_position = ball_transform.translation;
        if ball_position.x < min_x {
            ball_position.x = min_x;
            ball.direction.x *= -1.0;
        } else if ball_position.x > max_x {
            ball_position.x = max_x;
            ball.direction.x *= -1.0;
        }

        if ball_position.y > max_y {
            ball_position.y = max_y;
            ball.direction.y *= -1.0;
        }

        ball_transform.translation = ball_position;
    }
}