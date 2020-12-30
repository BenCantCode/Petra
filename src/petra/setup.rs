use crate::petra;
use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn setup_terrain(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(petra::mesh::generate_terrain_mesh()),
        material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        ..Default::default()
    });
}

fn setup_scene(commands: &mut Commands) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
    commands
        .spawn(Camera3dBundle::default())
        .with(FlyCamera::default());
}

pub fn setup() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_terrain.system())
        .add_startup_system(setup_scene.system())
        .add_plugin(FlyCameraPlugin)
        //.add_plugin(petra::camera::CameraPlugin)
        .run();
}