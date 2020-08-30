use bevy_render::{
    mesh::{Mesh, VertexAttribute},
    pipeline::PrimitiveTopology,
};
use crate::regionem::generation;

pub fn generate_terrain_mesh() -> Mesh{
    let mut positions: Vec<[f32;3]> = Vec::new();
    let mut normals: Vec<[f32;3]> = Vec::new();
    let mut uvs: Vec<[f32;2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut index: usize = 0;

    let terrain: generation::Terrain = generation::generate_terrain_data();

    for x in 0..terrain.size{
        for z in 0..terrain.size {
            positions.push([
                ((x as f32)/(terrain.size as f32))*terrain.worldscale, 
                terrain.data[x as usize][z as usize], 
                ((z as f32)/(terrain.size as f32))*terrain.worldscale]);
            normals.push([0f32,1f32,0f32]);
            uvs.push([x as f32, z as f32]);

            if x != terrain.size-1 && z != terrain.size-1 {
                indices.append(&mut vec![
                    index as u32, (index+1) as u32, (index+terrain.size) as u32, //First triangle. Bevy expects u32.
                    (index+terrain.size+1) as u32, (index+terrain.size) as u32, (index+1) as u32 //Second triangle
                ])
            }

            index += 1;
        }
    }

    Mesh {
        primitive_topology: PrimitiveTopology::TriangleList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs),
        ],
        indices: Some(indices),
    }
}