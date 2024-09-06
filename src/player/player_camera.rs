use bevy::{asset::LoadState, core_pipeline::Skybox, prelude::*, render::render_resource::{TextureViewDescriptor, TextureViewDimension}};

use super::PlayerTag;

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

#[derive(Component, Default)]
pub struct PlayerCameraTag { 
    pub id: u32 
}

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    camera: Camera3dBundle,
    skybox: Skybox,
    // smoothing component and so on
}
impl PlayerCameraBundle {
    pub fn new(skybox_handle: Handle<Image>) -> Self {
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
            skybox: Skybox {
                image: skybox_handle,
                brightness: 1000.0,
            },
        }
    }
}

fn create_player_camera_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const SKYBOX_PATH: &str = "tyro/skybox/cube.png";
    let skybox_handle = asset_server.load(SKYBOX_PATH);

    let camera = commands.spawn(PlayerCameraBundle::new(skybox_handle.clone())).id();

    let root = commands.spawn((PlayerCameraTag { id: 0 }, TransformBundle::default())).id();

    commands.entity(root).add_child(camera);

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    })
}

fn apply_skybox_system(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skybox_query: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            })
        }

        for mut skybox in &mut skybox_query {
            skybox.image = cubemap.image_handle.clone();
        }
    
        cubemap.is_loaded = true;
    }
}

fn follow_player_system(
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
        app.add_systems(Update, apply_skybox_system);
        app.add_systems(PostUpdate, follow_player_system);
    }
}