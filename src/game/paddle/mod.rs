use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;
use crate::AppState;
use super::InGameState;

pub const PADDLE_WIDTH: f32 = 104.0;
pub const PADDLE_HALF_WIDTH: f32 = PADDLE_WIDTH / 2.0;
pub const PADDLE_HEIGHT: f32 = 24.0;
pub const PADDLE_HALF_HEIGHT: f32 = PADDLE_HEIGHT / 2.0;
pub const PADDLE_SPEED: f32 = 500.0;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn_paddle)
            .add_systems(Update, move_paddle
                .run_if(in_state(InGameState::Play)))
            .add_systems(OnExit(AppState::InGame), despawn_paddle);
    }
}