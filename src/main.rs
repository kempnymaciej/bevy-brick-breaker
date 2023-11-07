mod game;
pub mod utility;
mod menu;
mod common;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResized;
use crate::common::better_button::BetterButtonPlugin;
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;

pub const WINDOW_WORLD_HEIGHT: f32 = 720.0;
pub const WINDOW_USABLE_WORLD_WIDTH: f32 = 1280.0;
const MIN_WINDOW_WIDTH_TO_HEIGHT: f32 = WINDOW_USABLE_WORLD_WIDTH / WINDOW_WORLD_HEIGHT;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins((MenuPlugin, GamePlugin, BetterButtonPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, guard_resolution)
        .run();
}

fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window>
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scaling_mode: ScalingMode::FixedVertical(WINDOW_WORLD_HEIGHT),
                ..default()
            },
            .. default()
        }
    );
}

fn guard_resolution(
    mut window_query: Query<&mut Window>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        if e.width / e.height < MIN_WINDOW_WIDTH_TO_HEIGHT {
            let mut window = window_query.get_single_mut().unwrap();
            window.resolution.set(MIN_WINDOW_WIDTH_TO_HEIGHT * e.height, e.height);
        }
    }
}