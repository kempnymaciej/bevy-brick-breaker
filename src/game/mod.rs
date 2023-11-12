pub mod ball;
pub mod collider;
mod brick;
mod paddle;
pub mod settings;
mod shared;

use bevy::prelude::*;
use crate::{AppState};

use paddle::{despawn_paddles, spawn_paddle, move_paddle, keep_paddle_synced_with_settings};
use ball::{ spawn_first_ball, move_balls, despawn_balls };
use brick::{ despawn_bricks, destroy_bricks_on_hit, spawn_bricks };
use crate::game::ball::keep_ball_synced_with_settings;
use crate::game::settings::{BallSettings, PaddleSettings};
use crate::game::shared::keep_ball_at_paddle_center;

pub struct GamePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum InGameState {
    #[default]
    Preparation,
    Play,
    Pause,
    Summary,
}

#[derive(Event, Default)]
pub struct BallHitGround;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<InGameState>()
            .init_resource::<BallSettings>()
            .init_resource::<PaddleSettings>()
            .add_event::<BallHitGround>()
            .add_systems(OnEnter(AppState::InGame),
                (
                    spawn_first_ball,
                    spawn_paddle,
                    spawn_bricks,
                )
            )
            .add_systems(Update,
                 (
                     (
                         (move_paddle, keep_ball_at_paddle_center).chain(),
                         check_preparation_end_condition,
                     ).run_if(in_state(InGameState::Preparation)),
                     (
                         move_paddle,
                         move_balls,
                         destroy_bricks_on_hit,
                         test_settings,
                         keep_ball_synced_with_settings,
                         keep_paddle_synced_with_settings,
                         check_end_game,
                     ).run_if(in_state(InGameState::Play)),
                     (
                         check_menu_condition,
                         toggle_pause,
                     ),
                 ).run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame),
                 (
                     despawn_balls,
                     despawn_paddles,
                     despawn_bricks,
                     reset_resources,
                 )
            );
    }
}

fn reset_resources(
    mut commands: Commands
)
{
    commands.insert_resource(PaddleSettings::default());
    commands.insert_resource(BallSettings::default());
}

fn check_preparation_end_condition(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut next_state: ResMut<NextState<InGameState>>,
)
{
    if let Some(key) = keyboard_input.get_just_pressed().next() {
        if *key != KeyCode::Left && *key != KeyCode::Right {
            next_state.set(InGameState::Play);
        }
    }
    else if mouse_input.get_just_pressed().next() != None {
        next_state.set(InGameState::Play);
    }
}

fn check_end_game(
    mut ball_hit_ground: EventReader<BallHitGround>,
)
{
    for _event in ball_hit_ground.read() {
        println!("end");
    }
}

fn check_menu_condition(
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>
)
{
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

pub fn test_settings(
    input: Res<Input<KeyCode>>,
    mut paddle_settings: ResMut<PaddleSettings>,
    mut ball_settings: ResMut<BallSettings>,
)
{
    let value =
        if input.just_pressed(KeyCode::Q) { -1 }
        else if input.just_pressed(KeyCode::E) { 1 }
        else { 0 };

    if value == 0 {
        return;
    }

    if input.pressed(KeyCode::Key1) {
        paddle_settings.change_size_points(value);
    }
    if input.pressed(KeyCode::Key2) {
        paddle_settings.change_speed_points(value);
    }
    if input.pressed(KeyCode::Key3) {
        ball_settings.change_size_points(value);
    }
    if input.pressed(KeyCode::Key4) {
        ball_settings.change_speed_points(value);
    }
}