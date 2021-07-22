use crate::{transform::Transform, world::vox::Vox};
use std::collections::HashMap;

const COLOR_EARTH: [u8; 3] = [0x7d, 0x44, 0x27];
const COLOR_GRASS: [u8; 3] = [0x48, 0x6b, 0x00];
const COLOR_GREEN: [u8; 3] = [0x2e, 0x46, 0x00];
const COLOR_LIME: [u8; 3] = [0xa2, 0xc5, 0x23];
const COLOR_EARTH_ID: u8 = 0;
const COLOR_GRASS_ID: u8 = 1;
const COLOR_GREEN_ID: u8 = 2;
const COLOR_LIME_ID: u8 = 3;
const COLOR_TABLE: [[u8; 3]; 4] = [COLOR_EARTH, COLOR_GRASS, COLOR_GREEN, COLOR_LIME];

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
                return Some(COLOR_GREEN_ID);
            } else {
                return Some(COLOR_EARTH_ID);
            }
        }
        None
    }

    fn get_color(&self, color_id: u8) -> [f32; 3] {
        let color = COLOR_TABLE[color_id as usize];
        [
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
        ]
    }

    fn get_y_offset(&self) -> f32 {
        self.y_min_voxel() * 0.1
    }
}
