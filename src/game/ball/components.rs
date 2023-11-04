use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct BallObstacle {
    pub extends: Vec2,
    pub hit_flag: bool
}

