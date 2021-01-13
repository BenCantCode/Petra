use crate::petra::terrain::{Terrain, TerrainDataChunk};
use bevy::math::{vec2, Vec2};
use image::{ImageBuffer, Luma, Rgb};
use rand::random;
use std::f32;
use std::usize;

const MAX_ITERATIONS: u32 = 1028;
const MAX_CARRIED_SEDIMENT: f32 = 1.0;
const EROSION_RATE: f32 = 0.12;
const DEPOSITION_RATE: f32 = 0.2;
const DRY_TRESHOLD: f32 = 0.08;

struct Droplet {
    xy: Vec2,
    sediment: f32,
    alive: bool,
}

impl Droplet {
    fn new(xy: Vec2, sediment: f32) -> Self {
        Self {
            xy,
            sediment,
            alive: true,
        }
    }
    fn step(&mut self, terrain: &mut Terrain) {
        let slope_vector_option = terrain.data.get_slope_vector(self.xy);
        if slope_vector_option.is_some() {
            let slope_vector = slope_vector_option.unwrap();
            let new_xy = slope_vector + self.xy;
            let height_difference =
                terrain.data.sample(new_xy).unwrap() - terrain.data.sample(self.xy).unwrap();
            if height_difference > 0.0 {
                // Deposit the carried sediment.
                terrain
                    .data
                    .modify(self.xy, self.sediment.min(DEPOSITION_RATE));
                self.sediment -= self.sediment.min(DEPOSITION_RATE);
                if self.sediment <= DRY_TRESHOLD {
                    self.alive = false;
                }
            } else {
                self.xy = new_xy;
                if self.sediment < MAX_CARRIED_SEDIMENT {
                    terrain.data.modify(self.xy, -EROSION_RATE);
                    self.sediment += EROSION_RATE;
                }
            }
        } else {
            self.alive = false;
        }
    }
}

pub fn trigger(xy: Vec2, radius: i64, terrain: &mut Terrain) {
    for x in -radius..radius {
        for y in -radius..radius {
            let strength = ((radius as f32) - vec2(x as f32, y as f32).length()) / (radius as f32);
            if random::<f32>() < strength {
                let mut droplet = Droplet::new(xy + vec2(x as f32, y as f32), 0.0);
                for i in 0..MAX_ITERATIONS {
                    droplet.step(terrain);
                    if !droplet.alive {
                        break;
                    }
                }
            }
        }
    }
}
