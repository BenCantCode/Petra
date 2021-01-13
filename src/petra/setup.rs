use super::shader;
use crate::petra;
use bevy::{math::vec3, pbr::AmbientLight, prelude::*};
use goshawk::{rts_camera_system, RtsCamera, ZoomSettings, PanSettings};
use bevy::{
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{asset_shader_defs_system, ShaderDefs, ShaderStage, ShaderStages},
    },
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_mod_picking::*;

fn setup_terrain(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    terrain: ResMut<petra::terrain::Terrain>
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            shader::VERTEX_SHADER,
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            shader::FRAGMENT_SHADER,
        ))),
    }));

    let terrain_mesh = meshes.add(petra::mesh::generate_mesh(&terrain));

    commands.spawn(MeshBundle {
        mesh: terrain_mesh,
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            pipeline_handle.clone(),
        )]),
        ..Default::default()
    })
    .with(PickableMesh::default());
}

fn setup_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3dBundle {
            //transform: Transform::from_translation(vec3(0.0, 100.0, 0.0)),
            ..Default::default()
        })
        .with(RtsCamera {
            looking_at: Vec3::new(64.0, 0.0, 64.0),
            zoom_distance: 100.0,

            ..Default::default()
        })
        .with(ZoomSettings {
            scroll_accel: 10.0,
            max_velocity: 50.0,
            idle_deceleration: 200.0,
            angle_change_zone: 30.0..=75.0,
            distance_range: 25.0..=500.0,
            ..Default::default()
        })
        .with(PanSettings {
            mouse_accel: 75.0,
            keyboard_accel: 50.0,
            idle_deceleration: 75.0,
            max_speed: 25.0,
            ..Default::default()
        })
        .with(PickSource::default());
}

pub fn setup() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_terrain.system())
        .add_startup_system(setup_scene.system())
        .add_plugin(FlyCameraPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(petra::modify::Modify)
        .add_system(rts_camera_system.system())
        //.add_plugin(InteractablePickingPlugin)
        //.add_plugin(DebugPickingPlugin)
        //.add_plugin(petra::camera::CameraPlugin)
        .run();
}
