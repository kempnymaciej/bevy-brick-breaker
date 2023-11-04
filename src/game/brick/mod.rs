use bevy::prelude::*;
use systems::*;
use crate::AppState;
use crate::game::InGameState;

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
            .add_systems(OnEnter(AppState::InGame), spawn_bricks)
            .add_systems(Update, destroy_bricks_on_hit
                .run_if(in_state(InGameState::Play)))
            .add_systems(OnExit(AppState::InGame), despawn_bricks);
    }
}