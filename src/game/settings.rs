use bevy::prelude::*;
use super::ball::BALL_SIZE;
use super::paddle::PADDLE_WIDTH;

#[derive(Resource)]
pub struct BallSpeed {
    points: usize,
}

impl BallSpeed {
    const DEFAULT_POINTS: usize = 3;
    const POINT_SPEEDS: &'static [f32] = &[250., 325., 400., 500.0, 600., 700., 900., 1100., 1300.];

    pub fn change_points(&mut self, delta_points: i32) {
        let points = (self.points as i32 + delta_points).max(0);
        self.points = (points as usize).min(Self::POINT_SPEEDS.len() - 1);
        println!("BallSpeed.points: {}/{}", self.points, Self::POINT_SPEEDS.len() - 1);
    }

    pub fn get_speed(&self) -> f32 {
        Self::POINT_SPEEDS[self.points]
    }
}

impl Default for BallSpeed {
    fn default() -> Self {
        BallSpeed {
            points: Self::DEFAULT_POINTS,
        }
    }
}

#[derive(Resource)]
pub struct BallSize {
    points: usize,
}

impl BallSize {
    const DEFAULT_POINTS: usize = 0;
    const POINT_SCALES: &'static [f32] = &[1.0, 2.0, 3.0];

    pub fn change_points(&mut self, delta_points: i32) {
        let points = (self.points as i32 + delta_points).max(0);
        self.points = (points as usize).min(Self::POINT_SCALES.len() - 1);
        println!("BallSize.points: {}/{}", self.points, Self::POINT_SCALES.len() - 1);
    }

    pub fn get_scale(&self) -> f32 {
        Self::POINT_SCALES[self.points]
    }

    pub fn get_scale3(&self) -> Vec3 {
        let scale = self.get_scale();
        Vec3::new(scale, scale, 1.)
    }

    pub fn get_radius(&self) -> f32 {
        self.get_scale() * BALL_SIZE / 2.0
    }
}

impl Default for BallSize {
    fn default() -> Self {
        Self {
            points: Self::DEFAULT_POINTS,
        }
    }
}

#[derive(Resource)]
pub struct PaddleSpeed {
    points: usize,
}

impl PaddleSpeed {
    const DEFAULT_POINTS: usize = 4;
    const POINT_SPEEDS: &'static [f32] = &[200., 250., 300., 350., 400., 500., 600., 800., 1000., 1200., 1400.];

    pub fn change_points(&mut self, delta_points: i32) {
        let points = (self.points as i32 + delta_points).max(0);
        self.points = (points as usize).min(Self::POINT_SPEEDS.len() - 1);
        println!("PaddleSpeed.points: {}/{}", self.points, Self::POINT_SPEEDS.len() - 1);
    }

    pub fn get_speed(&self) -> f32 {
        Self::POINT_SPEEDS[self.points]
    }
}

impl Default for PaddleSpeed {
    fn default() -> Self {
        Self {
            points: Self::DEFAULT_POINTS,
        }
    }
}

#[derive(Resource)]
pub struct PaddleSize {
    points: usize,
}

impl PaddleSize {
    const DEFAULT_POINTS: usize = 3;
    const MIN_WIDTH: f32 = 0.75 * PADDLE_WIDTH;
    const POINT_EXTRA_WIDTHS: &'static [f32] = &[0., 25., 49., 81., 121., 169., 225., 289., 361., 441., 529.];

    pub fn change_points(&mut self, delta_points: i32) {
        let points = (self.points as i32 + delta_points).max(0);
        self.points = (points as usize).min(Self::POINT_EXTRA_WIDTHS.len() - 1);
        println!("PaddleSize.points: {}/{}", self.points, Self::POINT_EXTRA_WIDTHS.len() - 1);
    }

    pub fn get_width(&self) -> f32 {
        Self::MIN_WIDTH + Self::POINT_EXTRA_WIDTHS[self.points]
    }
}

impl Default for PaddleSize {
    fn default() -> Self {
        Self {
            points: Self::DEFAULT_POINTS,
        }
    }
}