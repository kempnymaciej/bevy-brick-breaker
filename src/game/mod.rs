use bevy::prelude::*;

mod paddle;
mod ball;
mod brick;

use paddle::PaddlePlugin;
use ball::BallPlugin;
use brick::BrickPlugin;
use crate::AppState;

pub struct GamePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum InGameState {
    #[default]
    Play,
    Pause,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InGameState>()
            .add_plugins(PaddlePlugin)
            .add_plugins(BallPlugin)
            .add_plugins(BrickPlugin)
            .add_systems(Update, (check_menu_condition, toggle_pause)
                .run_if(in_state(AppState::InGame)));
    }
}

fn check_menu_condition(
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if input.just_pressed(KeyCode::G) {
        next_state.set(AppState::Menu)
    }
}

fn toggle_pause(
    input: Res<Input<KeyCode>>,
    current_state: Res<State<InGameState>>,
    mut next_state: ResMut<NextState<InGameState>>
) {
    if input.just_pressed(KeyCode::P) {
        if *current_state.get() == InGameState::Play {
            next_state.set(InGameState::Pause);
        }
        else {
            next_state.set(InGameState::Play);
        }
    }
}






