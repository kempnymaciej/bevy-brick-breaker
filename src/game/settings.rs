use bevy::prelude::*;
use super::ball::BALL_SIZE;
use super::paddle::PADDLE_WIDTH;

#[derive(Resource)]
pub struct BallSettings {
    speed_points: i32,
    speed_points_clamped: usize,
    size_points: i32,
    size_points_clamped: usize,
}

impl BallSettings {
    const DEFAULT_SPEED_POINTS: usize = 3;
    const SPEED_LEVELS: &'static [f32] = &[250., 325., 400., 500.0, 600., 700., 900., 1100., 1300.];

    const DEFAULT_SIZE_POINTS: usize = 0;
    const SIZE_LEVEL_SCALES: &'static [f32] = &[1.0, 2.0, 3.0];

    pub fn change_speed_points(&mut self, delta_points: i32) {
        self.speed_points += delta_points;
        self.speed_points_clamped = (self.speed_points.max(0) as usize)
            .min(Self::SPEED_LEVELS.len() - 1);
        println!("BallSettings.speed_points_clamped: {}", self.speed_points_clamped);
    }

    pub fn get_speed(&self) -> f32 {
        Self::SPEED_LEVELS[self.speed_points_clamped]
    }

    pub fn change_size_points(&mut self, delta_points: i32) {
        self.size_points += delta_points;
        self.size_points_clamped = (self.size_points.max(0) as usize)
            .min(Self::SIZE_LEVEL_SCALES.len() - 1);
        println!("BallSettings.size_points_clamped: {}", self.size_points_clamped);
    }

    pub fn get_scale(&self) -> f32 {
        Self::SIZE_LEVEL_SCALES[self.size_points_clamped]
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
            speed_points: BallSettings::DEFAULT_SPEED_POINTS as i32,
            speed_points_clamped: BallSettings::DEFAULT_SPEED_POINTS,
            size_points: BallSettings::DEFAULT_SIZE_POINTS as i32,
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