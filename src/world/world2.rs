use crate::{mesh::Mesh, registry::Handle, world::sliding_vec3d::Vec3dSliding};

#[derive(Clone)]
pub struct Chunk {
    location: [i32; 3],
    mesh_handle: Option<Handle<Mesh>>,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            location: [x, y, z],
            mesh_handle: None,
        }
    }
}

pub struct World {
    chunks: Vec3dSliding<Option<Chunk>>,
    position: Option<[f32; 3]>,
    center: Option<[f32; 3]>,
    previous_center: Option<[f32; 3]>,
    voxel_size: f32,
    chunk_size_in_voxels: u32,
    walking_window: [f32; 3],
    world_size_in_chunks_radius: [usize; 3],
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: Vec3dSliding::new([100, 100, 100]),
            position: None,
            center: None,
            previous_center: None,
            voxel_size: 0.1,
            chunk_size_in_voxels: 16,
            walking_window: [10.0, 10.0, 10.0],
            world_size_in_chunks_radius: [10, 10, 10],
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

    pub fn set_position(&mut self, position: [f32; 3]) {
        if let Some(previous_center) = self.center {
            self.center = Some([
                Self::move_to_posidtion_1d(position[0], previous_center[0], self.walking_window[0]),
                Self::move_to_posidtion_1d(position[1], previous_center[1], self.walking_window[1]),
                Self::move_to_posidtion_1d(position[2], previous_center[2], self.walking_window[2]),
            ]);
        } else {
            self.center = Some(position);
        }
        self.position = Some(position);
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

    pub fn delete_obsolete(&mut self /*vertex buffers / meshes / entities to delete*/) {
        if let (Some(previous_center), Some(center)) = (self.previous_center, self.center) {
            let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
            let chunk_index = Self::position_to_chunk_index_3d(previous_center, chunk_length);
            for z in chunk_index[2] - self.world_size_in_chunks_radius[2] as i32
                ..chunk_index[2] + self.world_size_in_chunks_radius[2] as i32
            {
                for y in chunk_index[1] - self.world_size_in_chunks_radius[1] as i32
                    ..chunk_index[1] + self.world_size_in_chunks_radius[1] as i32
                {
                    for x in chunk_index[0] - self.world_size_in_chunks_radius[0] as i32
                        ..chunk_index[0] + self.world_size_in_chunks_radius[0] as i32
                    {
                        let center_index = Self::position_to_chunk_index_3d(center, chunk_length);
                        if Self::outside_distance_3d(center_index, [x, y, z], self.world_size_in_chunks_radius) {
                            self.chunks.set(chunk_index, None);
                        }
                    }
                }
            }
        }
    }

    pub fn generate_new(&mut self) {
        if let Some(center) = self.center {
            let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
            let chunk_index = Self::position_to_chunk_index_3d(center, chunk_length);
            for z in chunk_index[2] - self.world_size_in_chunks_radius[2] as i32
                ..chunk_index[2] + self.world_size_in_chunks_radius[2] as i32
            {
                for y in chunk_index[1] - self.world_size_in_chunks_radius[1] as i32
                    ..chunk_index[1] + self.world_size_in_chunks_radius[1] as i32
                {
                    for x in chunk_index[0] - self.world_size_in_chunks_radius[0] as i32
                        ..chunk_index[0] + self.world_size_in_chunks_radius[0] as i32
                    {
                        let center_index = Self::position_to_chunk_index_3d(previous_center, chunk_length);
                        if let Some(previous_center) = self.previous_center {
                            if Self::outside_distance_3d(center_index, [x, y, z], self.world_size_in_chunks_radius) {
                                self.chunks.set(chunk_index, Some(Chunk::new(x, y, z)));
                            }
                        } else {
                            self.chunks.set(chunk_index, Some(Chunk::new(x, y, z)));
                        }
                    }
                }
            }
        }
    }
}
