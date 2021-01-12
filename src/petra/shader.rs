use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        renderer::RenderResources,
    },
};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "790574b4-849d-4b3f-9d47-376e1cc3f182"]

pub struct TerrainMaterial {
    pub color: Color,
}

pub const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Normal = Vertex_Normal;
    v_Position = Vertex_Position;
    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

void main() {
    float brightness = dot( normalize(v_Normal), vec3(1., 1., 1.) );
    o_Target = vec4( vec3(0.054, 0.341, 0.019) * (brightness*0.25+0.75), 1.0 );
}
"#;