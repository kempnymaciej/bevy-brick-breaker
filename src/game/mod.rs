pub mod ball;
pub mod collider;
mod brick;
mod paddle;

use bevy::prelude::*;
use crate::{AppState};

use paddle::{ PaddleSize, despawn_paddles, spawn_paddle, move_paddle, update_paddle_size, test_update_paddle_size };
use ball::{ spawn_first_ball, move_balls, despawn_balls };
use brick::{ despawn_bricks, destroy_bricks_on_hit, spawn_bricks };
use crate::game::paddle::keep_ball_at_paddle_center;

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
            .add_event::<BallHitGround>()
            .insert_resource(PaddleSize::default())
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
                         move_paddle,
                         keep_ball_at_paddle_center,
                     ).chain().run_if(in_state(InGameState::Preparation)),
                     (
                         move_paddle,
                         move_balls,
                         destroy_bricks_on_hit,
                         test_update_paddle_size,
                         update_paddle_size,
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
                 )
            );
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