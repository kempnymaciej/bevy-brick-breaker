use bevy::asset::AssetServer;
use bevy::prelude::{BuildChildren, Commands, Component, DespawnRecursiveExt, Entity, Query, Res, With};
use crate::common::styles::{get_full_screen_menu_node_bundle, spawn_full_screen_menu_header};

#[derive(Component)]
pub struct PreparationView;

pub fn spawn_preparation_view(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
    commands.spawn(
        (
            PreparationView {},
            get_full_screen_menu_node_bundle(),
        )
    ).with_children(|parent| {
        spawn_full_screen_menu_header(parent, &asset_server, "Use arrows to move the paddle.");
        spawn_full_screen_menu_header(parent, &asset_server, "Collect squares to earn points.");
        spawn_full_screen_menu_header(parent, &asset_server, "Press any key to start.");
        spawn_full_screen_menu_header(parent, &asset_server, "(Optionally, press esc to pause.)");
    });
}

pub fn despawn_preparation_view(
    mut commands: Commands,
    view_query: Query<Entity, With<PreparationView>>
)
{
    for view in view_query.iter() {
        commands.entity(view).despawn_recursive();
    }
}