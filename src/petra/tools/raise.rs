use crate::petra::terrain;
use bevy::math::{vec2, Vec2};

pub fn trigger(xy: Vec2, radius: i64, terrain: &mut terrain::Terrain) {
    for x in -radius..radius {
        for y in -radius..radius {
            let strength = ((radius as f32) - vec2(x as f32, y as f32).length()) / (radius as f32);
            if strength > 0.0 {
                terrain
                    .data
                    .modify(xy + vec2(x as f32, y as f32), strength * 1.0);
            }
        }
    }
}
