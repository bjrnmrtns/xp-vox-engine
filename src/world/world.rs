use crate::{
    asset::{AssetLoader, Command},
    entity::Entity,
    mesh::MeshData,
    registry::{Handle, Registry},
    renderer::{Mesh, Renderer},
    transform::Transform,
    world::sliding_vec3d::Vec3dSliding,
};
use glam::Mat4;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Chunk {
    location: [i32; 3],
    transform: Transform,
    requested: bool,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            location: [x, y, z],
            transform: Transform::identity(),
            requested: true,
        }
    }
}

pub struct World {
    chunks: Vec3dSliding<Option<Chunk>>,
    meshes: Vec3dSliding<Option<Handle<Mesh>>>,
    mesh_storage: Registry<Mesh>,
    center: Option<[f32; 3]>,
    previous_center: Option<[f32; 3]>,
    voxel_size: f32,
    chunk_size_in_voxels: u32,
    walking_window: [f32; 3],
    world_size_in_chunks_radius: [usize; 3],
}

impl World {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunks: Vec3dSliding::new([100, 100, 100]),
            meshes: Vec3dSliding::new([100, 100, 100]),
            mesh_storage: Registry::new(),
            center: None,
            previous_center: None,
            voxel_size: 0.1,
            chunk_size_in_voxels: chunk_size as u32,
            walking_window: [6.0, 6.0, 6.0],
            world_size_in_chunks_radius: [5, 3, 5],
        }
    }

    fn position_to_chunk_index_1d(position: f32, chunk_length: f32) -> i32 {
        (position / chunk_length).floor() as i32
    }

    fn position_to_chunk_index_3d(position: [f32; 3], chunk_length: f32) -> [i32; 3] {
        [
            Self::position_to_chunk_index_1d(position[0], chunk_length),
            Self::position_to_chunk_index_1d(position[1], chunk_length),
            Self::position_to_chunk_index_1d(position[2], chunk_length),
        ]
    }

    fn move_to_posidtion_1d(position: f32, center: f32, walking_window: f32) -> f32 {
        let offset = position - center;
        if offset < -walking_window / 2.0 {
            position + walking_window / 2.0
        } else if offset > walking_window / 2.0 {
            position - walking_window / 2.0
        } else {
            center
        }
    }

    fn update_center(&mut self, position: [f32; 3]) {
        if let Some(previous_center) = self.previous_center {
            let new_previous_center = self.center;
            self.center = Some([
                Self::move_to_posidtion_1d(position[0], previous_center[0], self.walking_window[0]),
                Self::move_to_posidtion_1d(position[1], previous_center[1], self.walking_window[1]),
                Self::move_to_posidtion_1d(position[2], previous_center[2], self.walking_window[2]),
            ]);
            self.previous_center = new_previous_center;
        } else {
            self.previous_center = self.center;
            self.center = Some(position);
        }
    }

    fn within_distance_1d(first: i32, second: i32, distance: usize) -> bool {
        (first - second).abs() <= distance as i32
    }

    fn within_distance_3d(first: [i32; 3], second: [i32; 3], distance: [usize; 3]) -> bool {
        Self::within_distance_1d(first[0], second[0], distance[0])
            && Self::within_distance_1d(first[1], second[1], distance[1])
            && Self::within_distance_1d(first[2], second[2], distance[2])
    }

    fn outside_distance_3d(first: [i32; 3], second: [i32; 3], distance: [usize; 3]) -> bool {
        !Self::within_distance_3d(first, second, distance)
    }

    fn delete_obsolete(&mut self) {
        if let (Some(previous_center), Some(center)) = (self.previous_center, self.center) {
            let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
            let previous_center_index = Self::position_to_chunk_index_3d(previous_center, chunk_length);
            for z in previous_center_index[2] - self.world_size_in_chunks_radius[2] as i32
                ..previous_center_index[2] + self.world_size_in_chunks_radius[2] as i32 + 1
            {
                for y in previous_center_index[1] - self.world_size_in_chunks_radius[1] as i32
                    ..previous_center_index[1] + self.world_size_in_chunks_radius[1] as i32 + 1
                {
                    for x in previous_center_index[0] - self.world_size_in_chunks_radius[0] as i32
                        ..previous_center_index[0] + self.world_size_in_chunks_radius[0] as i32 + 1
                    {
                        let center_index = Self::position_to_chunk_index_3d(center, chunk_length);
                        if Self::outside_distance_3d(center_index, [x, y, z], self.world_size_in_chunks_radius) {
                            if let Some(chunk) = self.chunks.get([x, y, z]) {
                                if chunk.location == [x, y, z] {
                                    self.chunks.set([x, y, z], None);
                                    if let Some(mesh_handle) = self.meshes.get([x, y, z]) {
                                        self.mesh_storage.remove(mesh_handle);
                                        self.meshes.set([x, y, z], None);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn request_new(&mut self, asset_loader: &mut AssetLoader) {
        match (self.previous_center, self.center) {
            (None, Some(center)) => {
                let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
                let center_index = Self::position_to_chunk_index_3d(center, chunk_length);
                for z in center_index[2] - self.world_size_in_chunks_radius[2] as i32
                    ..center_index[2] + self.world_size_in_chunks_radius[2] as i32 + 1
                {
                    for y in center_index[1] - self.world_size_in_chunks_radius[1] as i32
                        ..center_index[1] + self.world_size_in_chunks_radius[1] as i32 + 1
                    {
                        for x in center_index[0] - self.world_size_in_chunks_radius[0] as i32
                            ..center_index[0] + self.world_size_in_chunks_radius[0] as i32 + 1
                        {
                            asset_loader.request(Command::Load(x, y, z));
                            self.chunks.set([x, y, z], Some(Chunk::new(x, y, z)));
                        }
                    }
                }
            }
            (Some(previous_center), Some(center)) => {
                let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
                let previous_center_index = Self::position_to_chunk_index_3d(previous_center, chunk_length);
                let center_index = Self::position_to_chunk_index_3d(center, chunk_length);
                if center_index != previous_center_index {
                    println!("prev: {:?}, current: {:?}", previous_center_index, center_index);
                    for z in center_index[2] - self.world_size_in_chunks_radius[2] as i32
                        ..center_index[2] + self.world_size_in_chunks_radius[2] as i32 + 1
                    {
                        for y in center_index[1] - self.world_size_in_chunks_radius[1] as i32
                            ..center_index[1] + self.world_size_in_chunks_radius[1] as i32 + 1
                        {
                            for x in center_index[0] - self.world_size_in_chunks_radius[0] as i32
                                ..center_index[0] + self.world_size_in_chunks_radius[0] as i32 + 1
                            {
                                if !Self::within_distance_3d(
                                    previous_center_index,
                                    [x, y, z],
                                    self.world_size_in_chunks_radius,
                                ) {
                                    println!("{:?}", [x, y, z]);
                                    asset_loader.request(Command::Load(x, y, z));
                                    self.chunks.set([x, y, z], Some(Chunk::new(x, y, z)));
                                }
                            }
                        }
                    }
                }
            }
            (None, None) => {
                assert!(false);
            }
            (Some(_), None) => {
                assert!(false);
            }
        }
    }

    fn retrieve_new(&mut self, asset_loader: &mut AssetLoader, renderer: &mut Renderer) {
        if let Some(center) = self.center {
            let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
            let center_index = Self::position_to_chunk_index_3d(center, chunk_length);
            if let Some((mesh_data, transform, location)) = asset_loader.try_retrieve() {
                if Self::within_distance_3d(center_index, location, self.world_size_in_chunks_radius) {
                    if let Some(chunk) = self.chunks.get(location) {
                        if chunk.location == location && chunk.requested {
                            let mesh_handle = self.mesh_storage.add(Mesh::from_mesh_data(renderer, mesh_data));
                            self.meshes.set(location, Some(mesh_handle));
                            self.chunks.set(
                                location,
                                Some(Chunk {
                                    location,
                                    transform,
                                    requested: true,
                                }),
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self, position: [f32; 3], asset_loader: &mut AssetLoader, renderer: &mut Renderer) {
        self.update_center(position);
        self.request_new(asset_loader);
        self.delete_obsolete();
        self.retrieve_new(asset_loader, renderer);
    }
}
