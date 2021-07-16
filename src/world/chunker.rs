use crate::{
    mesh::MeshData,
    registry::Handle,
    transform::Transform,
    world::{greedy_meshing, vox3d::Vox3d, voxHeightMap::VoxHeightMap},
};
use glam::Vec3;
use noise::{Fbm, MultiFractal, NoiseFn};
use std::collections::HashMap;

pub struct Chunker {
    noise_function: Fbm,
    chunk_size: usize,
}

impl Chunker {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            noise_function: Fbm::new()
                .set_octaves(5)
                .set_frequency(0.001)
                .set_lacunarity(2.09)
                .set_persistence(1.0),
            chunk_size,
        }
    }

    pub fn generate_chunk(&mut self, chunk: [i32; 2]) -> (MeshData, Transform) {
        let mut vox_to_gen = VoxHeightMap::new(self.chunk_size, self.chunk_size);
        for z in 0..self.chunk_size {
            for x in 0..self.chunk_size {
                let x_w = chunk[0] as f32 * self.chunk_size as f32 * 0.1 + x as f32 * 0.1;
                let z_w = chunk[1] as f32 * self.chunk_size as f32 * 0.1 + z as f32 * 0.1;
                vox_to_gen.set(x, z, self.noise_function.get([x_w as f64, z_w as f64]) as f32);
            }
        }
        (
            greedy_meshing::greedy_mesh_base(&vox_to_gen),
            Transform::from_translation(Vec3::new(
                chunk[0] as f32 * self.chunk_size as f32 * 0.1,
                0.0,
                chunk[1] as f32 * self.chunk_size as f32 * 0.1,
            )),
        )
    }
}
