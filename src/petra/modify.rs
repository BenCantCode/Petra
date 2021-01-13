pub struct Modify;
use bevy::{math::vec2, prelude::*};
use bevy_mod_picking::{Group, PickState};
use tools::{raise, erode};
use super::{terrain, tools, mesh};

impl Plugin for Modify {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(terrain::Terrain::default())
            .add_resource(SelectedTool(Tool::Raise))
            .add_system(modify_system.system());
    }
}

enum Tool {
    Erode,
    Raise
}
struct SelectedTool(Tool);
fn modify_system(mut terrain: ResMut<terrain::Terrain>, pick_state: Res<PickState>, mut meshes: ResMut<Assets<Mesh>>, query: Query<(&Handle<Mesh>, )>, mouse_input: Res<Input<MouseButton>>, mut selected_tool: ResMut<SelectedTool>) {
    if mouse_input.pressed(MouseButton::Left) {
        if let Some(pick) = pick_state.top(Group::default()) {
            let position = pick.1.position();
            match selected_tool.0 {
                Tool::Raise => {raise::trigger(vec2(position.x, position.z), 25, &mut terrain);},
                Tool::Erode => {erode::trigger(vec2(position.x, position.z), 25, &mut terrain);},
            }
            
        }
    }
    if mouse_input.just_pressed(MouseButton::Right){
        match selected_tool.0 {
            Tool::Raise => {selected_tool.0 = Tool::Erode},
            Tool::Erode => {selected_tool.0 = Tool::Raise}
        }
    }
}