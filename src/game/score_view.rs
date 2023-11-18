use bevy::prelude::*;
use crate::game::settings::Score;

#[derive(Component)]
pub struct ScoreView;
#[derive(Component)]
pub struct ScoreIcon;
#[derive(Component)]
pub struct ScoreIndicator;

// pub fn spawn_score_view(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// )
// {
//     commands.spawn(
//         (
//             NodeBundle {
//                 style: Style {
//                     height: Val::Percent(3.),
//                     align_items: AlignItems::Start,
//                     justify_content: JustifyContent::Start,
//                     flex_direction: FlexDirection::Row,
//                     ..default()
//                 },
//                 //background_color: Color::WHITE.into(),
//                 ..default()
//             },
//             ScoreView{},
//         )
//     ).with_children(|parent| {
//         // parent.spawn(
//         //     (
//         //         ImageBundle {
//         //             image: UiImage {
//         //                 texture: asset_server.load("sprites/collectables/element_blue_square.png"),
//         //                 ..default()
//         //             },
//         //             style: Style {
//         //                 height: Val::Percent(100.),
//         //                 ..default()
//         //             },
//         //             ..default()
//         //         },
//         //         ScoreIcon{},
//         //     )
//         // );
//
//         parent.spawn(
//             (
//                 NodeBundle {
//                     style: Style {
//                         height: Val::Percent(100.),
//                         align_items: AlignItems::Start,
//                         justify_content: JustifyContent::SpaceAround,
//                         ..default()
//                     },
//                     ..default()
//                 },
//                 //WindowHeightRelativeScale::default(),
//             )
//         ).with_children(|parent| {
//             parent.spawn(
//                 (
//                     TextBundle::from_section("x 0", TextStyle {
//                         font: asset_server.load("fonts/OpenSans-Regular.ttf"),
//                         font_size: 30.,
//                         color: Color::BLACK,
//                         ..default()
//                     }).with_style(Style {
//                         //padding: UiRect::horizontal(Val::Percent(10.0)),
//                         ..default()
//                     }).with_background_color(Color::RED),
//                     ScoreIndicator{},
//                     WindowHeightRelativeScale::default(),
//                 )
//             );
//         });
//     });
// }

pub fn despawn_score_view(
    mut commands: Commands,
    view_query: Query<Entity, With<ScoreView>>,
)
{
    for view in view_query.iter() {
        commands.entity(view).despawn_recursive();
    }
}

pub fn update_score_view(
    score: Res<Score>,
    mut indicator_query: Query<&mut Text, With<ScoreIndicator>>,
    mut icon_query: Query<&mut Node, With<ScoreIcon>>,
)
{
    if !score.is_changed() {
        return;
    }

    for mut indicator in indicator_query.iter_mut() {
        indicator.sections[0].value = format!("x {}", score.0);
    }
}

pub fn spawn_score_view(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
    commands.spawn(
        (
            ScoreView {},
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Auto,
                    height: Val::Px(32.),
                    top: Val::Px(12.),
                    left: Val::Px(12.),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.),
                    ..default()
                },
                ..default()
            }
        )
    ).with_children(|parent| {
        parent.spawn(
            (
                ScoreIcon {},
                ImageBundle {
                    style: Style {
                        height: Val::Percent(75.),
                        ..default()
                    },
                    image: UiImage {
                        texture: asset_server.load("sprites/collectables/element_blue_square.png"),
                        ..default()
                    },
                    ..default()
                }
            )
        );
        parent.spawn(
            (
                ScoreIndicator {},
                TextBundle {
                    style: Style {

                        ..default()
                    },
                    text: Text::from_section("x 0", TextStyle {
                        font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                        font_size: 30.,
                        color: Color::BLACK,
                        ..default()
                    }),
                    ..default()
                }
            )
        );
    });
}