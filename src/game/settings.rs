use bevy::prelude::*;
use super::ball::BALL_SIZE;
use super::paddle::PADDLE_WIDTH;

#[derive(Resource)]
pub struct BallSettings {
    speed_points: i32,
    speed_points_clamped: i32,
    size_points: i32,
    size_points_clamped: i32,
}

impl BallSettings {
    pub const DEFAULT_SPEED_POINTS: i32 = 0;
    pub const MIN_SPEED_POINTS: i32 = 0;
    pub const MAX_SPEED_POINTS: i32 = 10;
    pub const MIN_SPEED: f32 = 500.0;
    pub const SPEED_PER_SPEED_POINT: f32 = 32.0;

    pub const DEFAULT_SIZE_POINTS: i32 = 3;
    pub const MIN_SIZE_POINTS: i32 = 0;
    pub const MAX_SIZE_POINTS: i32 = 12;
    pub const MIN_SCALE: f32 = 1.0;
    pub const SCALE_PER_SIZE_POINT: f32 = 0.2;

    pub fn change_speed_points(&mut self, delta_points: i32) {
        self.speed_points += delta_points;
        self.speed_points_clamped = self.speed_points
            .clamp(BallSettings::MIN_SPEED_POINTS, BallSettings::MAX_SPEED_POINTS);
        println!("BallSettings.speed_points_clamped: {}", self.speed_points_clamped);
    }

    pub fn get_speed(&self) -> f32 {
        BallSettings::MIN_SPEED + self.speed_points_clamped as f32 * BallSettings::SPEED_PER_SPEED_POINT
    }

    pub fn change_size_points(&mut self, delta_points: i32) {
        self.size_points += delta_points;
        self.size_points_clamped = self.size_points
            .clamp(BallSettings::MIN_SIZE_POINTS, BallSettings::MAX_SIZE_POINTS);
        println!("BallSettings.size_points_clamped: {}", self.size_points_clamped);
    }

    pub fn get_scale(&self) -> f32 {
        BallSettings::MIN_SCALE + self.size_points_clamped as f32 * BallSettings::SCALE_PER_SIZE_POINT
    }

    pub fn get_scale3(&self) -> Vec3 {
        let scale = self.get_scale();
        Vec3::new(scale, scale, 1.)
    }

    pub fn get_radius(&self) -> f32 {
        self.get_scale() * BALL_SIZE / 2.0
    }
}

impl Default for BallSettings {
    fn default() -> Self {
        BallSettings {
            speed_points: BallSettings::DEFAULT_SPEED_POINTS,
            speed_points_clamped: BallSettings::DEFAULT_SPEED_POINTS,
            size_points: BallSettings::DEFAULT_SIZE_POINTS,
            size_points_clamped: BallSettings::DEFAULT_SIZE_POINTS,
        }
    }
}

#[derive(Resource)]
pub struct PaddleSettings {
    speed_points: i32,
    speed_points_clamped: i32,
    size_points: i32,
    size_points_clamped: i32,
}

impl PaddleSettings {
    pub const DEFAULT_SPEED_POINTS: i32 = 0;
    pub const MIN_SPEED_POINTS: i32 = 0;
    pub const MAX_SPEED_POINTS: i32 = 10;
    pub const MIN_SPEED: f32 = 500.0;
    pub const SPEED_PER_SPEED_POINT: f32 = 32.0;

    pub const DEFAULT_SIZE_POINTS: i32 = 3;
    pub const MIN_SIZE_POINTS: i32 = 0;
    pub const MAX_SIZE_POINTS: i32 = 12;
    pub const WIDTH_PER_SIZE_POINT: f32 = 32.0;

    pub fn change_speed_points(&mut self, delta_points: i32) {
        self.speed_points += delta_points;
        self.speed_points_clamped = self.speed_points
            .clamp(PaddleSettings::MIN_SPEED_POINTS, PaddleSettings::MAX_SPEED_POINTS);
        println!("PaddleSettings.speed_points_clamped: {}", self.speed_points_clamped);
    }

    pub fn get_speed(&self) -> f32 {
        PaddleSettings::MIN_SPEED + self.speed_points_clamped as f32 * PaddleSettings::SPEED_PER_SPEED_POINT
    }

    pub fn change_size_points(&mut self, delta_points: i32) {
        self.size_points += delta_points;
        self.size_points_clamped = self.size_points
            .clamp(PaddleSettings::MIN_SIZE_POINTS, PaddleSettings::MAX_SIZE_POINTS);
        println!("PaddleSettings.size_points_clamped: {}", self.size_points_clamped);
    }

    pub fn get_width(&self) -> f32 {
        PADDLE_WIDTH + self.size_points_clamped as f32 * PaddleSettings::WIDTH_PER_SIZE_POINT
    }
}

impl Default for PaddleSettings {
    fn default() -> Self {
        PaddleSettings {
            speed_points: PaddleSettings::DEFAULT_SPEED_POINTS,
            speed_points_clamped: PaddleSettings::DEFAULT_SPEED_POINTS,
            size_points: PaddleSettings::DEFAULT_SIZE_POINTS,
            size_points_clamped: PaddleSettings::DEFAULT_SIZE_POINTS,
        }
    }
}