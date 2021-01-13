pub struct Modify;
use bevy::{math::vec2, prelude::*};
use bevy_mod_picking::{Group, PickState};
use tools::{raise, erode};
use super::{terrain, tools, mesh};

impl Plugin for Modify {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(terrain::Terrain::default())
            .add_system(modify_system.system());
    }
}

enum Tool {
    Erode,
    Raise
}
const selected_tool: Tool = Tool::Raise;
fn modify_system(mut terrain: ResMut<terrain::Terrain>, pick_state: Res<PickState>, mut meshes: ResMut<Assets<Mesh>>, query: Query<&Handle<Mesh>>, mouse_input: Res<Input<MouseButton>>) {
    if mouse_input.pressed(MouseButton::Left) {
        if let Some(pick) = pick_state.top(Group::default()) {
            let position = pick.1.position();
            erode::trigger(vec2(position.x, position.z), 25, &mut terrain);
            for mesh in query.iter() {
                meshes.set(mesh, mesh::generate_mesh(&terrain));
            }
        }
    }
}