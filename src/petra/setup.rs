use crate::petra;
use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_mod_picking::*;

fn setup_terrain(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(petra::mesh::generate_terrain_mesh()),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            ..Default::default()
        })
        .with(PickableMesh::default());
}

fn setup_scene(commands: &mut Commands) {
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 64.0, 4.0)),
        ..Default::default()
    });
    commands
        .spawn(Camera3dBundle::default())
        .with(FlyCamera::default())
        .with(PickSource::default());
}

pub fn setup() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_terrain.system())
        .add_startup_system(setup_scene.system())
        .add_plugin(FlyCameraPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(DebugPickingPlugin)
        //.add_plugin(petra::camera::CameraPlugin)
        .run();
}
