use crate::{mesh::Mesh, registry::Handle};

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
    chunks: Vec<Option<Chunk>>,
    position: Option<[f32; 3]>,
    center: Option<[f32; 3]>,
    previous_center: Option<[f32; 3]>,
    voxel_size: f32,
    chunk_size_in_voxels: u32,
    walking_window: [f32; 3],
    world_size_in_chunks: [usize; 3],
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: vec![],
            position: None,
            center: None,
            previous_center: None,
            voxel_size: 0.1,
            chunk_size_in_voxels: 16,
            walking_window: [10.0, 10.0, 10.0],
            world_size_in_chunks: [10, 10, 10],
        }
    }

    fn position_to_chunk_index_1d(position: f32, chunk_length: f32) -> i32 {
        (position / chunk_length).floor() as i32
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

    pub fn set(&mut self, x: i32, y: i32, z: i32, from: [f32; 3]) {
        let chunk_size = self.chunk_size_in_voxels as f32 * self.voxel_size;
        let x = Self::position_to_chunk_index_1d(from[0], chunk_size);
        let y = Self::position_to_chunk_index_1d(from[1], chunk_size);
        let z = Self::position_to_chunk_index_1d(from[2], chunk_size);
        /*        let max = i32::MAX / (self.size as i32 * 2) * self.size as i32;
               let x = (max + chunk_position[0]) as usize % self.size;
               let y = (max + chunk_position[1]) as usize % self.size;
               let z = (max + chunk_position[2]) as usize % self.size;
               self.chunks[z * self.size * self.size + y * self.size + x] = chunk;

        */
    }

    pub fn get(&mut self) {}

    // this is here for debugging purposes and should not be used otherwise
    pub fn set_center(&mut self, center: [f32; 3]) {
        self.previous_center = self.center;
        self.center = Some(center);
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

    pub fn delete_obsolete(/*vertex buffers / meshes / entities to delete*/) {}

    pub fn generate_new() {}
}
