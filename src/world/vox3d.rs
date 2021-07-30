use crate::{
    registry::{Handle, Registry},
    transform::Transform,
    world::{constants::VOXEL_SIZE_IN_METERS, vox::Vox},
};
use std::collections::HashMap;

pub struct Vox3d {
    data: Vec<Option<u8>>,
    palette: HashMap<u8, [f32; 3]>,
    pub x_size: usize,
    pub y_size: usize,
    pub z_size: usize,
    pub touched: bool,
}

impl Vox3d {
    pub fn new(x_size: usize, y_size: usize, z_size: usize) -> Self {
        Self {
            data: vec![None; z_size * y_size * x_size],
            palette: HashMap::default(),
            x_size,
            y_size,
            z_size,
            touched: false,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, color_id: u8, color: [f32; 3]) {
        self.touched = true;
        self.data[z * self.y_size * self.x_size + y * self.x_size + x] = Some(color_id);
        self.palette.insert(color_id, color);
    }
}

impl Vox for Vox3d {
    fn get_size(&self) -> [usize; 3] {
        [self.x_size, self.y_size, self.z_size]
    }

    fn get(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        self.data[z * self.y_size * self.x_size + y * self.x_size + x]
    }

    fn get_color(&self, color_id: u8) -> [f32; 3] {
        self.palette[&color_id]
    }

    fn get_y_min_offset(&self) -> f32 {
        0.0
    }

    fn get_y_max_offset(&self) -> f32 {
        self.get_size()[1] as f32 * VOXEL_SIZE_IN_METERS
    }
}

pub fn load_vox(data: &dot_vox::DotVoxData, registry: &mut Registry<Vox3d>) -> Handle<Vox3d> {
    let model = &data.models[0];
    let mut vox_model = Vox3d::new(model.size.x as usize, model.size.z as usize, model.size.y as usize);
    for v in &model.voxels {
        let color = palette_to_color(data.palette[v.i as usize]);
        vox_model.set(v.x as usize, v.z as usize, v.y as usize, v.i, color);
    }
    registry.add(vox_model)
}

fn palette_to_color(from: u32) -> [f32; 3] {
    let (_a, b, g, r) = (from >> 24 & 0xFF, from >> 16 & 0xFF, from >> 8 & 0xFF, from & 0xFF);
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
}
