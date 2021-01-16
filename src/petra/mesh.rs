use std::borrow::Cow;

use crate::petra::modify::CursorPosition;
use crate::petra::shader::*;
use crate::petra::terrain::*;
use bevy::{
    math::vec2,
    render::{mesh::Mesh, pipeline::PrimitiveTopology, render_graph::AssetRenderResourcesNode},
};
use bevy::{
    math::vec3,
    reflect::TypeUuid,
    render::{
        camera::Camera,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};
use bevy::{prelude::*, render::shader::asset_shader_defs_system};
use bevy_mod_picking::PickableMesh;

use super::terrain;

pub struct CustomPipeline(Handle<PipelineDescriptor>);
pub struct TerrainMaterial(pub Handle<CursorPosition>);
pub struct ChunkComponent((i32, i32));
pub struct TerrainMeshPlugin;
impl Plugin for TerrainMeshPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<CursorPosition>()
            .add_system_to_stage(
                stage::POST_UPDATE,
                asset_shader_defs_system::<CursorPosition>.system(),
            )
            .add_startup_system(chunk_mesh_system_setup.system())
            .add_system(chunk_mesh_system.system());
    }
}

fn chunk_mesh_system_setup(
    commands: &mut Commands,
    mut shaders: ResMut<Assets<Shader>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut render_graph: ResMut<RenderGraph>,
    mut materials: ResMut<Assets<CursorPosition>>,
) {
    let pipeline_handle = {
        pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
        }))
    };
    render_graph.add_system_node(
        "cursor_position",
        AssetRenderResourcesNode::<CursorPosition>::new(true),
    );

    render_graph
        .add_node_edge("cursor_position", base::node::MAIN_PASS)
        .unwrap();

    let terrain_material = materials.add(CursorPosition {
        pos: vec2(0.0, 0.0),
        radius: 25.0,
        hovering: 0
    });

    commands.insert_resource(CustomPipeline(pipeline_handle));
    commands.insert_resource(TerrainMaterial(terrain_material));

}

const RENDER_DISTANCE: i32 = 1;

fn chunk_mesh_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    chunks_query: Query<(Entity, &ChunkComponent, &Handle<Mesh>, &Transform)>,
    camera_query: Query<(&Camera, &Transform)>,
    mut terrain: ResMut<Terrain>,
    custom_pipeline: Res<CustomPipeline>,
    terrain_material: Res<TerrainMaterial>
) {
    let camera_translation = camera_query.iter().next().unwrap().1.translation;
    let camera_chunk_coordinates = TerrainData::get_terrain_chunk_coordinates((
        camera_translation.x as i32,
        camera_translation.z as i32,
    ));

    for x in (camera_chunk_coordinates.0 - RENDER_DISTANCE)
        ..(camera_chunk_coordinates.0 + RENDER_DISTANCE + 1)
    {
        for z in (camera_chunk_coordinates.1 - RENDER_DISTANCE)
            ..(camera_chunk_coordinates.1 + RENDER_DISTANCE + 1)
        {
            if let Some(e) = chunks_query.iter().find(|e| e.1 .0 == (x, z)) {
                if let Some(chunk) = terrain.data.chunks.get(&(x, z)) {
                    if chunk.modified {
                        meshes.set(e.2, generate_mesh(&terrain, (x, z)));
                        terrain.data.chunks.get_mut(&(x, z)).unwrap().modified = false;
                    }
                }
            } else {
                println!("Spawning mesh! {}, {}", x, z);
                let terrain_mesh = generate_mesh(&terrain, (x, z));
                let terrain_mesh_handle = meshes.add(terrain_mesh);
                commands
                    .spawn(MeshBundle {
                        mesh: terrain_mesh_handle.clone(),
                        render_pipelines: RenderPipelines::from_pipelines(vec![
                            RenderPipeline::new(custom_pipeline.0.clone()),
                        ]),
                        transform: Transform::from_translation(vec3(
                            (x * TerrainDataChunk::size as i32) as f32,
                            0.0,
                            (z * TerrainDataChunk::size as i32) as f32, //inverted
                        )),
                        ..Default::default()
                    })
                    .with(terrain_material.0.clone())
                    .with(PickableMesh::default().with_bounding_sphere(terrain_mesh_handle.clone()))
                    .with(ChunkComponent((x, z)));
            }
        }
    }

    for i in chunks_query.iter() {
        if (i.1 .0 .0 - camera_chunk_coordinates.0).abs() > RENDER_DISTANCE
            || (i.1 .0 .1 - camera_chunk_coordinates.1).abs() > RENDER_DISTANCE
        {
            println!("Deleting mesh!");
            commands.despawn_recursive(i.0);
        }
    }
}

