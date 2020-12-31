use crate::petra::generation;
use generation::Terrain;
use rand::random;
use std::f32;
use std::usize;
use image::{ImageBuffer, Rgb, Luma};

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

fn lerp(s: f32, e: f32, i: f32) -> f32 {
    s + (e - s) * i
}

fn get_subpixel_value(x: f32, y: f32, data: &Vec<Vec<f32>>, len: usize) -> Option<f32> {
    // P1   P2
    //  (x,y)
    // P3   P4
    // Bilinear interpolation, with linear interpolation/nearest neighbor for edge values.
    let x_adjusted = x % 1.0;
    let y_adjusted = y % 1.0;
    assert!(x_adjusted.is_nan() == false);
    assert!(y_adjusted.is_nan() == false);
    if x.floor() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
        && y.floor() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
    {
        // Full bilinear
        let p1 = data[y.floor() as usize][x.floor() as usize];
        let p2 = data[y.floor() as usize][x.ceil() as usize];
        let p3 = data[y.ceil() as usize][x.floor() as usize];
        let p4 = data[y.ceil() as usize][x.ceil() as usize];
        assert!(p1.is_nan() == false);
        assert!(p2.is_nan() == false);
        assert!(p3.is_nan() == false);
        assert!(p4.is_nan() == false);
        let top_x_interp = lerp(p1, p2, x_adjusted);
        assert!(top_x_interp.is_nan() == false);
        let bottom_x_interp = lerp(p3, p4, x_adjusted);
        assert!(bottom_x_interp.is_nan() == false);
        assert!(p2.is_nan() == false);
        assert!(bottom_x_interp.is_nan() == false);
        assert!(lerp(top_x_interp, bottom_x_interp, y_adjusted).is_nan() == false);
        return Some(lerp(top_x_interp, bottom_x_interp, y_adjusted));
    } else if x.floor() < 0.0
        && x.ceil() <= (len as f32) - 1.0
        && x.ceil() >= 0.0
        && y.floor() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
    {
        // left side cut off
        let p2 = data[y.floor() as usize][x.ceil() as usize];
        let p4 = data[y.ceil() as usize][x.ceil() as usize];
        assert!(p2.is_nan() == false);
        assert!(p4.is_nan() == false);
        assert!(lerp(p2, p4, y_adjusted).is_nan() == false);
        return Some(lerp(p2, p4, y_adjusted));
    } else if x.floor() >= 0.0
        && x.floor() <= (len as f32) - 1.0
        && x.ceil() > (len as f32) - 1.0
        && y.floor() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
    {
        // right side cut off
        let p1 = data[y.floor() as usize][x.floor() as usize];
        let p3 = data[y.ceil() as usize][x.floor() as usize];
        assert!(p1.is_nan() == false);
        assert!(p3.is_nan() == false);
        assert!(lerp(p1, p3, y_adjusted).is_nan() == false);
        return Some(lerp(p1, p3, y_adjusted));
    } else if x.floor() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
        && y.floor() < 0.0
        && y.ceil() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
    {
        // top cut off
        let p3 = data[y.ceil() as usize][x.floor() as usize];
        let p4 = data[y.ceil() as usize][x.ceil() as usize];
        assert!(p3.is_nan() == false);
        assert!(p4.is_nan() == false);
        assert!(lerp(p3, p4, x_adjusted).is_nan() == false);
        return Some(lerp(p3, p4, x_adjusted));
    } else if x.floor() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
        && y.floor() >= 0.0
        && y.floor() <= (len as f32) - 1.0
        && y.ceil() > (len as f32) - 1.0
    {
        // bottom cut off
        let p1 = data[y.floor() as usize][x.floor() as usize];
        let p2 = data[y.floor() as usize][x.ceil() as usize];
        assert!(p1.is_nan() == false);
        assert!(p2.is_nan() == false);
        assert!(lerp(p1, p2, x_adjusted).is_nan() == false);
        return Some(lerp(p1, p2, x_adjusted));
    } else if y.floor() >= 0.0
        && y.floor() <= (len as f32) - 1.0
        && x.floor() >= 0.0
        && x.floor() <= (len as f32) - 1.0
    {
        // If p1 isn't cut off (top left corner)
        assert!(data[y.floor() as usize][x.floor() as usize].is_nan() == false);
        return Some(data[y.floor() as usize][x.floor() as usize]);
    } else if y.floor() >= 0.0
        && y.floor() <= (len as f32) - 1.0
        && x.ceil() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
    {
        // If p2 isn't cut off (top right corner)
        assert!(data[y.floor() as usize][x.ceil() as usize].is_nan() == false);
        return Some(data[y.floor() as usize][x.ceil() as usize]);
    } else if y.ceil() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
        && x.floor() >= 0.0
        && x.floor() <= (len as f32) - 1.0
    {
        // If p3 isn't cut off (bottom left corner)
        assert!(data[y.ceil() as usize][x.floor() as usize].is_nan() == false);
        return Some(data[y.ceil() as usize][x.floor() as usize]);
    } else if y.ceil() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
        && x.ceil() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
    {
        // If p4 isn't cut off (top right corner)
        assert!(data[y.ceil() as usize][x.ceil() as usize].is_nan() == false);
        return Some(data[y.ceil() as usize][x.ceil() as usize]);
    } else {
        return None;
    }
}

