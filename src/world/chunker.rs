use crate::{mesh::MeshData, registry::Handle, transform::Transform, vox::Vox, world::greedy_meshing};
use glam::Vec3;
use std::collections::HashMap;

pub struct Chunker {
    entities: Vec<(Handle<Vox>, [usize; 3], [i32; 3])>,
    chunk_entity_map: HashMap<(i32, i32, i32), (usize, [usize; 3], [usize; 3], [usize; 3])>,
    chunk_size: usize,
}

fn chunk_number_and_offset(start: i32, chunk_size: usize) -> (i32, usize) {
    if start >= 0 {
        let chunk_number = start / chunk_size as i32;
        let offset = start as usize % chunk_size;
        (chunk_number, offset)
    } else {
        let chunk_number = (start + 1) / chunk_size as i32 - 1;
        let offset = chunk_size - (-start as usize % (chunk_size + 1));
        (chunk_number, offset)
    }
}

impl Chunker {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            entities: vec![],
            chunk_entity_map: HashMap::new(),
            chunk_size,
        }
    }

    pub fn generate_chunk(&mut self, chunk: [i32; 2]) -> (Option<MeshData>, Transform) {
        let mut vox_to_gen = Vox::new(self.chunk_size, self.chunk_size, self.chunk_size);
        for z in 0..self.chunk_size {
            for x in 0..self.chunk_size {
                let x_w = chunk[0] as f32 * self.chunk_size as f32 * 0.1 + x as f32 * 0.1;
                let z_w = chunk[1] as f32 * self.chunk_size as f32 * 0.1 + z as f32 * 0.1;
                let y = 0;
                let y_w = 0.0;
                if y_w > -5.0 && ((x_w as f32).sin() * (z_w as f32).sin()) > y_w {
                    vox_to_gen.set(x, y, z, 255, [1.0, 0.0, 0.0]);
                }
            }
        }
        (
            greedy_meshing::greedy_mesh(&vox_to_gen),
            Transform::from_translation(Vec3::new(
                chunk[0] as f32 * self.chunk_size as f32 * 0.1,
                0.0,
                chunk[1] as f32 * self.chunk_size as f32 * 0.1,
            )),
        )
    }
}
