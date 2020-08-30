use crate::petra;
use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

pub fn setup() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup_terrain.system())
        .add_startup_system(setup_scene.system())
        .add_plugin(FlyCameraPlugin)
        //.add_plugin(petra::camera::CameraPlugin)
        .run();
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrComponents {
        mesh: meshes.add(petra::mesh::generate_terrain_mesh()),
        material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        ..Default::default()
    });
}

fn setup_scene(mut commands: Commands) {
    commands
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .spawn(FlyCamera::default());
}
