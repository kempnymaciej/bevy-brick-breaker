use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const PADDLE_WIDTH: f32 = 104.0;
pub const PADDLE_HALF_WIDTH: f32 = PADDLE_WIDTH / 2.0;
pub const PADDLE_HEIGHT: f32 = 24.0;
pub const PADDLE_HALF_HEIGHT: f32 = PADDLE_HEIGHT / 2.0;
pub const PADDLE_SPEED: f32 = 500.0;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_paddle)
            .add_systems(Update, move_paddle);
    }
}