use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;

pub struct CameraPlugin;

fn startup(mut commands: Commands) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 100.0;
    camera.transform = Transform::from_xyz(-500.0, 500.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands
        .spawn_bundle(camera)
        .insert_bundle(PickingCameraBundle::default());
}

fn update(mut query: Query<(&Camera, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>) {
    let (_camera, mut transform) = query.single_mut();
    if keyboard_input.pressed(KeyCode::W) {
        let _forward = transform.forward(); // "forward" seems to be reversed.
        transform.translation += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        let backward = Vec3::new(-1.0, 0.0, 0.0);
        transform.translation += backward;
    }
    if keyboard_input.pressed(KeyCode::A) {
        let left = transform.left();
        transform.translation += left;
    }
    if keyboard_input.pressed(KeyCode::D) {
        let right = transform.right();
        transform.translation += right;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        transform.rotate(Quat::from_rotation_y(0.02));
    }
    if keyboard_input.pressed(KeyCode::E) {
        transform.rotate(Quat::from_rotation_y(-0.02));
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
        app.add_system(update);
    }
}
