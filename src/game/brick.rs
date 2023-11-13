use bevy::prelude::*;

use super::ball::{BallObstacle, BallObstacleType};
use crate::{WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT};
use crate::game::events::BrickDestroyed;
use crate::game::settings::BrickGhost;
use super::collider::BoxCollider;

pub const BRICK_WIDTH: f32 = 64.0;
pub const BRICK_HALF_WIDTH: f32 = BRICK_WIDTH / 2.0;
pub const BRICK_HEIGHT: f32 = 32.0;
pub const BRICK_HALF_HEIGHT: f32 = BRICK_HEIGHT / 2.0;

#[derive(Component)]
pub struct Brick;

pub fn spawn_bricks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
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
                BallObstacle::new(BallObstacleType::Natural),
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
)
{
    for brick in bricks_query.iter() {
        commands.entity(brick).despawn();
    }
}

pub fn destroy_bricks_on_hit(
    mut commands: Commands,
    bricks_query: Query<(Entity, &BallObstacle, &Transform), With<Brick>>,
    mut brick_destroyed_events: EventWriter<BrickDestroyed>,
)
{
    for (entity, obstacle, transform) in bricks_query.iter() {
        if obstacle.hit_flag == true {
            commands.entity(entity).despawn();
            brick_destroyed_events.send(BrickDestroyed { brick_position: transform.translation.xy()});
        }
    }
}

pub fn keep_brick_synced_with_settings(
    mut bricks_query: Query<&mut BallObstacle, With<Brick>>,
    brick_ghost: Res<BrickGhost>,
)
{
    if brick_ghost.is_changed() {
        let obstacle_type = brick_ghost.get_obstacle_type();

        for mut obstacle in bricks_query.iter_mut() {
            obstacle.obstacle_type = obstacle_type;
        }
    }
}