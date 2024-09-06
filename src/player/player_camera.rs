use bevy::{app::{Plugin, PostUpdate, Startup}, math::Quat, prelude::{BuildChildren, Bundle, Camera3dBundle, Commands, Component, PerspectiveProjection, Query, Transform, TransformBundle, With, Without}, utils::default};

use super::PlayerTag;

#[derive(Component, Default)]
pub struct PlayerCameraTag { 
    pub id: u32 
}

#[derive(Bundle, Default)]
pub struct PlayerCameraBundle {
    camera: Camera3dBundle,
    // smoothing component and so on
}
impl PlayerCameraBundle {
    pub fn new() -> Self {
        Self {
            camera: Camera3dBundle {
                projection: bevy::prelude::Projection::Perspective(PerspectiveProjection {
                    fov: 90.0,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 15.0, 7.5)
                    .with_rotation(Quat::from_rotation_x(f32::to_radians(-60.0))),
                ..default()
            },
            ..default()
        }
    } 
}

pub fn create_player_camera_system(
    mut commands: Commands,
) {
    let root = commands.spawn((PlayerCameraTag { id: 0 }, TransformBundle::default())).id();
    let camera = commands.spawn(PlayerCameraBundle::new()).id();

    commands.entity(root).add_child(camera);
}

pub fn follow_player_system(
    mut camera_query: Query<(&PlayerCameraTag, &mut Transform), Without<PlayerTag>>,
    player_query: Query<(&PlayerTag, &Transform), Without<PlayerCameraTag>>,
) {
    for (camera_tag, mut camera_transform) in &mut camera_query {
        for (player_tag, player_transform) in &player_query {
            if camera_tag.id != player_tag.id { continue; }

            camera_transform.translation = player_transform.translation;
        }
    }
}

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_player_camera_system);
        app.add_systems(PostUpdate, follow_player_system);
    }
}