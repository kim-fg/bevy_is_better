use bevy::prelude::*;
use bevy_is_better::player::{PlayerCameraPlugin, PlayerInputPlugin, PlayerPlugin};
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins((PlayerPlugin, PlayerCameraPlugin, PlayerInputPlugin))
    .run();
}
