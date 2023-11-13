pub mod ball;
pub mod collider;
mod brick;
mod paddle;
pub mod settings;
mod shared;
pub mod events;
mod collectable;

use bevy::prelude::*;
use crate::{AppState};

use paddle::{despawn_paddles, spawn_paddle, move_paddle, keep_paddle_synced_with_settings};
use ball::{ spawn_first_ball, move_balls, despawn_balls };
use brick::{ despawn_bricks, destroy_bricks_on_hit, spawn_bricks };
use crate::game::ball::keep_ball_synced_with_settings;
use crate::game::brick::keep_brick_synced_with_settings;
use crate::game::events::{BallHitGround, BrickDestroyed};
use crate::game::collectable::{despawn_collectables, keep_despawning_collectables, keep_spawning_collectables, move_collectables};
use crate::game::settings::{BallSize, BallSpeed, BrickGhost, PaddleSize, PaddleSpeed};
use crate::game::shared::{collect_collectables, keep_ball_at_paddle_center};

pub struct GamePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum InGameState {
    #[default]
    Preparation,
    Play,
    Pause,
    Summary,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<InGameState>()
            .init_resource::<BallSize>()
            .init_resource::<BallSpeed>()
            .init_resource::<BrickGhost>()
            .init_resource::<PaddleSize>()
            .init_resource::<PaddleSpeed>()
            .add_event::<BallHitGround>()
            .add_event::<BrickDestroyed>()
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
                         keep_brick_synced_with_settings,
                         keep_spawning_collectables,
                         move_collectables,
                         keep_despawning_collectables,
                         collect_collectables,
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
                     despawn_collectables,
                     reset_resources,
                 )
            );
    }
}

fn reset_resources(
    mut commands: Commands
)
{
    commands.insert_resource(BallSize::default());
    commands.insert_resource(BallSpeed::default());
    commands.insert_resource(BrickGhost::default());
    commands.insert_resource(PaddleSize::default());
    commands.insert_resource(PaddleSpeed::default());
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
    mut ball_size: ResMut<BallSize>,
    mut ball_speed: ResMut<BallSpeed>,
    mut brick_ghost: ResMut<BrickGhost>,
    mut paddle_size: ResMut<PaddleSize>,
    mut paddle_speed: ResMut<PaddleSpeed>,
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
        paddle_size.change_points(value);
    }
    if input.pressed(KeyCode::Key2) {
        paddle_speed.change_points(value);
    }
    if input.pressed(KeyCode::Key3) {
        ball_size.change_points(value);
    }
    if input.pressed(KeyCode::Key4) {
        ball_speed.change_points(value);
    }
    if input.pressed(KeyCode::Key5) {
        brick_ghost.set_enabled(value > 0);
    }
}