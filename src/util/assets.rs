use bevy::{asset::AssetServer, prelude::Res, scene::SceneBundle, utils::default};

pub fn load_scene(
    asset_server: &Res<AssetServer>,
    scene_path: &'static str,
) -> SceneBundle {
    let scene_handle = asset_server.load(scene_path);
    
    SceneBundle {
        scene: scene_handle,
        ..default()
    }
}