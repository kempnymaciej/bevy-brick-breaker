use bevy::prelude::*;
use crate::{WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT};
use crate::game::BallHitGround;
use crate::game::collider::BoxCollider;

pub const BALL_SIZE: f32 = 22.0;
pub const BALL_RADIUS: f32 = BALL_SIZE / 2.0;
pub const BALL_RADIUS_SQUARED: f32 = BALL_RADIUS * BALL_RADIUS;
pub const BALL_SPEED: f32 = 600.0;

pub enum BallObstacleType {
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
)
{
    spawn_ball(
        &mut commands,
        &asset_server,
        Vec2::new(WINDOW_USABLE_WORLD_WIDTH / 2.0, WINDOW_WORLD_HEIGHT / 2.0),
        Vec2::new(0., 1.)
    )
}

pub fn spawn_ball(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
    direction: Vec2,
)
{
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.),
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

pub fn move_balls(
    mut balls_query: Query<(&mut Transform, &mut Ball)>,
    mut obstacle_query: Query<(&Transform, &BoxCollider, &mut BallObstacle), Without<Ball>>,
    time: Res<Time>,
    mut ball_hit_ground_events: EventWriter<BallHitGround>,
)
{
    for (mut ball_transform, ball) in balls_query.iter_mut() {
        ball_transform.translation += BALL_SPEED * time.delta_seconds() * ball.direction;
    }

    bounce_ball_on_obstacles(&mut balls_query, &mut obstacle_query);
    bounce_ball_on_edges(&mut balls_query, &mut ball_hit_ground_events);
}

fn bounce_ball_on_obstacles(
    balls_query: &mut Query<(&mut Transform, &mut Ball)>,
    obstacle_query: &mut Query<(&Transform, &BoxCollider, &mut BallObstacle), Without<Ball>>
)
{
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
            if (potential_collision - ball_position).length_squared() <= BALL_RADIUS_SQUARED
            {
                obstacle.hit_flag = true;

                match obstacle.obstacle_type {
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
    balls_query: &mut Query<(&mut Transform, &mut Ball)>,
    ball_hit_ground_events: &mut EventWriter<BallHitGround>,
)
{
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
            ball_hit_ground_events.send_default();
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