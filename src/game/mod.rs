use bevy::prelude::*;

mod paddle;
mod ball;
mod brick;

use paddle::PaddlePlugin;
use ball::BallPlugin;
use crate::game::brick::BrickPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PaddlePlugin);
        app.add_plugins(BallPlugin);
        app.add_plugins(BrickPlugin);
    }
}




