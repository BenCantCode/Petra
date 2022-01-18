use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use bevy::render::camera::{CameraProjection, DepthCalculation};
use bevy_mod_picking::{PickingCamera, PickingCameraBundle};

pub struct CameraPlugin;

use bevy::render::camera;
use bevy::render::view::VisibleEntities;

fn startup(mut commands: Commands) {
    // set up the camera
    let mut camera = PerspectiveCameraBundle::new_3d();
    //camera.orthographic_projection.scale = 100.0;
    camera.transform = Transform::from_xyz(-100.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands
        .spawn_bundle(camera)
        .insert_bundle(PickingCameraBundle::default());
        //.insert(FlyCamera::default());
}

fn update(mut query: Query<(&Camera, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>){
    let (camera, mut transform) = query.single_mut();
    if(keyboard_input.pressed(KeyCode::W)){
        let forward = transform.forward(); // "forward" seems to be reversed.
        transform.translation += Vec3::new(1.0, 0.0, 0.0);
    }
    if(keyboard_input.pressed(KeyCode::S)){
        let backward = Vec3::new(-1.0, 0.0, 0.0);
        transform.translation += backward;
    }
    if(keyboard_input.pressed(KeyCode::A)){
        let left = transform.left();
        transform.translation += left;
    }
    if(keyboard_input.pressed(KeyCode::D)){
        let right = transform.right();
        transform.translation += right;
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
        app.add_system(update);
    }
}
