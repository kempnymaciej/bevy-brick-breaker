use bevy::app::AppExit;
use bevy::prelude::*;
use crate::AppState;
use crate::common::better_button::{ColorButton, ReleaseButton};
use crate::menu::components::*;
use crate::menu::styles::*;

pub fn spawn_menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/OpenSans-Regular.ttf");
    let button_image = asset_server.load("sprites/buttonDefault.png");

    commands.spawn((
        MenuUiRoot {},
        NodeBundle {
            style: get_menu_root_style(),
            ..default()
        }
    ))
        .with_children(|builder| {
            builder.spawn((
                ButtonBundle {
                    style: get_menu_button_style(),
                    image: UiImage::new(button_image.clone()),
                    ..default()
                },
                ReleaseButton::default(),
                ColorButton::default(),
                PlayButton
            ))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            builder.spawn((
                ButtonBundle {
                    style: get_menu_button_style(),
                    image: UiImage::new(button_image.clone()),
                    ..default()
                },
                ReleaseButton::default(),
                ColorButton::default(),
                QuitButton
            ))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}

pub fn despawn_menu_ui(
    mut commands: Commands,
    root_query: Query<Entity, With<MenuUiRoot>>
) {
    if let Ok(root) = root_query.get_single() {
        commands.entity(root).despawn_recursive();
    }
}

pub fn check_play_trigger(
    play_button_query: Query<&ReleaseButton, With<PlayButton>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    for button in play_button_query.iter() {
        if button.just_released {
            next_state.set(AppState::InGame);
            return;
        }
    }
}

pub fn check_quit_trigger(
    quit_button_query: Query<&ReleaseButton, With<QuitButton>>,
    mut app_exit_event_writer: EventWriter<AppExit>
) {
    for button in quit_button_query.iter() {
        if button.just_released {
            app_exit_event_writer.send(AppExit);
            return;
        }
    }
}