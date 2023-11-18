use bevy::prelude::*;
use crate::common::better_button::{ColorButton, ReleaseButton};
use crate::game::events::{MenuRequested, RestartRequested};
use crate::game::settings::Score;

#[derive(Component)]
pub struct SummaryView;
#[derive(Component)]
pub struct RestartButton;
#[derive(Component)]
pub struct MenuButton;

pub fn spawn_summary_view(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
)
{
    commands.spawn(
        (
            SummaryView {},
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(12.),
                    ..default()
                },
                ..default()
            }
        )
    ).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(format!("Your score: {}", score.0), TextStyle {
                font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                font_size: 90.,
                color: Color::WHITE,
            })
        );

        parent.spawn(
            (
                RestartButton {},
                ReleaseButton::default(),
                ColorButton::default(),
                ButtonBundle {
                    image: UiImage {
                        texture: asset_server.load("sprites/buttonDefault.png"),
                        ..default()
                    },
                    style: Style {
                        height: Val::Px(49.),
                        width: Val::Px(190.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                }
            )
        ).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section("Restart", TextStyle {
                    font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                    font_size: 40.,
                    color: Color::BLACK,
                }).with_style(Style {
                    margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(6.)),
                    ..default()
                })
            );
        });

        parent.spawn(
            (
                MenuButton {},
                ReleaseButton::default(),
                ColorButton::default(),
                ButtonBundle {
                    image: UiImage {
                        texture: asset_server.load("sprites/buttonDefault.png"),
                        ..default()
                    },
                    style: Style {
                        height: Val::Px(49.),
                        width: Val::Px(190.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                }
            )
        ).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section("Menu", TextStyle {
                    font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                    font_size: 40.,
                    color: Color::BLACK,
                }).with_style(Style {
                    margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(6.)),
                    ..default()
                })
            );
        });
    });
}

pub fn despawn_summary_view(
    mut commands: Commands,
    view_query: Query<Entity, With<SummaryView>>
)
{
    for view in view_query.iter() {
        commands.entity(view).despawn_recursive();
    }
}

pub fn check_summary_interactions(
    menu_button_query: Query<&ReleaseButton, With<MenuButton>>,
    restart_button_query: Query<&ReleaseButton, With<RestartButton>>,
    mut menu_requested_events: EventWriter<MenuRequested>,
    mut restart_requested_events: EventWriter<RestartRequested>,
)
{
    for button in menu_button_query.iter() {
        if button.just_released {
            menu_requested_events.send_default();
            return;
        }
    }

    for button in restart_button_query.iter() {
        if button.just_released {
            restart_requested_events.send_default();
            return;
        }
    }
}