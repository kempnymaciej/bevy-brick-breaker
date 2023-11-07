use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::WINDOW_USABLE_WORLD_WIDTH;
use super::{PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_SPEED, PADDLE_HALF_WIDTH, PADDLE_HALF_HEIGHT, PADDLE_WIDTH_PER_SIZE_POINT};
use super::components::*;
use super::super::ball::components::BallObstacle;

pub fn spawn_paddle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let paddle_width = get_paddle_width(PaddleSize::get_default_points());

    commands.spawn((
        Paddle,
        TransformBundle {
            local: Transform::from_xyz(WINDOW_USABLE_WORLD_WIDTH / 2.0, PADDLE_HEIGHT / 2.0, 0.0),
            ..default()
        },
        BallObstacle {
            extends: Vec2::new(paddle_width / 2.0, PADDLE_HALF_HEIGHT),
            hit_flag: false
        }))
        .with_children(|builder| {
            builder.spawn((
                SpriteBundle {
                    transform: Transform{
                        scale: get_center_paddle_segment_local_scale(paddle_width),
                        ..default()
                    },
                    texture: asset_server.load("sprites/paddleBlue.png"),
                    sprite: Sprite {
                        rect: Some(Rect {
                            min: Vec2::new(PADDLE_WIDTH * 0.25, 0.0),
                            max: Vec2::new(PADDLE_WIDTH * 0.75, PADDLE_HEIGHT),
                        }),
                        ..default()
                    },
                    .. default()
                },
                PaddleSegment {
                    segment_type: PaddleSegmentType::Center,
                }
            ));

            builder.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: get_left_paddle_segment_local_position(paddle_width),
                        ..default()
                    },
                    texture: asset_server.load("sprites/paddleBlue.png"),
                    sprite: Sprite {
                        anchor: Anchor::CenterRight,
                        rect: Some(Rect {
                           min: Vec2::new(0.,0.),
                           max: Vec2::new(PADDLE_HALF_WIDTH, PADDLE_HEIGHT),
                        }),
                        .. default()
                    },
                    ..default()
                },
                PaddleSegment {
                    segment_type: PaddleSegmentType::Left,
                }
            ));

            builder.spawn((
                SpriteBundle {
                    transform: Transform{
                        translation: get_right_paddle_segment_local_position(paddle_width),
                        ..default()
                    },
                    texture: asset_server.load("sprites/paddleBlue.png"),
                    sprite: Sprite {
                        anchor: Anchor::CenterLeft,
                        rect: Some(Rect {
                            min: Vec2::new(PADDLE_HALF_WIDTH,0.),
                            max: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
                        }),
                        .. default()
                    },
                    ..default()
                },
                PaddleSegment {
                    segment_type: PaddleSegmentType::Right,
                }
            ));
        });
}

pub fn despawn_paddle(
    mut commands: Commands,
    paddle_query: Query<Entity, With<Paddle>>
) {
    if let Ok(paddle) = paddle_query.get_single() {
        commands.entity(paddle).despawn();
    }
}

pub fn move_paddle(
    input: Res<Input<KeyCode>>,
    mut paddle_query: Query<(&mut Transform, &BallObstacle), With<Paddle>>,
    time: Res<Time>
) {
    let mut value: f32 = 0.0;
    if input.pressed(KeyCode::Left) {
        value -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        value += 1.0;
    }

    if value != 0.0 {
        if let Ok((mut transform, obstacle)) = paddle_query.get_single_mut() {
            let mut position = transform.translation;
            position.x += value * PADDLE_SPEED * time.delta_seconds();

            let paddle_half_width = obstacle.extends.x;
            let min_x = paddle_half_width;
            let max_x = WINDOW_USABLE_WORLD_WIDTH - paddle_half_width;
            if position.x < min_x {
                position.x = min_x;
            }
            else if position.x > max_x {
                position.x = max_x;
            }
            transform.translation = position;
        }
    }
}

pub fn test_update_size(
    input: Res<Input<KeyCode>>,
    mut paddle_size: ResMut<PaddleSize>,
) {
    if input.just_pressed(KeyCode::Q) {
        paddle_size.change_size(-1);
    }
    else if input.just_pressed(KeyCode::E) {
        paddle_size.change_size(1);
    }
}

pub fn update_paddle_size(
    paddle_size: Res<PaddleSize>,
    mut paddle_query: Query<&mut BallObstacle, With<Paddle>>,
    mut paddle_segments_query: Query<(&mut Transform, &PaddleSegment)>
) {
    if paddle_size.is_changed() {
        let width = get_paddle_width(paddle_size.get_points());

        if let Ok(mut obstacle) = paddle_query.get_single_mut(){
            obstacle.extends = Vec2::new(width / 2.0, PADDLE_HALF_HEIGHT);
        }

        for (mut transform, segment) in paddle_segments_query.iter_mut() {
            match segment.segment_type {
                PaddleSegmentType::Center => {
                    transform.scale = get_center_paddle_segment_local_scale(width);
                }
                PaddleSegmentType::Left => {
                    transform.translation = get_left_paddle_segment_local_position(width);
                }
                PaddleSegmentType::Right => {
                    transform.translation = get_right_paddle_segment_local_position(width);
                }
            }
        }
    }
}

fn get_paddle_width(size_points: i32) -> f32 {
    PADDLE_WIDTH + size_points as f32 * PADDLE_WIDTH_PER_SIZE_POINT
}

fn get_center_paddle_segment_local_scale(width: f32) -> Vec3 {
    Vec3::new((width - PADDLE_WIDTH) / PADDLE_HALF_WIDTH, 1., 1.)
}

fn get_left_paddle_segment_local_position(width: f32) -> Vec3 {
    Vec3::new(-(width / 2.0 - PADDLE_HALF_WIDTH), 0., 0.)
}

fn get_right_paddle_segment_local_position(width: f32) -> Vec3 {
    Vec3::new(width / 2.0 - PADDLE_HALF_WIDTH, 0., 0.)
}