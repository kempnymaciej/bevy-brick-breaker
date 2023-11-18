use bevy::prelude::*;

#[derive(Event, Default)]
pub struct LastBallDestroyed;

#[derive(Event, Default)]
pub struct RestartRequested;
#[derive(Event, Default)]
pub struct MenuRequested;
#[derive(Event, Default)]
pub struct TogglePauseRequested;

#[derive(Event, Default)]
pub struct BrickDestroyed {
    pub brick_position: Vec2,
}