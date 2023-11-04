use bevy::prelude::*;
use systems::spawn_bricks;
use crate::game::brick::systems::destroy_bricks_on_hit;

mod components;
mod systems;

pub const BRICK_WIDTH: f32 = 64.0;
pub const BRICK_HALF_WIDTH: f32 = BRICK_WIDTH / 2.0;
pub const BRICK_HEIGHT: f32 = 32.0;
pub const BRICK_HALF_HEIGHT: f32 = BRICK_HEIGHT / 2.0;

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_bricks)
            .add_systems(Update, destroy_bricks_on_hit);
    }
}