mod game;
pub mod utility;
mod menu;
mod common;

use bevy::prelude::*;
use crate::common::better_button::BetterButtonPlugin;
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;

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
            .. default()
        }
    );
}


