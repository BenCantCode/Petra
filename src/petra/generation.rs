use noise::{Fbm, NoiseFn};

pub struct Terrain {
    pub data: Vec<Vec<f32>>,
    pub size: usize,
    pub worldscale: f32,
    pub height: f32,
    pub noisescale: f32,
}

pub fn generate_terrain_data() -> Terrain {
    const TERRAIN_SIZE: usize = 1024;
    const TERRAIN_WORLDSCALE: f32 = 256f32;
    const TERRAIN_HEIGHT: f32 = 50f32;
    const TERRAIN_NOISESCALE: f32 = 0.005;
    let fbm = Fbm::new();
    let mut data: Vec<Vec<f32>> = Vec::with_capacity(TERRAIN_SIZE);

    for x in 0..TERRAIN_SIZE {
        data.push(Vec::new());
        for z in 0..TERRAIN_SIZE {
            data[x as usize].push(
                ((fbm.get([
                    (((x as f32) / TERRAIN_SIZE as f32) * TERRAIN_WORLDSCALE * TERRAIN_NOISESCALE)
                        as f64,
                    (((z as f32) / TERRAIN_SIZE as f32) * TERRAIN_WORLDSCALE * TERRAIN_NOISESCALE)
                        as f64,
                    0f64,
                ]) as f64)
                    * TERRAIN_HEIGHT as f64) as f32,
            );
        }
    }

    Terrain {
        data: data,
        size: TERRAIN_SIZE,
        worldscale: TERRAIN_WORLDSCALE,
        height: TERRAIN_HEIGHT,
        noisescale: TERRAIN_NOISESCALE,
    }
}