pub fn generate_mesh(terrain: &Terrain, chunk_coordinates: (i32, i32)) -> Mesh {
    let mut positions: Vec<[f32; 3]> =
        Vec::with_capacity((TerrainDataChunk::size + 1) * (TerrainDataChunk::size + 1));
    let mut real_positions: Vec<[f32; 2]> =
        Vec::with_capacity((TerrainDataChunk::size + 1) * (TerrainDataChunk::size + 1));
    let mut normals: Vec<[f32; 3]> =
        Vec::with_capacity((TerrainDataChunk::size + 1) * (TerrainDataChunk::size + 1));
    let mut uvs: Vec<[f32; 2]> =
        Vec::with_capacity((TerrainDataChunk::size + 1) * (TerrainDataChunk::size + 1));
    let mut indices: Vec<u32> =
        Vec::with_capacity((TerrainDataChunk::size + 1) * (TerrainDataChunk::size + 1) * 6);
    let mut index: usize = 0;

    let chunk_real_coordinates = (
        chunk_coordinates.0 * TerrainDataChunk::size as i32,
        chunk_coordinates.1 * TerrainDataChunk::size as i32,
    );

    let empty_chunk = TerrainDataChunk::new(chunk_coordinates);
    let main_chunk: Cow<TerrainDataChunk> = if let Some(i) = terrain
        .data
        .chunks
        .get(&chunk_coordinates) {
            Cow::Borrowed(i)
        }else{
            Cow::Owned(TerrainDataChunk::new(chunk_coordinates))
        };
    let right_chunk: Cow<TerrainDataChunk> = if let Some(i) = terrain
        .data
        .chunks
        .get(&(chunk_coordinates.0 + 1, chunk_coordinates.1)) {
            Cow::Borrowed(i)
        }else{
            Cow::Owned(TerrainDataChunk::new((chunk_coordinates.0 + 1, chunk_coordinates.1)))
        };
    let bottom_chunk: Cow<TerrainDataChunk> = if let Some(i) = terrain
        .data
        .chunks
        .get(&(chunk_coordinates.0, chunk_coordinates.1 + 1)) {
            Cow::Borrowed(i)
        }else{
            Cow::Owned(TerrainDataChunk::new((chunk_coordinates.0, chunk_coordinates.1 + 1)))
        };
    let bottom_right_value = terrain
        .data
        .chunks
        .get(&(chunk_coordinates.0 + 1, chunk_coordinates.1 + 1))
        .and_then(|c| Some(c.data[0]))
        .unwrap_or(0.0);

    for x in 0..TerrainDataChunk::size + 1 {
        for z in 0..TerrainDataChunk::size + 1 {
            if x == TerrainDataChunk::size {
                if z == TerrainDataChunk::size {
                    positions.push([x as f32, bottom_right_value, z as f32]);
                } else {
                    positions.push([
                        x as f32,
                        right_chunk.data[z * TerrainDataChunk::size],
                        z as f32,
                    ]);
                }
            } else {
                if z == TerrainDataChunk::size {
                    positions.push([x as f32, bottom_chunk.data[x], z as f32]);
                } else {
                    positions.push([
                        x as f32,
                        main_chunk.data[z * TerrainDataChunk::size + x],
                        z as f32,
                    ]);
                }
            }
            real_positions.push([(x as i32 + (main_chunk.coords.0 * TerrainDataChunk::size as i32)) as f32, (z as i32 + (main_chunk.coords.1 * TerrainDataChunk::size as i32)) as f32]);

            // Calculate normals
            let up = terrain.data[(
                x as i32 + chunk_real_coordinates.0,
                z as i32 - 1 + chunk_real_coordinates.1,
            )];

            let upright = terrain.data[(
                x as i32 + 1 + chunk_real_coordinates.0,
                z as i32 - 1 + chunk_real_coordinates.1,
            )];
            let right = terrain.data[(
                x as i32 + 1 + chunk_real_coordinates.0,
                z as i32 + chunk_real_coordinates.1,
            )];
            let down = terrain.data[(
                x as i32 + chunk_real_coordinates.0,
                z as i32 + 1 + chunk_real_coordinates.1,
            )];
            let downleft = terrain.data[(
                x as i32 - 1 + chunk_real_coordinates.0,
                z as i32 + 1 + chunk_real_coordinates.1,
            )];
            let left = terrain.data[(
                x as i32 - 1 + chunk_real_coordinates.0,
                z as i32 + chunk_real_coordinates.1,
            )];

            let normal = vec3(
                2.0 * (left - right) - upright + downleft + up - down,
                2.0 * (down - up) + upright + downleft - up - left,
                6.0,
            );
            let normal_normalized = normal.normalize();

            normals.push(normal_normalized.into());
            uvs.push([x as f32, z as f32]);
            if x != TerrainDataChunk::size && z != TerrainDataChunk::size {
                indices.append(&mut vec![
                    index as u32,
                    (index + 1) as u32,
                    (index + TerrainDataChunk::size + 1) as u32, //First triangle. Bevy expects u32.
                    (index + TerrainDataChunk::size + 2) as u32,
                    (index + TerrainDataChunk::size + 1) as u32,
                    (index + 1) as u32, //Second triangle
                ]);
            }

            index += 1;
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute("Vertex_Position", positions);
    mesh.set_attribute("Vertex_Normal", normals);
    mesh.set_attribute("Vertex_Uv", uvs);
    mesh.set_attribute("Vertex_RealPosition", real_positions);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

    mesh
}
