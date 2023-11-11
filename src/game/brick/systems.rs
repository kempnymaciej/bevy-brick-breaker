use bevy::prelude::*;
use crate::game::ball::components::{BallObstacle, CollisionType};
use crate::game::brick::components::Brick;
use crate::{WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT};
use crate::game::shared::collider::BoxCollider;
use super::{BRICK_HALF_HEIGHT, BRICK_HALF_WIDTH, BRICK_HEIGHT, BRICK_WIDTH};

pub fn spawn_bricks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let number_of_bricks_x = (WINDOW_USABLE_WORLD_WIDTH / BRICK_WIDTH) as i32;
    let x_space = WINDOW_USABLE_WORLD_WIDTH / number_of_bricks_x as f32;
    for y_index in 0..4 {
        for x_index in 0..number_of_bricks_x {
            let x = x_index as f32 * x_space + BRICK_HALF_WIDTH;
            let y = WINDOW_WORLD_HEIGHT - BRICK_HALF_HEIGHT - y_index as f32 * BRICK_HEIGHT;
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/element_blue_rectangle.png"),
                    ..default()
                },
                Brick {},
                BallObstacle {
                    hit_flag: false,
                    collision_type: CollisionType::Natural
                },
                BoxCollider {
                    extends: Vec2::new(BRICK_HALF_WIDTH, BRICK_HALF_HEIGHT),
                }
            ));
        }
    }
}

pub fn despawn_bricks(
    mut commands: Commands,
    bricks_query: Query<Entity, With<Brick>>
) {
    for brick in bricks_query.iter() {
        commands.entity(brick).despawn();
    }
}

pub fn destroy_bricks_on_hit(
    mut commands: Commands,
    bricks_query: Query<(Entity, &BallObstacle), With<Brick>>
) {
    for brick in bricks_query.iter() {
        if brick.1.hit_flag == true {
            commands.entity(brick.0).despawn();
        }
    }
}