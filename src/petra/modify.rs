pub struct Modify;
use super::{material::TerrainMaterial, terrain, tools};
use bevy::{
    input::mouse::{MouseButtonInput, MouseWheel},
    math::{vec2, vec3},
    prelude::*,
    reflect::TypeUuid,
};
use bevy_egui::EguiContext;
use bevy_mod_picking::{PickingCamera, Primitive3d};
use tools::{erode, raise};

impl Plugin for Modify {
    fn build(&self, app: &mut App) {
        app.insert_resource(terrain::Terrain::default())
            .insert_resource(SelectedTool(Tool::Raise))
            .insert_resource(CursorPosition {
                pos: Vec2::new(0.0, 0.0),
                plane_pos: vec3(0.0, 0.0, 0.0),
                radius: 10.0,
                hovering: 1,
            })
            .add_system(modify_system.system());
    }
}

pub enum Tool {
    Erode,
    Raise,
}
pub struct SelectedTool(pub Tool);

#[derive(Default, TypeUuid, Clone, Copy)]
#[uuid = "080ca54b-8c80-4aa5-891d-4c0cbcd0937d"]
#[repr(C)]
pub struct CursorPosition {
    pub pos: Vec2,
    pub plane_pos: Vec3,
    pub radius: f32,
    pub hovering: u32, // bool isn't supported. u8 has an error with "copy buffer alignment"
}

fn modify_system(
    mut terrain: ResMut<terrain::Terrain>,
    camera: Query<&PickingCamera>,
    mouse_input: ResMut<Input<MouseButton>>,
    mut selected_tool: ResMut<SelectedTool>,
    mut cursor_position: ResMut<CursorPosition>,
    egui_ctx: Res<EguiContext>,
) {
    if !egui_ctx.ctx().wants_pointer_input() {
        let cast_source = camera.iter().next().unwrap();

        if mouse_input.just_pressed(MouseButton::Left) {
            if let Some(intersection_result) = cast_source.intersect_top() {
                cursor_position.plane_pos = intersection_result.1.position();
            }
        }

        if mouse_input.just_pressed(MouseButton::Right) {
            match selected_tool.0 {
                Tool::Raise => selected_tool.0 = Tool::Erode,
                Tool::Erode => selected_tool.0 = Tool::Raise,
            }
        }
        if mouse_input.pressed(MouseButton::Left) {
            let pick_pos = cast_source
                .intersect_primitive(Primitive3d::Plane {
                    point: cursor_position.plane_pos,
                    normal: vec3(0.0, 1.0, 0.0),
                })
                .unwrap()
                .position();
            cursor_position.pos = vec2(pick_pos.x, pick_pos.z);
            match selected_tool.0 {
                Tool::Raise => {
                    raise::trigger(
                        vec2(cursor_position.pos.x, cursor_position.pos.y),
                        cursor_position.radius as i64,
                        &mut terrain,
                    );
                }
                Tool::Erode => {
                    erode::trigger(
                        vec2(cursor_position.pos.x, cursor_position.pos.y),
                        25,
                        &mut terrain,
                    );
                }
            }
        } else {
            if let Some(intersection_result) = cast_source.intersect_top() {
                let intersection_pos = intersection_result.1.position();
                cursor_position.pos = vec2(intersection_pos.x, intersection_pos.z);
            }
        }
    }
}
