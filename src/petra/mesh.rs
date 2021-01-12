use crate::petra::*;
use bevy::render::{
    mesh::{Mesh},
    pipeline::PrimitiveTopology,
};
use bevy::math::{Vec3, vec3};
use rand;

pub fn generate_terrain_mesh() -> Mesh {

    let terrain: terrain::Terrain = generation::generate_terrain_data();

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(terrain.size*terrain.size);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(terrain.size*terrain.size);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(terrain.size*terrain.size);
    let mut indices: Vec<u32> = Vec::with_capacity(terrain.size*terrain.size*6);
    let mut index: usize = 0;

    for x in 0..terrain.size {
        for z in 0..terrain.size {
            positions.push([
                ((x as f32) / (terrain.size as f32)) * terrain.worldscale,
                terrain.data[(x as usize, z as usize)],
                ((z as f32) / (terrain.size as f32)) * terrain.worldscale,
            ]);

            // Calculate normals
            let up = terrain.data.get_safe(x, z, 0, 1).unwrap_or(0.0);
            
            let upright = terrain.data.get_safe(x, z, 1, -1).unwrap_or(0.0);
            let right = terrain.data.get_safe(x, z, 1, -1).unwrap_or(0.0);
            let down = terrain.data.get_safe(x, z, 0, 1).unwrap_or(0.0);
            let downleft = terrain.data.get_safe(x, z, -1, 1).unwrap_or(0.0);
            let left = terrain.data.get_safe(x, z, -1, 0).unwrap_or(0.0);
            let normal = vec3(
                2.0*(left-right) - upright + downleft + up - down,
                2.0*(down-up) + upright + downleft - up - left,
                6.0
            );
            let normal_normalized = normal.normalize();
            
            normals.push(normal_normalized.into());
            uvs.push([x as f32, z as f32]);

            if x != terrain.size - 1 && z != terrain.size - 1 {
                indices.append(&mut vec![
                    index as u32,
                    (index + 1) as u32,
                    (index + terrain.size) as u32, //First triangle. Bevy expects u32.
                    (index + terrain.size + 1) as u32,
                    (index + terrain.size) as u32,
                    (index + 1) as u32, //Second triangle
                ])
            }

            index += 1;
        }
    }

    let mut terrain_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    terrain_mesh.set_attribute("Vertex_Position", positions);
    terrain_mesh.set_attribute("Vertex_Normal", normals);
    terrain_mesh.set_attribute("Vertex_Uv", uvs);
    terrain_mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    terrain_mesh
}
