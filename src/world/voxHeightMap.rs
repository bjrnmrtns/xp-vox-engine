use std::collections::HashMap;

pub struct VoxHeightMap {
    data: Vec<f32>,
    pub x_size: usize,
    pub z_size: usize,
}

impl VoxHeightMap {
    pub fn new(x_size: usize, z_size: usize) -> Self {
        Self {
            data: vec![-10.0; z_size * x_size],
            x_size,
            z_size,
        }
    }

    pub fn set(&mut self, x: usize, z: usize, height: f32) {
        self.data[z * self.x_size + x] = height;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        let y_height = y as f32 * 0.1 - 1.6;
        if y_height <= self.data[z * self.x_size + x] {
            if y_height > 0.0 {
                return Some(1);
            } else {
                return Some(0);
            }
        }
        None
    }

    pub fn get_color(color_id: u8) -> [f32; 3] {
        if color_id == 1 {
            [0.0, 1.0, 0.0]
        } else {
            [1.0, 0.0, 0.0]
        }
    }
}
