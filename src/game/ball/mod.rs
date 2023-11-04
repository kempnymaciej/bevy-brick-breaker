use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const BALL_SIZE: f32 = 22.0;
pub const BALL_RADIUS: f32 = BALL_SIZE / 2.0;
pub const BALL_RADIUS_SQUARED: f32 = BALL_RADIUS * BALL_RADIUS;
pub const BALL_SPEED: f32 = 500.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ball)
            .add_systems(Update, (
                move_balls,
                bounce_ball_on_obstacles,
                bounce_ball_on_edges).chain()
            );
    }
}