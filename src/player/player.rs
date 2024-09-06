use bevy::{app::{Plugin, Startup, Update}, asset::AssetServer, prelude::{default, Bundle, Commands, Component, Query, Res, Transform}, scene::SceneBundle, time::Time};

use crate::util;

use super::PlayerInput;

#[derive(Component, Default)]
pub struct PlayerTag { pub id: u32 }

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
    pub const MODEL_PATH: &str = "kenney_space-kit/Models/GLTF format/craft_racer.glb#Scene0";

    pub fn new(id: u32, model: SceneBundle) -> Self {
        Self {
            tag: PlayerTag { id },
            model,
            ..default()
        }
    }
}

fn create_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let scene_bundle = util::load_scene(&asset_server, PlayerBundle::MODEL_PATH);

    commands.spawn(PlayerBundle::new(0, scene_bundle));
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