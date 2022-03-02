#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

[[group(1), binding(0)]]
var<uniform> mesh: Mesh;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
    [[location(3)]] real_position: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
    [[location(1)]] real_position: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    out.normal = vertex.normal;
    out.real_position = vertex.real_position;
    return out;
}

struct Cursor {
    x: f32;
    y: f32;
    radius: f32;
    hovering: u32;
};

[[group(2), binding(0)]]
var<uniform> cursor: Cursor;

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var cursor_position = vec2<f32>(cursor.x, cursor.y);
    var brightness = dot(normalize(in.normal), vec3<f32>(1.0));
    var cursor_closeness = -distance(in.real_position, cursor_position) + cursor.radius;
    var color = vec3<f32>(0.1, 0.1, 0.4);
    var fill = vec3<f32>(color * max(cursor_closeness, 0.5))/cursor.radius;
    var outline = color * step(cursor_closeness, 0.0) * step(-1.0, cursor_closeness);
    return vec4<f32>(vec3<f32>(0.054, 0.341, 0.019) * (brightness*0.25+0.75) + fill + outline, 1.0 );
}