use std::borrow::Cow;

use crate::petra::modify::CursorPosition;
use crate::petra::terrain::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{
    math::vec2,
    render::{mesh::Mesh},
};
use bevy::{
    math::vec3,
};
use bevy::{prelude::*};
use bevy_mod_picking::PickableBundle;

use super::material::TerrainMaterial;

#[derive(Component)]
pub struct ChunkComponent((i32, i32));
pub struct TerrainMeshPlugin;
impl Plugin for TerrainMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(chunk_mesh_system.system());
    }
}

const RENDER_DISTANCE: i32 = 3;

fn find_ray_ground_intersection(pos: Vec3, dir: Vec3) -> Vec2 {
    vec2(dir.x * (pos.y/dir.y) + pos.x, dir.z * (pos.y/dir.y) + pos.z)
}

fn chunk_mesh_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    chunks_query: Query<(Entity, &ChunkComponent, &Handle<Mesh>, &Transform)>,
    camera_query: Query<(&Camera, &Transform)>,
    mut terrain: ResMut<Terrain>,
) {
    let camera_translation = camera_query.iter().next().unwrap().1.translation;
    let camera_direction = camera_query.iter().next().unwrap().1.rotation.mul_vec3(vec3(0.0, 1.0, 0.0));
    let looking_at = find_ray_ground_intersection(camera_translation, camera_direction);
    let camera_chunk_coordinates = TerrainData::get_terrain_chunk_coordinates((
        looking_at.x as i32,
        looking_at.y as i32,
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
                commands.spawn_bundle((
                        terrain_mesh_handle,
                        Transform::from_translation(vec3(
                            (x * TerrainDataChunk::size as i32) as f32,
                            0.0,
                            (z * TerrainDataChunk::size as i32) as f32, //inverted
                        )),
                        GlobalTransform::default(),
                        TerrainMaterial,
                        Visibility::default(),
                        ComputedVisibility::default(),
                    ))
                    .insert_bundle(PickableBundle::default())
                    .insert(ChunkComponent((x, z)));
            }
        }
    }

    for i in chunks_query.iter() {
        if (i.1 .0 .0 - camera_chunk_coordinates.0).abs() > RENDER_DISTANCE
            || (i.1 .0 .1 - camera_chunk_coordinates.1).abs() > RENDER_DISTANCE
        {
            println!("Deleting mesh!");
            commands.entity(i.0).despawn_recursive();
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
    mesh.set_attribute("Vertex_RealPosition", real_positions); // Hack: Use Vertex_Uv to pass in real position.
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

    mesh
}
