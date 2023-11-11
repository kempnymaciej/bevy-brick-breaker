use bevy::prelude::*;

pub enum CollisionType {
    Natural,
    Centric,
}

#[derive(Component)]
pub struct Ball {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct BallObstacle {
    pub collision_type: CollisionType,
    pub hit_flag: bool
}

