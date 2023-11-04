use bevy::prelude::*;
use super::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu), notify_menu_enter)
            .add_systems(Update, check_game_condition.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), notify_menu_exit);
    }
}

fn notify_menu_enter() {
    println!("menu entered");
}

fn notify_menu_exit() {
    println!("menu exited");
}

fn check_game_condition(
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if input.just_pressed(KeyCode::G) {
        next_state.set(AppState::InGame)
    }
}