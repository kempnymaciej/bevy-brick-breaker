use bevy::prelude::*;

#[derive(Event, Default)]
pub struct BallHitGround;

#[derive(Event, Default)]
pub struct BrickDestroyed {
    pub brick_position: Vec2,
}