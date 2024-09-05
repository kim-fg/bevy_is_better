use bevy::{app::{Plugin, Startup}, math::Quat, prelude::{Bundle, Camera3dBundle, Commands, PerspectiveProjection, Transform}, utils::default};

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    camera: Camera3dBundle,
    // smoothing component and so on
}
impl PlayerCameraBundle {
    pub fn new() -> Self {
        Self {
            camera: Camera3dBundle {
                projection: bevy::prelude::Projection::Perspective(PerspectiveProjection {
                    fov: 120.0,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 10.0, 5.0)
                    .with_rotation(Quat::from_rotation_x(f32::to_radians(-60.0))),
                ..default()
            },
        }
    } 
}

pub fn create_player_camera_system(
    mut commands: Commands,
) {
    commands.spawn(PlayerCameraBundle::new());
}

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_player_camera_system);
    }
}