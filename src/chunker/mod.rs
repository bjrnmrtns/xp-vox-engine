use crate::{
    mesh::Mesh,
    registry::{Handle, Registry},
    transform::Transform,
    vox::Vox,
};
use glam::Vec3;
use std::collections::HashMap;

mod greedy_meshing;

pub struct Chunker {
    entities: Vec<(Handle<Vox>, [usize; 3], [i32; 3])>,
    chunk_entity_map: HashMap<(i32, i32, i32), (usize, [usize; 3], [usize; 3], [usize; 3])>,
    chunk_size: usize,
}

impl Chunker {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            chunk_entity_map: HashMap::new(),
            chunk_size: 32,
        }
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

    pub fn add(&mut self, handle: Handle<Vox>, position: [i32; 3], registry: &Registry<Vox>) {
        let vox = registry.get(&handle).unwrap();
        self.entities
            .push((handle, [vox.x_size, vox.y_size, vox.z_size], position));
        let x_min = position[0];
        let y_min = position[1];
        let z_min = position[2];
        let mut z_size = vox.z_size;
        let (mut z_number, mut target_z_offset) = Chunker::chunk_number_and_offset(z_min, self.chunk_size);
        let mut source_z_offset = 0;
        while z_size != 0 {
            let z_current_size = std::cmp::min(z_size, self.chunk_size - target_z_offset);
            let mut y_size = vox.y_size;
            let (mut y_number, mut target_y_offset) = Chunker::chunk_number_and_offset(y_min, self.chunk_size);
            let mut source_y_offset = 0;
            while y_size != 0 {
                let y_current_size = std::cmp::min(y_size, self.chunk_size - target_y_offset);
                let mut x_size = vox.x_size;
                let (mut x_number, mut target_x_offset) = Chunker::chunk_number_and_offset(x_min, self.chunk_size);
                let mut source_x_offset = 0;
                while x_size != 0 {
                    let x_current_size = std::cmp::min(x_size, self.chunk_size - target_x_offset);
                    self.chunk_entity_map.insert(
                        (x_number, y_number, z_number),
                        (
                            self.entities.len() - 1,
                            [source_x_offset, source_y_offset, source_z_offset],
                            [target_x_offset, target_y_offset, target_z_offset],
                            [x_current_size, y_current_size, z_current_size],
                        ),
                    );
                    x_number += 1;
                    source_x_offset += x_current_size;
                    target_x_offset = 0;
                    x_size -= x_current_size;
                }
                y_number += 1;
                source_y_offset += y_current_size;
                target_y_offset = 0;
                y_size -= y_current_size;
            }
            z_number += 1;
            source_z_offset += z_current_size;
            target_z_offset = 0;
            z_size -= z_current_size;
        }
    }

    pub fn generate_chunk(&mut self, registry: &Registry<Vox>, chunk: (i32, i32, i32)) -> (Option<Mesh>, Transform) {
        let mut vox_to_gen = Vox::new(self.chunk_size, self.chunk_size, self.chunk_size);
        for z in 0..self.chunk_size {
            for y in 0..self.chunk_size {
                for x in 0..self.chunk_size {
                    let x_w = chunk.0 as f32 * self.chunk_size as f32 * 0.1 + x as f32 * 0.1;
                    let y_w = chunk.1 as f32 * self.chunk_size as f32 * 0.1 + y as f32 * 0.1;
                    let z_w = chunk.2 as f32 * self.chunk_size as f32 * 0.1 + z as f32 * 0.1;
                    if y_w > -5.0 && ((x_w as f32).sin() * (z_w as f32).sin()) > y_w {
                        vox_to_gen.set(x, y, z, 255, [1.0, 0.0, 0.0]);
                    }
                }
            }
        }
        if let Some((vox_id, source_offset, target_offset, size)) = &self.chunk_entity_map.get(&chunk) {
            let (handle, _, _) = &self.entities[*vox_id];
            let vox = registry.get(handle).unwrap();
            for z in 0..size[2] {
                for y in 0..size[1] {
                    for x in 0..size[0] {
                        if let Some(color_id) =
                            vox.get(source_offset[0] + x, source_offset[1] + y, source_offset[2] + z)
                        {
                            let color = vox.get_color(color_id);
                            vox_to_gen.set(
                                target_offset[0] + x,
                                target_offset[1] + y,
                                target_offset[2] + z,
                                color_id,
                                color,
                            );
                        }
                    }
                }
            }
        }
        let mesh = greedy_meshing::greedy_mesh(vox_to_gen);
        (
            mesh,
            Transform::from_translation(Vec3::new(
                chunk.0 as f32 * self.chunk_size as f32 * 0.1,
                chunk.1 as f32 * self.chunk_size as f32 * 0.1,
                chunk.2 as f32 * self.chunk_size as f32 * 0.1,
            )),
        )
    }
}
