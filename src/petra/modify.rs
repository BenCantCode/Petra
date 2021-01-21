pub struct Modify;
use super::{mesh::TerrainMaterial, terrain, tools};
use bevy::{input::mouse::{MouseButtonInput, MouseWheel}, math::vec2, prelude::*, reflect::TypeUuid, render::{renderer::RenderResources, shader::ShaderDefs}};
use bevy_mod_picking::{Group, InteractableMesh, PickState};
use tools::{erode, raise};
use bevy_egui::EguiContext;

impl Plugin for Modify {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(terrain::Terrain::default())
            .add_resource(SelectedTool(Tool::Raise))
            .add_system(modify_system.system());
    }
}

pub enum Tool {
    Erode,
    Raise,
}
pub struct SelectedTool(pub Tool);

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "080ca54b-8c80-4aa5-891d-4c0cbcd0937d"]
pub struct CursorPosition {
    pub pos: Vec2,
    pub radius: f32,
    pub hovering: u32, // bool isn't supported. u8 has an error with "copy buffer alignment"
}

fn modify_system(
    mut terrain: ResMut<terrain::Terrain>,
    pick_state: Res<PickState>,
    mouse_input: ResMut<Input<MouseButton>>,
    mut selected_tool: ResMut<SelectedTool>,
    mut materials: ResMut<Assets<CursorPosition>>,
    terrain_material: Res<TerrainMaterial>,
    egui_ctx: Res<EguiContext>
) {
    if !egui_ctx.ctx.wants_mouse_input() {
        let cursor_position = materials.get_mut(terrain_material.0.clone_weak()).unwrap();
        if let Some(pick) = pick_state.top(Group::default()) {
            let pick_pos = pick.1.position();
            cursor_position.hovering = 1;
            cursor_position.pos = vec2(pick_pos.x, pick_pos.z);
            if mouse_input.pressed(MouseButton::Left) {
                match selected_tool.0 {
                    Tool::Raise => {
                        raise::trigger(
                            vec2(cursor_position.pos.x, cursor_position.pos.y),
                            25,
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