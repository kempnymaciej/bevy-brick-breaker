use bevy::prelude::*;

#[derive(Component)]
pub struct BoxCollider {
    pub extends: Vec2,
}


pub fn check_box_overlaps_box(
    center: Vec2,
    extends: Vec2,
    other_center: Vec2,
    other_extends: Vec2,
) -> bool {
    let delta = other_center - center;
    let max_delta = other_extends + extends;
    delta.x.abs() <= max_delta.x && delta.y.abs() <= max_delta.y
}

pub fn check_circle_overlaps_circle(
    center: Vec2,
    radius: f32,
    other_center: Vec2,
    other_radius: f32,
) -> bool {
    (other_center - center).length_squared() <= radius + other_radius
}

pub fn check_box_overlaps_circle(
    box_center: Vec2,
    box_extends: Vec2,
    circle_center: Vec2,
    circle_radius: f32,
) -> bool {
    // Calculate the distance between the centers of the rectangle and the circle
    let centers_delta = circle_center - box_center;

    // Find the point on the rectangle closest to the circle
    let closest = Vec2 {
        x: centers_delta.x.clamp(-box_extends.x, box_extends.x),
        y: centers_delta.y.clamp(-box_extends.y, box_extends.y),
    };

    // Check if the distance between the circle's center and the closest point on the rectangle is less than or equal to the radius
    let distance = (centers_delta.x - closest.x) * (centers_delta.x - closest.x)
        + (centers_delta.y - closest.y) * (centers_delta.y - closest.y);

    distance <= circle_radius
}