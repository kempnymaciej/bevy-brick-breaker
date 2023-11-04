use bevy::prelude::*;

mod paddle;
mod ball;

use paddle::PaddlePlugin;
use ball::BallPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PaddlePlugin);
        app.add_plugins(BallPlugin);
    }
}




