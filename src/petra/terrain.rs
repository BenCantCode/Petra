use bevy::math::{vec2, Vec2};
use std::collections::HashMap;
use std::f32;
use std::ops::Index;
use std::ops::IndexMut;
const CHUNK_SIZE: usize = 64;

// This is mostly meant as a thin layer on top of TerrainData. Most relevant methods will go under TerrainData.
#[derive(Debug, Clone, Copy)]
pub struct TerrainDataChunk {
    pub data: [f32; (CHUNK_SIZE * CHUNK_SIZE) as usize],
    pub coords: (i32, i32),
    pub modified: bool
}

impl TerrainDataChunk {
    pub const size: usize = CHUNK_SIZE;
    pub fn new(coords: (i32, i32)) -> Self {
        Self {
            data: [0.0; Self::size*Self::size],
            coords,
            modified: false
        }
    }
    pub fn get_safe(&self, x: usize, y: usize, x_offset: i32, y_offset: i32) -> Option<f32> {
        let new_x = if x_offset.is_positive() {
            x.checked_add(x_offset as usize)?
        }else{
            x.checked_sub(x_offset.abs() as usize)?
        };

        let new_y = if y_offset.is_positive() {
            y.checked_add(y_offset as usize)?
        }else{
            y.checked_sub(y_offset.abs() as usize)?
        };

        if new_x >= Self::size || new_y >= Self::size {
            return None;
        }

        Some(self.data[new_y*TerrainDataChunk::size+new_x])
    }
}
pub struct TerrainData {
    pub chunks: HashMap<(i32, i32), TerrainDataChunk>,
}

fn lerp(s: f32, e: f32, i: f32) -> f32 {
    s + (e - s) * i
}

impl Index<(i32, i32)> for TerrainData {
    type Output = f32;

    fn index(&self, coordinates: (i32, i32)) -> &Self::Output {
        let chunk_coordinates = Self::get_terrain_chunk_coordinates(coordinates);
        let relative_x = coordinates.0.rem_euclid(TerrainDataChunk::size as i32);
        let relative_y = coordinates.1.rem_euclid(TerrainDataChunk::size as i32);
        if !self.chunks.contains_key(&chunk_coordinates) {
            &0.0
        } else {
            &self.chunks.get(&chunk_coordinates).unwrap().data
                [(relative_y as usize) * TerrainDataChunk::size + (relative_x as usize)]
        }
    }
}

impl IndexMut<(i32, i32)> for TerrainData {
    fn index_mut(&mut self, coordinates: (i32, i32)) -> &mut f32 {
        let chunk_coordinates = Self::get_terrain_chunk_coordinates(coordinates);
        let relative_x = coordinates.0.rem_euclid(TerrainDataChunk::size as i32);
        let relative_y = coordinates.1.rem_euclid(TerrainDataChunk::size as i32);
        if !self.chunks.contains_key(&chunk_coordinates) {
            self.chunks.insert(
                chunk_coordinates,
                TerrainDataChunk::new(chunk_coordinates)
            );
        }
        &mut self.chunks.get_mut(&chunk_coordinates).unwrap().data
            [(relative_y as usize) * TerrainDataChunk::size + (relative_x as usize)]
    }
}

impl TerrainData {
    pub fn new(data: HashMap<(i32, i32), TerrainDataChunk>) -> Self {
        TerrainData {
            chunks: data,
        }
    }

    pub fn zeros() -> Self {
        TerrainData {
            chunks: HashMap::new(),
        }
    }

    pub fn get_terrain_chunk_coordinates(coordinates: (i32, i32)) -> (i32, i32) {
        (
            (coordinates.0 as i32).div_euclid(TerrainDataChunk::size as i32),
            (coordinates.1 as i32).div_euclid(TerrainDataChunk::size as i32),
        )
    }

    pub fn sample(&self, pos: Vec2) -> Option<f32> {
        // P1   P2
        //  (x,y)
        // P3   P4
        // Bilinear interpolation, with linear interpolation/nearest neighbor for edge values.
        let x = pos.x;
        let y = pos.y;
        let x_adjusted = x.rem_euclid(1.0);
        let y_adjusted = x.rem_euclid(1.0);

        let p1 = self[(x.floor() as i32, y.floor() as i32)];
        let p2 = self[(x.ceil() as i32, y.floor() as i32)];
        let p3 = self[(x.floor() as i32, y.ceil() as i32)];
        let p4 = self[(x.ceil() as i32, y.ceil() as i32)];
        let top_x_interp = lerp(p1, p2, x_adjusted);
        let bottom_x_interp = lerp(p3, p4, x_adjusted);
        Some(lerp(top_x_interp, bottom_x_interp, y_adjusted))
    }

    fn get_subpixel_weights(&self, xy: Vec2) -> (f32, f32, f32, f32) {
        let x_adjusted = xy.x.rem_euclid(1.0);
        let y_adjusted = xy.y.rem_euclid(1.0);
        let p1_weight = lerp(lerp(1.0, 0.0, x_adjusted), 0.0, y_adjusted);
        let p2_weight = lerp(lerp(0.0, 1.0, x_adjusted), 0.0, y_adjusted);
        let p3_weight = lerp(0.0, lerp(1.0, 0.0, x_adjusted), y_adjusted);
        let p4_weight = lerp(0.0, lerp(0.0, 1.0, x_adjusted), y_adjusted);
        (p1_weight, p2_weight, p3_weight, p4_weight)
    }

    pub fn modify(&mut self, xy: Vec2, change: f32) {
        //println!("Bleh. ${:?}, ${:?}", xy, change);
        let (p1, p2, p3, p4) = &self.get_subpixel_weights(xy);
        let (x, y) = (xy.x, xy.y);
        if let Some(i) = self.chunks.get_mut(&Self::get_terrain_chunk_coordinates((xy.x as i32, xy.y as i32))) {
            i.modified = true;
        }
        //println!("I am going to add {} * {} to {} = {}", p1, change, self[(x.floor() as i32, y.floor() as i32)], p1 * change);
        self[(x.floor() as i32, y.floor() as i32)] += p1 * change;
        self[(x.ceil() as i32, y.floor() as i32)] += p2 * change;
        self[(x.floor() as i32, y.ceil() as i32)] += p3 * change;
        self[(x.ceil() as i32, y.ceil() as i32)] += p4 * change;
    }
    //Takes point and value map, returns downhill vector
    pub fn get_slope_vector(&self, pos: Vec2) -> Option<Vec2> {
        // To get the angle, we "pull" the point towards each side based on the values of each side subpixel.
        let left_value: f32 = self.sample(pos + vec2(-1., 0.0))?;
        let right_value: f32 = self.sample(pos + vec2(1., 0.0))?;
        let up_value: f32 = self.sample(pos + vec2(0.0, -1.0))?;
        let down_value: f32 = self.sample(pos + vec2(0.0, 1.0))?;

        let x_weighted: f32 = left_value * 1f32 + right_value * -1f32;
        let y_weighted: f32 = up_value * 1f32 + down_value * -1f32;

        return Some(vec2(x_weighted, y_weighted));
    }
}

pub struct Terrain {
    pub data: TerrainData,
    pub worldscale: f32,
    pub height: f32,
    pub noisescale: f32,
}

impl Default for Terrain {
    fn default() -> Terrain {
        Terrain {
            data: TerrainData::zeros(),
            worldscale: 256.0,
            height: 64.0,
            noisescale: 0.01,
        }
    }
}