fn get_subpixel_weights(x: f32, y: f32) -> (f32, f32, f32, f32) {
    let x_adjusted = x % 1.0;
    let y_adjusted = y % 1.0;
    assert!(x_adjusted.is_nan() == false);
    assert!(y_adjusted.is_nan() == false);
    let p1_weight = lerp(lerp(1.0, 0.0, x_adjusted), 0.0, y_adjusted);
    let p2_weight = lerp(lerp(0.0, 1.0, x_adjusted), 0.0, y_adjusted);
    let p3_weight = lerp(0.0, lerp(1.0, 0.0, x_adjusted), y_adjusted);
    let p4_weight = lerp(0.0, lerp(0.0, 1.0, x_adjusted), y_adjusted);
    assert!(p1_weight.is_nan() == false);
    assert!(p2_weight.is_nan() == false);
    assert!(p3_weight.is_nan() == false);
    assert!(p4_weight.is_nan() == false);
    (p1_weight, p2_weight, p3_weight, p4_weight)
}

fn modify_data(x: f32, y: f32, change: f32, data: &mut Vec<Vec<f32>>) {
    let (p1, p2, p3, p4) = get_subpixel_weights(x, y);
    data[y.floor() as usize][x.floor() as usize] += p1 * change;
    data[y.floor() as usize][x.ceil() as usize] += p2 * change;
    data[y.ceil() as usize][x.floor() as usize] += p3 * change;
    data[y.ceil() as usize][x.ceil() as usize] += p4 * change;
    assert!(data[y.floor() as usize][x.floor() as usize].is_nan() == false);
    assert!(data[y.floor() as usize][x.ceil() as usize].is_nan() == false);
    assert!(data[y.ceil() as usize][x.floor() as usize].is_nan() == false);
    assert!(data[y.ceil() as usize][x.ceil() as usize].is_nan() == false);
}

// Simple pythagorean theorem based distance measure.
fn get_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    return ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
}

//Takes point and value map, returns downhill vector
fn get_slope_vector(x: f32, y: f32, data: &Vec<Vec<f32>>, len: usize) -> Option<(f32, f32)> {
    // To get the angle, we "pull" the point towards each side based on the values of each side subpixel.
    let base_value: f32 = get_subpixel_value(x, y, data, len)?;
    let left_value: f32 = get_subpixel_value(x - 1., y, data, len)? - base_value;
    let right_value: f32 = get_subpixel_value(x + 1., y, data, len)? - base_value;
    let up_value: f32 = get_subpixel_value(x, y - 1., data, len)? - base_value;
    let down_value: f32 = get_subpixel_value(x, y + 1., data, len)? - base_value;
    assert!(base_value.is_nan() == false);
    assert!(left_value.is_nan() == false);
    assert!(right_value.is_nan() == false);
    assert!(up_value.is_nan() == false);
    assert!(down_value.is_nan() == false);

    let x_weighted: f32 = left_value * 1f32 + right_value * -1f32; //Weights are inverted because it goes downhill
    let y_weighted: f32 = up_value * 1f32 + down_value * -1f32;
    if !x_weighted.is_nan() == false {
        panic!("{}", x_weighted);
    }
    if !y_weighted.is_nan() == false {
        panic!("{}", y_weighted);
    }
    assert!(x_weighted.is_nan() == false);
    assert!(y_weighted.is_nan() == false);

    let angle: f32;
    if x_weighted != 0.0 {
        //The slope will only be attainable if there is a "run"
        angle = y_weighted.atan2(x_weighted);
        assert!(angle.is_nan() == false);
    } else {
        if y_weighted >= 0.0 {
            angle = f32::consts::PI / 2.;
            assert!(angle.is_nan() == false);
        } else if y_weighted <= 0.0 {
            angle = f32::consts::PI * 1.5;
            assert!(angle.is_nan() == false);
        } else {
            angle = random::<f32>() * 2.0 * f32::consts::PI; // Random angle if there's no clear slope.
            assert!(angle.is_nan() == false);
        }
    }
    assert!(angle.is_nan() == false);

    let magnitude = get_distance(0., 0., x_weighted, y_weighted); // This will be biased towards "cardinal" directions.
    assert!(magnitude.is_nan() == false);

    return Some((angle, magnitude));
}

