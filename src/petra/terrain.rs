use bevy::math::Vec2;
use std::ops::Index;
use std::ops::IndexMut;

pub struct TerrainData {
    pub data: Vec<f32>,
    pub size: usize,
}

fn lerp(s: f32, e: f32, i: f32) -> f32 {
    s + (e - s) * i
}

impl Index<(usize, usize)> for TerrainData {
    type Output = f32;

    fn index(&self, coordinates: (usize, usize)) -> &Self::Output {
        &self.data[self.size * coordinates.1 + coordinates.0]
    }
}

impl IndexMut<(usize, usize)> for TerrainData {
    fn index_mut(&mut self, coordinates: (usize, usize)) -> &mut f32 {
        &mut self.data[self.size * coordinates.1 + coordinates.0]
    }
}

impl TerrainData {
    pub fn new(data: Vec<f32>, size: usize) -> Self {
        TerrainData { data, size }
    }

    pub fn zeros(size: usize) -> Self {
        TerrainData {
            data: vec![0.0; size*size],
            size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<f32> {
        if x >= 0 && x < self.size && y >= 0 && y < self.size {
            Some(self[(x, y)])
        }else{
            None
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

        self.get(new_x, new_y)
    }

    pub fn sample(&self, pos: Vec2) -> Option<f32> {
        // P1   P2
        //  (x,y)
        // P3   P4
        // Bilinear interpolation, with linear interpolation/nearest neighbor for edge values.
        let Vec2 { x, y } = pos;
        let len = self.size;
        let x_adjusted = x % 1.0;
        let y_adjusted = y % 1.0;
        if x.floor() >= 0.0
            && x.ceil() <= (len as f32) - 1.0
            && y.floor() >= 0.0
            && y.ceil() <= (len as f32) - 1.0
        {
            // Full bilinear
            let p1 = self[(x.floor() as usize, y.floor() as usize)];
            let p2 = self[(x.ceil() as usize, y.floor() as usize)];
            let p3 = self[(x.floor() as usize, y.ceil() as usize)];
            let p4 = self[(x.ceil() as usize, y.ceil() as usize)];
            let top_x_interp = lerp(p1, p2, x_adjusted);
            let bottom_x_interp = lerp(p3, p4, x_adjusted);
            Some(lerp(top_x_interp, bottom_x_interp, y_adjusted))
        } else if x.floor() < 0.0
            && x.ceil() <= (len as f32) - 1.0
            && x.ceil() >= 0.0
            && y.floor() >= 0.0
            && y.ceil() <= (len as f32) - 1.0
        {
            // left side cut off
            let p2 = self[(x.ceil() as usize, y.floor() as usize)];
            let p4 = self[(x.ceil() as usize, y.ceil() as usize)];
            return Some(lerp(p2, p4, y_adjusted));
        } else if x.floor() >= 0.0
            && x.floor() <= (len as f32) - 1.0
            && x.ceil() > (len as f32) - 1.0
            && y.floor() >= 0.0
            && y.ceil() <= (len as f32) - 1.0
        {
            // right side cut off
            let p1 = self[(x.floor() as usize, y.floor() as usize)];
            let p3 = self[(x.floor() as usize, y.ceil() as usize)];
            Some(lerp(p1, p3, y_adjusted))
        } else if x.floor() >= 0.0
            && x.ceil() <= (len as f32) - 1.0
            && y.floor() < 0.0
            && y.ceil() >= 0.0
            && y.ceil() <= (len as f32) - 1.0
        {
            // top cut off
            let p3 = self[(x.floor() as usize, y.ceil() as usize)];
            let p4 = self[(x.ceil() as usize, y.ceil() as usize)];
            Some(lerp(p3, p4, x_adjusted))
        } else if x.floor() >= 0.0
            && x.ceil() <= (len as f32) - 1.0
            && y.floor() >= 0.0
            && y.floor() <= (len as f32) - 1.0
            && y.ceil() > (len as f32) - 1.0
        {
            // bottom cut off
            let p1 = self[(x.ceil() as usize, y.floor() as usize)];
            let p2 = self[(x.ceil() as usize, y.floor() as usize)];
            Some(lerp(p1, p2, x_adjusted))
        } else if y.ceil() >= 0.0
            && y.ceil() <= (len as f32) - 1.0
            && x.floor() >= 0.0
            && x.floor() <= (len as f32) - 1.0
        {
            // If p1 isn't cut off (top left corner)
            Some(self[(x.floor() as usize, y.floor() as usize)])
        } else if y.floor() >= 0.0
            && y.floor() <= (len as f32) - 1.0
            && x.ceil() >= 0.0
            && x.ceil() <= (len as f32) - 1.0
        {
            // If p2 isn't cut off (top right corner)
            Some(self[(x.ceil() as usize, y.floor() as usize)])
        } else if y.ceil() >= 0.0
            && y.ceil() <= (len as f32) - 1.0
            && x.floor() >= 0.0
            && x.floor() <= (len as f32) - 1.0
        {
            // If p3 isn't cut off (bottom left corner)
            Some(self[(x.floor() as usize, y.ceil() as usize)])
        } else if y.ceil() >= 0.0
            && y.ceil() <= (len as f32) - 1.0
            && x.ceil() >= 0.0
            && x.ceil() <= (len as f32) - 1.0
        {
            // If p4 isn't cut off (top right corner)
            Some(self[(x.ceil() as usize, y.ceil() as usize)])
        } else {
            None
        }
    }

    fn get_subpixel_weights(&self, x: f32, y: f32) -> (f32, f32, f32, f32) {
        let x_adjusted = x % 1.0;
        let y_adjusted = y % 1.0;
        let p1_weight = lerp(lerp(1.0, 0.0, x_adjusted), 0.0, y_adjusted);
        let p2_weight = lerp(lerp(0.0, 1.0, x_adjusted), 0.0, y_adjusted);
        let p3_weight = lerp(0.0, lerp(1.0, 0.0, x_adjusted), y_adjusted);
        let p4_weight = lerp(0.0, lerp(0.0, 1.0, x_adjusted), y_adjusted);
        (p1_weight, p2_weight, p3_weight, p4_weight)
    }

    pub fn offset(&mut self, x: f32, y: f32, change: f32) {
        let (p1, p2, p3, p4) = &self.get_subpixel_weights(x, y);
        self[(x.floor() as usize, y.floor() as usize)] += p1 * change;
        self[(x.ceil() as usize, y.floor() as usize)] += p2 * change;
        self[(x.floor() as usize, y.ceil() as usize)] += p3 * change;
        self[(x.ceil() as usize, y.ceil() as usize)] += p4 * change;
    }
}

pub struct Terrain {
    pub data: TerrainData,
    pub size: usize,
    pub worldscale: f32,
    pub height: f32,
    pub noisescale: f32,
}
