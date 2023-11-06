use bevy::prelude::*;

pub fn get_menu_root_style() -> Style {
    Style {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    }
}

pub fn get_menu_button_style() -> Style {
    Style {
        width: Val::Percent(40.),
        aspect_ratio: Some(190. / 49.),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        margin: UiRect::vertical(Val::Px(20.)),
        ..default()
    }
}