use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        renderer::RenderResources,
    },
};

pub const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 3) in vec2 Vertex_RealPosition;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec2 v_Uv;
layout(location = 3) out vec2 v_RealPosition;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Normal = Vertex_Normal;
    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    v_RealPosition = Vertex_RealPosition;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;
layout(location = 3) in vec2 v_RealPosition;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform CursorPosition_pos {
    vec2 cursor_pos;
};
layout(set = 2, binding = 1) uniform CursorPosition_hovering {
    int cursor_hovering; // active is reserved
};
layout(set = 2, binding = 2) uniform CursorPosition_radius {
    float cursor_radius;
};

void main() {
    float brightness = dot( normalize(v_Normal), vec3(1., 1., 1.) );
    float cursor_closeness = -distance(v_RealPosition, cursor_pos) + cursor_radius; // >= 0 if in cursor.
    vec3 color = vec3(0.1,0.1,0.4);
    vec3 fill = color * max(cursor_closeness, 0.0)/cursor_radius;
    vec3 outline = color * step(cursor_closeness, 0.0) * step(-1.0, cursor_closeness);
    o_Target = vec4( vec3(0.054, 0.341, 0.019) * (brightness*0.25+0.75) + fill + outline, 1.0 );
}
"#;