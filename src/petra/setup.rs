use super::shader;
use crate::petra;
use bevy::{pbr::AmbientLight, prelude::*};
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
use shader::TerrainMaterial;

fn setup_terrain(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
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

    commands.spawn(MeshBundle {
        mesh: meshes.add(petra::mesh::generate_terrain_mesh()),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            pipeline_handle.clone(),
        )]),
        ..Default::default()
    });
    //.with(PickableMesh::default());
}

fn setup_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(64.0, 64.0, 64.0)),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands
        .spawn(Camera3dBundle::default())
        .with(FlyCamera::default());
    //.with(PickSource::default());
}

pub fn setup() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_terrain.system())
        .add_startup_system(setup_scene.system())
        .add_plugin(FlyCameraPlugin)
        //.add_plugin(PickingPlugin)
        //.add_plugin(InteractablePickingPlugin)
        //.add_plugin(DebugPickingPlugin)
        //.add_plugin(petra::camera::CameraPlugin)
        .run();
}
