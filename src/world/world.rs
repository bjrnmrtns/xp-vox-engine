use crate::{
    registry::{Handle, Registry},
    renderer::{Mesh, Renderer},
    transform::Transform,
    world::{sliding_vec3d::Vec3dSliding, Chunker},
};

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
    chunker: Chunker,
    chunks: Vec3dSliding<Option<Chunk>>,
    meshes: Vec3dSliding<Option<Handle<Mesh>>>,
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
            chunker: Chunker::new(32),
            chunks: Vec3dSliding::new([100, 100, 100]),
            meshes: Vec3dSliding::new([100, 100, 100]),
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

    fn delete_obsolete(&mut self, meshes: &mut Registry<Mesh>) {
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
                                        meshes.remove(mesh_handle);
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

    fn request_new(&mut self, meshes: &mut Registry<Mesh>, renderer: &mut Renderer) {
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
                            let (mesh_data, transform) = self.chunker.generate_chunk([x, y, z]);
                            if let Some(mesh_data) = mesh_data {
                                let mesh_handle = meshes.add(Mesh::from_mesh_data(renderer, mesh_data));
                                self.meshes.set([x, y, z], Some(mesh_handle));
                                self.chunks.set(
                                    [x, y, z],
                                    Some(Chunk {
                                        location: [x, y, z],
                                        transform,
                                        requested: true,
                                    }),
                                );
                            }
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
                                    let (mesh_data, transform) = self.chunker.generate_chunk([x, y, z]);
                                    if let Some(mesh_data) = mesh_data {
                                        let mesh_handle = meshes.add(Mesh::from_mesh_data(renderer, mesh_data));
                                        self.meshes.set([x, y, z], Some(mesh_handle));
                                        self.chunks.set(
                                            [x, y, z],
                                            Some(Chunk {
                                                location: [x, y, z],
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
            }
            (None, None) => {
                assert!(false);
            }
            (Some(_), None) => {
                assert!(false);
            }
        }
    }

    pub fn update(&mut self, position: [f32; 3], renderer: &mut Renderer, meshes: &mut Registry<Mesh>) {
        self.update_center(position);
        self.delete_obsolete(meshes);
        self.request_new(meshes, renderer);
    }

    pub fn get_within_view_mesh_transform(&self, position: [f32; 3]) -> Vec<(Handle<Mesh>, Transform)> {
        let mut mesh_transforms = Vec::new();
        let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
        let position_index = Self::position_to_chunk_index_3d(position, chunk_length);
        for z in position_index[2] - self.world_size_in_chunks_radius[2] as i32
            ..position_index[2] + self.world_size_in_chunks_radius[2] as i32 + 1
        {
            for y in position_index[1] - self.world_size_in_chunks_radius[1] as i32
                ..position_index[1] + self.world_size_in_chunks_radius[1] as i32 + 1
            {
                for x in position_index[0] - self.world_size_in_chunks_radius[0] as i32
                    ..position_index[0] + self.world_size_in_chunks_radius[0] as i32 + 1
                {
                    if let (Some(mesh_handle), Some(chunk)) = (self.meshes.get([x, y, z]), self.chunks.get([x, y, z])) {
                        mesh_transforms.push((mesh_handle, chunk.transform));
                    }
                }
            }
        }
        mesh_transforms
    }
}
