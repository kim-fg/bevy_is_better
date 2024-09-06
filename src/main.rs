use bevy::prelude::*;
use bevy_is_better::prelude::*;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins((PlayerPlugin, PlayerCameraPlugin, PlayerInputPlugin))
    .add_plugins(DefaultLevelPlugin)
    .run();
}
