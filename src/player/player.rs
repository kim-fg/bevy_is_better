use bevy::{app::{Plugin, Startup, Update}, asset::AssetServer, prelude::{default, Bundle, Commands, Component, Query, Res, Transform}, scene::SceneBundle, time::Time};

use super::PlayerInput;

#[derive(Component, Default)]
pub struct PlayerTag;

#[derive(Component)]
pub struct PlayerMovement {
    pub thrust_power: f32,
    pub brake_power: f32,
    pub turn_speed: f32,
}
impl Default for PlayerMovement {
    fn default() -> Self {
        Self { 
            thrust_power: 10.0, 
            brake_power: 2.5, 
            turn_speed: 120.0, 
        }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    // todo! set up PlayerBundle
    tag: PlayerTag,
    model: SceneBundle,
    input: PlayerInput,
    movement: PlayerMovement,
}

impl PlayerBundle {
    pub fn new(model: SceneBundle) -> Self {
        Self {
            model,
            ..default()
        }
    }
}

fn create_player_system(
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
}

fn player_move_system(
    mut query: Query<(&PlayerInput, &PlayerMovement, &mut Transform)>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();

    for (input, movement, mut transform) in &mut query {
        let thrust = if input.thrust { movement.thrust_power } else { 0.0 }; // todo! add velocity based movement
        let forward = transform.forward();

        transform.translation += forward * thrust * delta;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_player_system);
        app.add_systems(Update, player_move_system);
    }
}