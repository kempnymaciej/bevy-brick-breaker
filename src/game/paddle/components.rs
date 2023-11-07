use bevy::prelude::*;
use crate::game::paddle::{PADDLE_WIDTH, PADDLE_WIDTH_PER_SIZE_POINT};

pub enum PaddleSegmentType {
    Center,
    Left,
    Right,
}

#[derive(Resource)]
pub struct PaddleSize{
    points: i32,
    clamped_points: i32,
}

impl Default for PaddleSize {
    fn default() -> Self {
        let default_points = Self::get_default_points();
        PaddleSize { points: default_points, clamped_points: default_points }
    }
}

impl PaddleSize {
    pub fn change_size(&mut self, delta_points: i32) {
        self.points += delta_points;
        self.clamped_points =
            if self.points < 0 { 0 }
            else if self.points > 12 { 12 }
            else { self.points };
    }

    pub fn get_points(&self) -> i32 {
        self.clamped_points
    }

    pub fn get_default_points() -> i32 {
        3
    }
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct PaddleSegment {
    pub segment_type: PaddleSegmentType
}