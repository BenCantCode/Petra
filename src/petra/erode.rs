use crate::petra::generation;
use crate::petra::modify::{get_subpixel_value, modify_data};
use crate::petra::terrain::Terrain;
use image::{ImageBuffer, Luma, Rgb};
use rand::random;
use std::f32;
use std::usize;

use super::terrain;

/*

The following is my implementation of a "terrain gradient" calculator.
This will output 3 images:

heightmap.bmp - the raw generated terrain from an fBm function to use as example terrain.
angle.bmp - the angles of the slope, with the angle represented as hue (because hue is circular).
steep.bmp - a map of how steep each pixel is.

This was designed in such a way to be used for particle simulations rather
than generating images, which is why I'm doing subpixel interpolation.
Regardless, it looks pretty cool when graphed as an image too.
*/

//Takes point and value map, returns downhill vector
fn get_slope_vector(x: f32, y: f32, data: &Vec<Vec<f32>>, len: usize) -> Option<(f32, f32)> {
    // To get the angle, we "pull" the point towards each side based on the values of each side subpixel.
    let base_value: f32 = get_subpixel_value(x, y, data, len)?;
    let left_value: f32 = get_subpixel_value(x - 1., y, data, len)? - base_value;
    let right_value: f32 = get_subpixel_value(x + 1., y, data, len)? - base_value;
    let up_value: f32 = get_subpixel_value(x, y - 1., data, len)? - base_value;
    let down_value: f32 = get_subpixel_value(x, y + 1., data, len)? - base_value;

    let x_weighted: f32 = left_value * 1f32 + right_value * -1f32; //Weights are inverted because it goes downhill
    let y_weighted: f32 = up_value * 1f32 + down_value * -1f32;

    let angle: f32;
    if x_weighted != 0.0 {
        //The slope will only be attainable if there is a "run"
        angle = y_weighted.atan2(x_weighted);
    } else {
        if y_weighted >= 0.0 {
            angle = f32::consts::PI / 2.;
        } else if y_weighted <= 0.0 {
            angle = f32::consts::PI * 1.5;
        } else {
            angle = random::<f32>() * 2.0 * f32::consts::PI; // Random angle if there's no clear slope.
        }
    }

    let magnitude = get_distance(0., 0., x_weighted, y_weighted); // This will be biased towards "cardinal" directions.

    return Some((angle, magnitude));
}

// Simple pythagorean theorem based distance measure.
fn get_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    return ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
}

//Currently unused. Will be used when implementing actual particle-based erosion.
fn offset_vector(xy: (f32, f32), vector: (f32, f32)) -> (f32, f32) {
    return (
        xy.0 + vector.0.cos() /* * vector.1*/,
        xy.1 + vector.0.sin() /* * vector.1*/,
    );
}

const MAX_ITERATIONS: u32 = 2048;
const NUM_DROPLETS: u32 = 2000000;
const MAX_CARRIED_SEDIMENT: f32 = 0.5;
const EROSION_RATE: f32 = 0.05;

pub fn erode(terrain: Terrain) -> Terrain {
    //Save original heightmap
    let mut original_heightmap_img = ImageBuffer::new(
        generation::TERRAIN_SIZE as u32,
        generation::TERRAIN_SIZE as u32,
    );
    for x in 0..(generation::TERRAIN_SIZE as u32) {
        for y in 0..(generation::TERRAIN_SIZE as u32) {
            original_heightmap_img.put_pixel(
                x,
                y,
                Luma([(terrain.data[y as usize][x as usize] + 128.0) as u8]),
            )
        }
    }
    original_heightmap_img.save("heightmap.bmp").unwrap();

    let mut data = terrain.data;
    for drop in 0..NUM_DROPLETS {
        let mut x = random::<f32>() * generation::TERRAIN_SIZE as f32;
        let mut y = random::<f32>() * generation::TERRAIN_SIZE as f32;
        let mut sediment: f32 = 0.0;
        for step in 0..MAX_ITERATIONS {
            let slope_vector_option = get_slope_vector(x, y, &data, generation::TERRAIN_SIZE);
            if slope_vector_option.is_some() {
                let slope_vector = slope_vector_option.unwrap();
                let new_xy = offset_vector((x, y), slope_vector);
                if new_xy.0 < 0.0
                    || new_xy.0.ceil() as usize > terrain.size - 1
                    || new_xy.1 < 0.0
                    || new_xy.1.ceil() as usize > terrain.size - 1
                {
                    break;
                }
                let height_difference = get_subpixel_value(new_xy.0, new_xy.1, &data, terrain.size)
                    .unwrap()
                    - get_subpixel_value(x, y, &data, terrain.size).unwrap();
                if height_difference > 0.0 {
                    // Deposit the carried sediment.
                    modify_data(x, y, sediment.min(height_difference), &mut data);
                    break;
                } else {
                    x = new_xy.0;
                    y = new_xy.1;
                    if sediment < MAX_CARRIED_SEDIMENT {
                        modify_data(x, y, -EROSION_RATE, &mut data);
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
            new_heightmap_img.put_pixel(x, y, Luma([(data[y as usize][x as usize] + 128.0) as u8]))
        }
    }
    new_heightmap_img.save("new_heightmap.bmp").unwrap();
    Terrain {
        data: data,
        ..terrain
    }
}
