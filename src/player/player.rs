use bevy::{app::{Plugin, Startup}, asset::AssetServer, prelude::{default, Bundle, Commands, Component, Res}, scene::SceneBundle};

use crate::player::PlayerCameraBundle;

#[derive(Component, Default)]
pub struct PlayerTag;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    // todo! set up PlayerBundle
    tag: PlayerTag,
    model: SceneBundle,
}

impl PlayerBundle {
    pub fn new(model: SceneBundle) -> Self {
        Self {
            model,
            ..default()
        }
    }
}

pub fn create_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const MODEL_PATH: &str = "kenney_space-kit/Models/GLTF format/craft_racer.glb#Scene0";

    let scene_handle = asset_server.load(MODEL_PATH);
    let scene_bundle = SceneBundle {
        scene: scene_handle,
        ..default()
    };

    commands.spawn(PlayerBundle::new(scene_bundle));
    commands.spawn(PlayerCameraBundle::new());
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_player_system);
    }
}