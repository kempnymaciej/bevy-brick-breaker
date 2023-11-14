use bevy::app::App;
use bevy::prelude::{Component, DetectChanges, EventReader, Plugin, Query, Res, Resource, Transform, Update, Vec3, Window, With};
use bevy::window::{PrimaryWindow, WindowResized};

pub struct WindowHeightRelativeScalePlugin;

impl Plugin for WindowHeightRelativeScalePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WindowHeightRelativeScaleReference>()
            .add_systems(Update, (track_resolution, update_sizes));
    }
}

#[derive(Component)]
pub struct WindowHeightRelativeScale {
    scale: Vec3,
    dirty: bool,
}

impl WindowHeightRelativeScale {
    pub fn new(scale: Vec3) -> Self {
        Self {
            scale,
            dirty: true,
        }
    }

    pub fn get_scale(&self) -> Vec3 {
        self.scale
    }

    pub fn set_size(&mut self, scale: Vec3) {
        if self.scale == scale {
            return;
        }

        self.scale = scale;
        self.dirty = true;
    }
}

impl Default for WindowHeightRelativeScale {
    fn default() -> Self {
        Self::new(Vec3::ONE)
    }
}

#[derive(Resource)]
pub struct WindowHeightRelativeScaleReference {
    pub reference_height: f32,
}

impl Default for WindowHeightRelativeScaleReference {
    fn default() -> Self {
        Self {
            reference_height: 1080.,
        }
    }
}

fn track_resolution(
    mut scale_query: Query<(&mut WindowHeightRelativeScale, &mut Transform)>,
    mut window_resized_events: EventReader<WindowResized>,
    reference: Option<Res<WindowHeightRelativeScaleReference>>,
)
{
    let mut last_resize_event: Option<&WindowResized> = None;
    for window_resized_event in window_resized_events.read() {
        last_resize_event = Some(window_resized_event);
    }

    let Some(last_resize_event) = last_resize_event else {
        return;
    };
    let Some(reference) = reference else {
        return;
    };

    let scale_factor = last_resize_event.height / reference.reference_height;
    for (mut window_related_scale, mut transform) in scale_query.iter_mut() {
        window_related_scale.dirty = false;
        transform.scale = scale_factor * window_related_scale.scale;
    }
}

fn update_sizes(
    window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut scale_query: Query<(&mut WindowHeightRelativeScale, &mut Transform)>,
    reference: Option<Res<WindowHeightRelativeScaleReference>>,
)
{
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Some(reference) = reference else {
        return;
    };

    let force = reference.is_changed();
    let scale_factor = window.resolution.height() / reference.reference_height;

    if force {
        for (mut window_related_scale, mut transform) in scale_query.iter_mut() {
            window_related_scale.dirty = false;
            transform.scale *= scale_factor * window_related_scale.scale;
        }
    }
    else {
        for (mut window_related_scale, mut transform) in scale_query.iter_mut() {
            if window_related_scale.dirty {
                window_related_scale.dirty = false;
                transform.scale *= scale_factor * window_related_scale.scale;
            }
        }
    }
}