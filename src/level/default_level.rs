use bevy::{app::{Plugin, Startup}, asset::AssetServer, math::Vec3, prelude::{Commands, Res, Transform}};

use crate::util::load_scene;

use super::StaticGeometryBundle;

fn create_default_level_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const METEOR_MODEL_PATH: &str = "kenney_space-kit/Models/GLTF format/meteor_detailed.glb#Scene0";
    let meteor_model = load_scene(&asset_server, METEOR_MODEL_PATH);
    
    let meteor_positions = vec![
        Vec3::new(-11.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -7.5),
        Vec3::new(14.5, 0.0, 5.0),
        Vec3::new(-1.0, 0.0, 12.0),
        Vec3::new(-10.5, 0.0, -8.0),
        Vec3::new(18.5, 0.0, -13.0),
        Vec3::new(-0.5, 0.0, -26.0),
        Vec3::new(-8.0, 0.0, 10.0),
    ];

    for pos in meteor_positions {
        commands.spawn(StaticGeometryBundle::new(
            meteor_model.clone(), 
            Transform::from_translation(pos),
        ));
    }
}

pub struct DefaultLevelPlugin;

impl Plugin for DefaultLevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_default_level_system);
    }
}