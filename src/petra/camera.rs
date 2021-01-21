use bevy::render::camera::{Camera, CameraProjection, DepthCalculation, VisibleEntities};
use bevy::prelude::*;

pub struct OrthoProjection {
    pub far: f32,
    pub aspect: f32,
}

impl CameraProjection for OrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            -self.aspect*100.0, self.aspect*100.0, -100.0, 100.0, 0.0, self.far
        )
    }

    // what to do on window resize
    fn update(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

impl Default for OrthoProjection {
    fn default() -> Self {
        Self { far: 1000.0, aspect: 1.0 }
    }
}