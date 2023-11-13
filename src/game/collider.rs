use bevy::prelude::*;

#[derive(Component)]
pub struct BoxCollider {
    pub extends: Vec2,
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self {
            extends: Vec2::new(100., 100.),
        }
    }
}

impl BoxCollider {
    pub fn overlap(box1_center: Vec2, box1_extents: Vec2, box2_center: Vec2, box2_extents: Vec2) -> bool {
        let box1_min = box1_center - box1_extents;
        let box1_max = box1_center + box1_extents;

        let box2_min = box2_center - box2_extents;
        let box2_max = box2_center + box2_extents;

        let x_overlap = box1_min.x <= box2_max.x && box1_max.x >= box2_min.x;
        let y_overlap = box1_min.y <= box2_max.y && box1_max.y >= box2_min.y;

        x_overlap && y_overlap
    }
}