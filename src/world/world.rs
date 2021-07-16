use crate::{
    registry::{Handle, Registry},
    renderer::{Mesh, Renderer},
    transform::Transform,
    world::{chunk::Chunk, sliding_vec3d::Vec2dSliding, Chunker},
};

pub struct ChunkArea {
    center: [i32; 2],
    next: [i32; 2],
    radius: i32,
}

impl ChunkArea {
    pub fn new(center: [i32; 2], radius: i32) -> Self {
        let next = [center[0] - radius, center[1] - radius];
        Self { center, next, radius }
    }
}

impl Iterator for ChunkArea {
    type Item = [i32; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next = if self.next[0] < self.center[0] + self.radius + 1 {
            let x = self.next[0] + 1;
            let y = self.next[1];
            [x, y]
        } else {
            let x = self.center[0] - self.radius;
            let y = self.next[1] + 1;
            [x, y]
        };
        if current[1] < self.center[1] + self.radius + 1 {
            Some(current)
        } else {
            None
        }
    }
}

pub struct World {
    chunker: Chunker,
    chunks: Vec2dSliding<Option<Chunk>>,
    mesh_handles: Vec2dSliding<Option<Handle<Mesh>>>,
    center: Option<[f32; 2]>,
    previous_center: Option<[f32; 2]>,
    voxel_size: f32,
    chunk_size_in_voxels: u32,
    walking_window: [f32; 2],
    radius: usize,
}

impl World {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunker: Chunker::new(32),
            chunks: Vec2dSliding::new([100, 100]),
            mesh_handles: Vec2dSliding::new([100, 100]),
            center: None,
            previous_center: None,
            voxel_size: 0.1,
            chunk_size_in_voxels: chunk_size as u32,
            walking_window: [6.0, 6.0],
            radius: 10,
        }
    }

    fn position_to_chunk_index_1d(position: f32, chunk_length: f32) -> i32 {
        (position / chunk_length).floor() as i32
    }

    fn position_to_chunk_index_2d(position: [f32; 2], chunk_length: f32) -> [i32; 2] {
        [
            Self::position_to_chunk_index_1d(position[0], chunk_length),
            Self::position_to_chunk_index_1d(position[1], chunk_length),
        ]
    }

    fn optional_position_to_chunk_index_2d(position: Option<[f32; 2]>, chunk_length: f32) -> Option<[i32; 2]> {
        if let Some(position) = position {
            Some([
                Self::position_to_chunk_index_1d(position[0], chunk_length),
                Self::position_to_chunk_index_1d(position[1], chunk_length),
            ])
        } else {
            None
        }
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

    fn update_center_2d(&mut self, position: [f32; 2]) {
        if let Some(previous_center) = self.previous_center {
            let new_previous_center = self.center;
            self.center = Some([
                Self::move_to_posidtion_1d(position[0], previous_center[0], self.walking_window[0]),
                Self::move_to_posidtion_1d(position[1], previous_center[1], self.walking_window[1]),
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

    fn within_distance_2d(first: [i32; 2], second: [i32; 2], distance: usize) -> bool {
        Self::within_distance_1d(first[0], second[0], distance)
            && Self::within_distance_1d(first[1], second[1], distance)
    }

    fn optional_within_distance_2d(first: Option<[i32; 2]>, second: [i32; 2], distance: usize) -> bool {
        if let Some(first) = first {
            Self::within_distance_1d(first[0], second[0], distance)
                && Self::within_distance_1d(first[1], second[1], distance)
        } else {
            false
        }
    }

    fn within_distance_3d(first: [i32; 3], second: [i32; 3], distance: usize) -> bool {
        Self::within_distance_1d(first[0], second[0], distance)
            && Self::within_distance_1d(first[1], second[1], distance)
            && Self::within_distance_1d(first[2], second[2], distance)
    }

    fn outside_distance_2d(first: [i32; 2], second: [i32; 2], distance: usize) -> bool {
        !Self::within_distance_2d(first, second, distance)
    }

    fn outside_distance_3d(first: [i32; 3], second: [i32; 3], distance: usize) -> bool {
        !Self::within_distance_3d(first, second, distance)
    }

    fn delete_obsolete(&mut self, meshes: &mut Registry<Mesh>) {
        if let (Some(previous_center), Some(center)) = (self.previous_center, self.center) {
            let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
            let previous_center_index = Self::position_to_chunk_index_2d(previous_center, chunk_length);
            for chunk_pos in ChunkArea::new(previous_center_index, self.radius as i32) {
                let center_index = Self::position_to_chunk_index_2d(center, chunk_length);
                if Self::outside_distance_2d(center_index, chunk_pos, self.radius) {
                    if let Some(chunk) = self.chunks.get(chunk_pos) {
                        if chunk.location == chunk_pos {
                            self.chunks.set(chunk_pos, None);
                            if let Some(mesh_handle) = self.mesh_handles.get(chunk_pos) {
                                meshes.remove(mesh_handle);
                                self.mesh_handles.set(chunk_pos, None);
                            }
                        }
                    }
                }
            }
        }
    }

    fn generate_new(&mut self, meshes: &mut Registry<Mesh>, renderer: &mut Renderer) {
        let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
        let center_index = Self::optional_position_to_chunk_index_2d(self.center, chunk_length);
        let previous_center_index = Self::optional_position_to_chunk_index_2d(self.previous_center, chunk_length);
        if center_index != previous_center_index {
            println!("prev: {:?}, current: {:?}", previous_center_index, center_index);
            if let Some(center_index) = center_index {
                for chunk_pos in ChunkArea::new(center_index, self.radius as i32) {
                    if !Self::optional_within_distance_2d(previous_center_index, chunk_pos, self.radius) {
                        println!("{:?}", chunk_pos);
                        let (mesh_data, transform) = self.chunker.generate_base_chunk(chunk_pos);
                        let mesh_handle = meshes.add(Mesh::from_mesh_data(renderer, mesh_data));
                        self.mesh_handles.set(chunk_pos, Some(mesh_handle));
                        self.chunks.set(
                            chunk_pos,
                            Some(Chunk {
                                location: chunk_pos,
                                transform,
                                requested: true,
                            }),
                        );
                    }
                }
            }
        }
    }

    pub fn update(&mut self, position: [f32; 3], renderer: &mut Renderer, meshes: &mut Registry<Mesh>) {
        self.update_center_2d([position[0], position[2]]);
        self.delete_obsolete(meshes);
        self.generate_new(meshes, renderer);
    }

    pub fn get_within_view_mesh_transform(&self, position: [f32; 2]) -> Vec<(Handle<Mesh>, Transform)> {
        let mut mesh_transforms = Vec::new();
        let chunk_length = self.voxel_size * self.chunk_size_in_voxels as f32;
        let position_index = Self::position_to_chunk_index_2d(position, chunk_length);
        for chunk_pos in ChunkArea::new(position_index, self.radius as i32) {
            if let (Some(mesh_handle), Some(chunk)) = (self.mesh_handles.get(chunk_pos), self.chunks.get(chunk_pos)) {
                mesh_transforms.push((mesh_handle, chunk.transform));
            }
        }
        mesh_transforms
    }
}
