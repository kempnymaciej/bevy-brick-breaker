mod systems;
mod components;
pub mod styles;

use bevy::prelude::*;
use systems::*;
use super::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu), (notify_menu_enter, spawn_menu_ui))
            .add_systems(Update, (check_play_trigger, check_quit_trigger)
                .run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), (notify_menu_exit, despawn_menu_ui));
    }
}

fn notify_menu_enter() {
    println!("menu entered");
}

fn notify_menu_exit() {
    println!("menu exited");
}