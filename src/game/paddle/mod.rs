use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;
use crate::AppState;
use crate::game::paddle::components::PaddleSize;
use super::InGameState;

pub const PADDLE_WIDTH: f32 = 104.0;
pub const PADDLE_HALF_WIDTH: f32 = PADDLE_WIDTH / 2.0;
pub const PADDLE_HEIGHT: f32 = 24.0;
pub const PADDLE_HALF_HEIGHT: f32 = PADDLE_HEIGHT / 2.0;
pub const PADDLE_SPEED: f32 = 500.0;

pub const PADDLE_WIDTH_PER_SIZE_POINT: f32 = 32.0;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PaddleSize::default())
            .add_systems(OnEnter(AppState::InGame), spawn_paddle)
            .add_systems(Update, (move_paddle, test_update_size, update_paddle_size)
                .run_if(in_state(InGameState::Play)))
            .add_systems(OnExit(AppState::InGame), despawn_paddle);
    }
}