//Currently unused. Will be used when implementing actual particle-based erosion.
fn offset_vector(xy: (f32, f32), vector: (f32, f32)) -> (f32, f32) {
    return (
        xy.0 + vector.0.cos() * vector.1,
        xy.1 + vector.0.sin() * vector.1,
    ); // Inverse y axis
}

/*
const NUM_DROPLETS: u32 = 500000;
const SLOPE_THRESHOLD: f32 = 0.01;
const DEPOSIT_RATE: f32 = 0.3;
const EROSION_RATE: f32 = 0.4;
const MAX_ITERATIONS: u16 = 128;
const ITERATION_SCALE: f32 = 0.05;
*/

const NUM_DROPLETS: u32 = 50000;
const SLOPE_THRESHOLD: f32 = 0.05;
const DEPOSIT_RATE: f32 = 0.08;
const EROSION_RATE: f32 = 0.12;
const MAX_ITERATIONS: u16 = 32;
const EXP_BASE: f32 = 1.5;
const EROSION_MULTIPLIER: f32 = 40.0;

pub fn erode(terrain: Terrain) -> Terrain {
    //Save original heightmap
    let mut original_heightmap_img = ImageBuffer::new(generation::TERRAIN_SIZE as u32, generation::TERRAIN_SIZE as u32);
    for x in 0..(generation::TERRAIN_SIZE as u32) {
        for y in 0..(generation::TERRAIN_SIZE as u32) {
            original_heightmap_img.put_pixel(x, y, Luma([(terrain.data[y as usize][x as usize] + 128.0) as u8]))
        }
    }
    original_heightmap_img.save("heightmap.bmp").unwrap();

    let mut data = terrain.data;
    for drop in 0..NUM_DROPLETS {
        let mut x = random::<f32>() * generation::TERRAIN_SIZE as f32;
        let mut y = random::<f32>() * generation::TERRAIN_SIZE as f32;
        let mut sediment: f32 = 0.0;
        for step in 0..MAX_ITERATIONS {
            let slope_vector_option =
                get_slope_vector(x, y, &data, generation::TERRAIN_SIZE);
            if drop == 0 {
                println!("{:?}", (x, y))
            }
            if slope_vector_option.is_some() {
                let slope_vector = slope_vector_option.unwrap();
                if slope_vector.1 < SLOPE_THRESHOLD {
                    break;
                }
                if drop == 8000 {
                    println!("{}", slope_vector.1);
                }
                /*let deposit = sediment * DEPOSIT_RATE * (slope_vector.1/256.0);
                let erosion =
                    EROSION_RATE * (1.0 - 0.5) * EXP_BASE.powi((step as i32) * -1);
                modify_data(x, y, deposit - erosion, &mut data);
                sediment += erosion - deposit;*/
                modify_data(x, y, (EXP_BASE).powi((step as i32) * -1) * -EROSION_MULTIPLIER * (slope_vector.1/256.0), &mut data);
                let xy = offset_vector((x, y), slope_vector);
                x = xy.0;
                y = xy.1;
            } else {
                break;
            }
        }
    }
    let mut new_heightmap_img = ImageBuffer::new(generation::TERRAIN_SIZE as u32, generation::TERRAIN_SIZE as u32);
    for x in 0..(generation::TERRAIN_SIZE as u32) {
        for y in 0..(generation::TERRAIN_SIZE as u32) {
            new_heightmap_img.put_pixel(x, y, Luma([(data[y as usize][x as usize] + 128.0)as u8]))
        }
    }
    new_heightmap_img.save("new_heightmap.bmp").unwrap();
    Terrain {
        data: data,
        ..terrain
    }
}
