use bevy::prelude::*;
use super::{PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_SPEED, PADDLE_HALF_WIDTH, PADDLE_HALF_HEIGHT};
use super::components::*;
use super::super::ball::components::BallObstacle;

pub fn spawn_paddle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, PADDLE_HEIGHT / 2.0, 0.0),
            texture: asset_server.load("sprites/paddleBlue.png"),
            .. default()
        },
         Paddle,
         BallObstacle {
            extends: Vec2::new(PADDLE_HALF_WIDTH, PADDLE_HALF_HEIGHT),
            just_hit: false
        }));
}

pub fn move_paddle(
    input: Res<Input<KeyCode>>,
    window_query: Query<&Window>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
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
        if let Ok(mut transform) = paddle_query.get_single_mut() {
            let mut position = transform.translation;
            position.x += value * PADDLE_SPEED * time.delta_seconds();

            let min_x = PADDLE_WIDTH / 2.0;
            let max_x = window_query.get_single().unwrap().width() - PADDLE_WIDTH / 2.0;
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