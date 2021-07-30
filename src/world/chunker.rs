use crate::{
    mesh::MeshData,
    registry::Handle,
    transform::Transform,
    world::{
        constants,
        constants::{CHUNK_SIZE_IN_METERS, CHUNK_SIZE_IN_VOXELS, VOXEL_SIZE_IN_METERS},
        greedy_meshing,
        vox::Vox,
        vox3d::Vox3d,
        voxheightmap::VoxHeightMap,
    },
};
use glam::Vec3;
use noise::{Fbm, MultiFractal, NoiseFn};
use std::collections::HashMap;

pub struct Chunker {
    noise_function: Fbm,
}

impl Chunker {
    pub fn new() -> Self {
        Self {
            noise_function: Fbm::new()
                .set_octaves(5)
                .set_frequency(0.001)
                .set_lacunarity(2.09)
                .set_persistence(1.0),
        }
    }

    pub fn generate_chunk(&mut self, chunk: [i32; 2]) -> Vec<(MeshData, Transform)> {
        let mut ground_vox = VoxHeightMap::new(CHUNK_SIZE_IN_VOXELS, CHUNK_SIZE_IN_VOXELS);
        for z in 0..CHUNK_SIZE_IN_VOXELS {
            for x in 0..CHUNK_SIZE_IN_VOXELS {
                let x_w = chunk[0] as f32 * CHUNK_SIZE_IN_METERS + x as f32 * VOXEL_SIZE_IN_METERS;
                let z_w = chunk[1] as f32 * CHUNK_SIZE_IN_METERS + z as f32 * VOXEL_SIZE_IN_METERS;
                ground_vox.set(x, z, self.noise_function.get([x_w as f64, z_w as f64]) as f32);
            }
        }
        let mut vegetation = Vox3d::new(1, 1, 1);
        vegetation.set(0, 0, 0, 1, [1.0, 0.0, 0.0]);
        let vegetation_mesh_data = greedy_meshing::greedy_mesh(&vegetation);
        let vegetation_transform = Transform::from_translation(Vec3::new(
            chunk[0] as f32 * CHUNK_SIZE_IN_METERS,
            ground_vox.get_y_max_offset(),
            chunk[1] as f32 * CHUNK_SIZE_IN_METERS,
        ));

        [
            (vegetation_mesh_data, vegetation_transform),
            (
                greedy_meshing::greedy_mesh(&ground_vox),
                Transform::from_translation(Vec3::new(
                    chunk[0] as f32 * CHUNK_SIZE_IN_METERS,
                    ground_vox.get_y_min_offset(),
                    chunk[1] as f32 * CHUNK_SIZE_IN_METERS,
                )),
            ),
        ]
        .to_vec()
    }
}
