/*use bevy::prelude::*;

pub struct CameraPlugin;

struct MouseLoc(Vec2);

#[derive(Default)]
struct State {
    event_reader: EventReader<CursorMoved>,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<State>()
            .add_resource(MouseLoc(Vec2::new(0.0, 0.0)))
            .add_startup_system(start_camera.system())
            .add_system(update_mouse.system())
            .add_system(update_camera.system());
    }
}

pub fn start_camera(mut commands: Commands) {
    commands.spawn(Camera3dComponents {
        transform: Transform::new_sync_disabled(Mat4::face_toward(
            Vec3::new(-3.0, 5.0, 8.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )),
        ..Default::default()
    });
}

fn update_mouse(
    mut state: ResMut<State>,
    mut mouse_location: ResMut<MouseLoc>,
    cursor_move_events: Res<Events<CursorMoved>>,
) {
    for event in state.event_reader.iter(&cursor_move_events) {
        mouse_location.0 = event.position;
        println!("{}", event.position);
    }
}

fn update_camera(mouse_position: Res<MouseLoc>, mut query: Query<&mut Camera3dComponents>) {
    for mut camera in &mut query.iter() {
        camera.rotation =
            Rotation::from_rotation_xyz(mouse_position.0.x(), mouse_position.0.y(), 0f32)
    }
}
*/