pub struct Modify;
use super::{material::TerrainMaterial, terrain, tools};
use bevy::{input::mouse::{MouseButtonInput, MouseWheel}, math::vec2, prelude::*, reflect::TypeUuid};
use bevy_mod_picking::PickingCamera;
use tools::{erode, raise};
use bevy_egui::EguiContext;

impl Plugin for Modify {
    fn build(&self, app: &mut App) {
        app.insert_resource(terrain::Terrain::default())
            .insert_resource(SelectedTool(Tool::Raise))
            .insert_resource(CursorPosition {pos: Vec2::new(0.0, 0.0), radius: 10.0, hovering: 1})
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
    pub radius: f32,
    pub hovering: u32, // bool isn't supported. u8 has an error with "copy buffer alignment"
}

fn modify_system(
    mut terrain: ResMut<terrain::Terrain>,
    camera: Query<&PickingCamera>,
    mouse_input: ResMut<Input<MouseButton>>,
    mut selected_tool: ResMut<SelectedTool>,
    mut cursor_position: ResMut<CursorPosition>,
    egui_ctx: Res<EguiContext>
) {
    if !egui_ctx.ctx().wants_pointer_input() {
        if let Some(pick) = camera.iter().next().unwrap().intersect_top() {
            let pick_pos = pick.1.position();
            cursor_position.hovering = 1;
            cursor_position.pos = vec2(pick_pos.x, pick_pos.z);
            if mouse_input.pressed(MouseButton::Left) {
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
            }
        } else {
            cursor_position.hovering = 0;
        }
        if mouse_input.just_pressed(MouseButton::Right) {
            match selected_tool.0 {
                Tool::Raise => selected_tool.0 = Tool::Erode,
                Tool::Erode => selected_tool.0 = Tool::Raise,
            }
        }
    }
}