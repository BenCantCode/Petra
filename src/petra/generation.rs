use noise::{Fbm, NoiseFn, Perlin};
use crate::petra::erode;
use crate::petra::terrain::Terrain;
use crate::petra::terrain::TerrainData;

pub const TERRAIN_SIZE: usize = 128;
const TERRAIN_WORLDSCALE: f32 = 256f32;
pub const TERRAIN_HEIGHT: f32 = 50f32;
const TERRAIN_NOISESCALE: f32 = 0.005;

pub fn generate_terrain_data() -> Terrain {
    let fbm = Fbm::new();
    let mut data = TerrainData::zeros(TERRAIN_SIZE);

    for z in 0..TERRAIN_SIZE {
        for x in 0..TERRAIN_SIZE {
            data[(x, z)] = (fbm.get([(x as f64)*0.01, (z as f64)*0.01]) as f32) * TERRAIN_HEIGHT;
        }
    }

    let mut terrain = Terrain {
        data: data,
        size: TERRAIN_SIZE,
        worldscale: TERRAIN_WORLDSCALE,
        height: TERRAIN_HEIGHT,
        noisescale: TERRAIN_NOISESCALE,
    };
    erode::erode(&mut terrain);
    terrain
}
