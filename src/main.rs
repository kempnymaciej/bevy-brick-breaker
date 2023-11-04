mod game;
pub mod utility;

use bevy::prelude::*;
use crate::game::GamePlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
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


