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

    pub const DEFAULT_SIZE_POINTS: i32 = 0;
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
    speed_points_clamped: usize,
    size_points: i32,
    size_points_clamped: usize,
}

impl PaddleSettings {
    const DEFAULT_SPEED_POINTS: usize = 4;
    const SPEED_LEVELS: &'static [f32] = &[200., 250., 300., 350., 400., 500., 600., 800., 1000., 1200., 1400.];

    const DEFAULT_SIZE_POINTS: usize = 3;
    const MIN_WIDTH: f32 = 0.75 * PADDLE_WIDTH;
    const SIZE_LEVEL_EXTRA_WIDTHS: &'static [f32] = &[0., 25., 49., 81., 121., 169., 225., 289., 361., 441., 529.];

    pub fn change_speed_points(&mut self, delta_points: i32) {
        self.speed_points += delta_points;
        self.speed_points_clamped = (self.speed_points.max(0) as usize)
            .min(Self::SPEED_LEVELS.len() - 1);
        println!("PaddleSettings.speed_points_clamped: {}", self.speed_points_clamped);
    }

    pub fn get_speed(&self) -> f32 {
        Self::SPEED_LEVELS[self.speed_points_clamped]
    }

    pub fn change_size_points(&mut self, delta_points: i32) {
        self.size_points += delta_points;
        self.size_points_clamped = (self.size_points.max(0) as usize)
            .min(Self::SIZE_LEVEL_EXTRA_WIDTHS.len() - 1);
        println!("PaddleSettings.size_points_clamped: {}", self.size_points_clamped);
    }

    pub fn get_width(&self) -> f32 {
        Self::MIN_WIDTH + Self::SIZE_LEVEL_EXTRA_WIDTHS[self.size_points_clamped]
    }
}

impl Default for PaddleSettings {
    fn default() -> Self {
        PaddleSettings {
            speed_points: PaddleSettings::DEFAULT_SPEED_POINTS as i32,
            speed_points_clamped: PaddleSettings::DEFAULT_SPEED_POINTS,
            size_points: PaddleSettings::DEFAULT_SIZE_POINTS as i32,
            size_points_clamped: PaddleSettings::DEFAULT_SIZE_POINTS,
        }
    }
}