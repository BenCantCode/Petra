use crate::petra::generation;
use crate::petra::terrain::Terrain;
use bevy::math::vec2;
use image::{ImageBuffer, Luma, Rgb};
use rand::random;
use std::f32;
use std::usize;

use super::terrain;

const MAX_ITERATIONS: u32 = 256;
const NUM_DROPLETS: u32 = 100000;
const MAX_CARRIED_SEDIMENT: f32 = 1.0;
const EROSION_RATE: f32 = 0.12;
const DEPOSITION_RATE: f32 = 0.2;
const DRY_TRESHOLD: f32 = 0.08;

pub fn erode(terrain: &mut Terrain) {
    //Save original heightmap
    let mut original_heightmap_img = ImageBuffer::new(
        generation::TERRAIN_SIZE as u32,
        generation::TERRAIN_SIZE as u32,
    );
    for x in 0..(terrain.size as u32) {
        for y in 0..(terrain.size as u32) {
            original_heightmap_img.put_pixel(
                x,
                y,
                Luma([(terrain.data[(x as usize, y as usize)] + 128.0) as u8]),
            )
        }
    }
    original_heightmap_img.save("heightmap.bmp").unwrap();

    for drop in 0..NUM_DROPLETS {
        let mut xy = vec2(
            random::<f32>() * generation::TERRAIN_SIZE as f32,
            random::<f32>() * generation::TERRAIN_SIZE as f32,
        );
        let mut sediment: f32 = 0.0;
        for step in 0..MAX_ITERATIONS {
            let slope_vector_option = terrain.data.get_slope_vector(xy);
            if slope_vector_option.is_some() {
                let slope_vector = slope_vector_option.unwrap();
                let new_xy = slope_vector + xy;
                if new_xy.x < 0.0
                    || new_xy.x.ceil() as usize > terrain.size - 1
                    || new_xy.y < 0.0
                    || new_xy.y.ceil() as usize > terrain.size - 1
                {
                    break;
                }
                let height_difference =
                    terrain.data.sample(new_xy).unwrap() - terrain.data.sample(xy).unwrap();
                if height_difference > 0.0 {
                    // Deposit the carried sediment.
                    terrain.data.modify(xy, sediment.min(DEPOSITION_RATE));
                    sediment -= sediment.min(DEPOSITION_RATE);
                    if (sediment <= DRY_TRESHOLD) {
                        break;
                    }
                } else {
                    xy = new_xy;
                    if sediment < MAX_CARRIED_SEDIMENT {
                        terrain.data.modify(xy, -EROSION_RATE);
                        sediment += EROSION_RATE;
                    }
                }
            } else {
                break;
            }
        }
    }
    let mut new_heightmap_img = ImageBuffer::new(
        generation::TERRAIN_SIZE as u32,
        generation::TERRAIN_SIZE as u32,
    );
    for x in 0..(generation::TERRAIN_SIZE as u32) {
        for y in 0..(generation::TERRAIN_SIZE as u32) {
            new_heightmap_img.put_pixel(
                x,
                y,
                Luma([(terrain.data[(x as usize, y as usize)] + 128.0) as u8]),
            )
        }
    }
    new_heightmap_img.save("new_heightmap.bmp").unwrap();
}
