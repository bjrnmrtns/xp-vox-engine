use crate::{transform::Transform, world::vox::Vox};
use std::collections::HashMap;

pub struct VoxHeightMap {
    data: Vec<f32>,
    pub x_size: usize,
    pub z_size: usize,
    pub y_min: f32,
    pub y_max: f32,
}

impl VoxHeightMap {
    pub fn new(x_size: usize, z_size: usize) -> Self {
        Self {
            data: vec![0.0; z_size * x_size],
            x_size,
            z_size,
            y_min: f32::MAX,
            y_max: f32::MIN,
        }
    }

    pub fn y_min_voxel(&self) -> f32 {
        (self.y_min / 0.1).floor()
    }

    pub fn y_max_voxel(&self) -> f32 {
        (self.y_max / 0.1).ceil()
    }

    pub fn set(&mut self, x: usize, z: usize, height: f32) {
        self.y_min = self.y_min.min(height);
        self.y_max = self.y_max.max(height);
        self.data[z * self.x_size + x] = height;
    }
}
impl Vox for VoxHeightMap {
    fn get_size(&self) -> [usize; 3] {
        let y_height = (self.y_max_voxel() - self.y_min_voxel()) as usize;
        [self.x_size, y_height, self.z_size]
    }

    fn get(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        let y_height = (y as f32 + self.y_min_voxel()) * 0.1;
        if y_height <= self.data[z * self.x_size + x] {
            if y_height > 0.0 {
                return Some(1);
            } else {
                return Some(0);
            }
        }
        None
    }

    fn get_color(&self, color_id: u8) -> [f32; 3] {
        if color_id == 1 {
            [0.0, 1.0, 0.0]
        } else {
            [1.0, 0.0, 0.0]
        }
    }

    fn get_y_offset(&self) -> f32 {
        self.y_min_voxel() * 0.1
    }
}
