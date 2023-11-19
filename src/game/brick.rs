use bevy::prelude::*;

use super::ball::{BallObstacle};
use crate::{WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT};
use crate::game::events::BrickDestroyed;
use crate::game::resources::{BrickGhost, BrickRowSpawnCooldown};
use super::collider::BoxCollider;

pub const BRICK_WIDTH: f32 = 64.0;
pub const BRICK_HALF_WIDTH: f32 = BRICK_WIDTH / 2.0;
pub const BRICK_HEIGHT: f32 = 32.0;
pub const BRICK_HALF_HEIGHT: f32 = BRICK_HEIGHT / 2.0;

const NUMBER_OF_BRICKS_IN_ROW: i32 = (WINDOW_USABLE_WORLD_WIDTH / BRICK_WIDTH) as i32;
const BRICK_HORIZONTAL_SPACE: f32 = WINDOW_USABLE_WORLD_WIDTH / NUMBER_OF_BRICKS_IN_ROW as f32;

const START_NUMBER_OF_ROWS: i32 = 5;
const MAX_NUMBER_OF_ROWS: i32 = 9;
const TARGET_MIN_NUMBER_OF_BRICKS: i32 = START_NUMBER_OF_ROWS * NUMBER_OF_BRICKS_IN_ROW;

#[derive(Component)]
pub struct Brick;

pub fn spawn_bricks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    brick_ghost: Res<BrickGhost>,
)
{
    for row_index in 0..START_NUMBER_OF_ROWS {
        spawn_row(row_index, &mut commands, &asset_server, &brick_ghost);
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

pub fn spawn_row(
    row_index: i32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    brick_ghost: &Res<BrickGhost>,
)
{
    let obstacle_type = brick_ghost.get_obstacle_type();
    for x_index in 0..NUMBER_OF_BRICKS_IN_ROW {
        let x = x_index as f32 * BRICK_HORIZONTAL_SPACE + BRICK_HORIZONTAL_SPACE / 2.;
        let y = WINDOW_WORLD_HEIGHT - BRICK_HALF_HEIGHT - row_index as f32 * BRICK_HEIGHT;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/element_blue_rectangle.png"),
                ..default()
            },
            Brick {},
            BallObstacle::new(obstacle_type),
            BoxCollider {
                extends: Vec2::new(BRICK_HALF_WIDTH, BRICK_HALF_HEIGHT),
            }
        ));
    }
}

pub fn keep_spawning_bricks(
    mut commands: Commands,
    mut brick_query: Query<&mut Transform, With<Brick>>,
    asset_server: Res<AssetServer>,
    brick_ghost: Res<BrickGhost>,
    time: Res<Time>,
    mut brick_row_spawn_cooldown: ResMut<BrickRowSpawnCooldown>
)
{
    brick_row_spawn_cooldown.0.tick(time.delta());
    if !brick_row_spawn_cooldown.0.finished() {
        return;
    }

    let mut number_of_bricks = 0;
    let mut lowest_brick_y = f32::MAX;

    for brick in brick_query.iter() {
        number_of_bricks += 1;
        lowest_brick_y = lowest_brick_y.min(brick.translation.y);
    }

    let lowest_row_index = (-(lowest_brick_y - WINDOW_WORLD_HEIGHT + BRICK_HALF_HEIGHT) / BRICK_HEIGHT).round() as i32;

    if lowest_row_index >= MAX_NUMBER_OF_ROWS - 1 || number_of_bricks >= TARGET_MIN_NUMBER_OF_BRICKS
    {
        return;
    }

    for mut brick in brick_query.iter_mut() {
        brick.translation.y -= BRICK_HEIGHT;
    }

    spawn_row(0, &mut commands, &asset_server, &brick_ghost);
    brick_row_spawn_cooldown.0.reset();
}