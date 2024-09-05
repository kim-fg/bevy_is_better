use bevy::prelude::*;
use bevy_is_better::player::PlayerPlugin;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PlayerPlugin)
    .run();
}
