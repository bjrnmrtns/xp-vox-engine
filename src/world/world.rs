use crate::{
    registry::{Handle, Registry},
    renderer::{Mesh, Renderer},
    transform::Transform,
    world::{
        chunk::Chunk,
        constants::{CHUNK_SIZE_IN_METERS, CHUNK_SIZE_IN_VOXELS},
        sliding_vec3d::Vec2dSliding,
        Chunker,
    },
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
    old_center: Option<[f32; 2]>,
    walking_window: [f32; 2],
    radius: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunker: Chunker::new(),
            chunks: Vec2dSliding::new([100, 100]),
            mesh_handles: Vec2dSliding::new([100, 100]),
            old_center: None,
            walking_window: [6.0, 6.0],
            radius: 10,
        }
    }

    fn position_to_chunk_index_1d(position: f32) -> i32 {
        (position / CHUNK_SIZE_IN_METERS).floor() as i32
    }

    fn position_to_chunk_index_2d(position: [f32; 2]) -> [i32; 2] {
        [
            Self::position_to_chunk_index_1d(position[0]),
            Self::position_to_chunk_index_1d(position[1]),
        ]
    }

    fn position_to_chunk_index_3d(position: [f32; 3]) -> [i32; 3] {
        [
            Self::position_to_chunk_index_1d(position[0]),
            Self::position_to_chunk_index_1d(position[1]),
            Self::position_to_chunk_index_1d(position[2]),
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

    fn move_to_posidtion_2d(position: [f32; 2], center: [f32; 2], walking_window: [f32; 2]) -> [f32; 2] {
        [
            Self::move_to_posidtion_1d(position[0], center[0], walking_window[0]),
            Self::move_to_posidtion_1d(position[1], center[1], walking_window[1]),
        ]
    }

    fn within_distance_1d(first: i32, second: i32, distance: usize) -> bool {
        (first - second).abs() <= distance as i32
    }

    fn within_distance_2d(first: [i32; 2], second: [i32; 2], distance: usize) -> bool {
        Self::within_distance_1d(first[0], second[0], distance)
            && Self::within_distance_1d(first[1], second[1], distance)
    }

    fn outside_distance_2d(first: [i32; 2], second: [i32; 2], distance: usize) -> bool {
        !Self::within_distance_2d(first, second, distance)
    }

    fn generate_chunk(&mut self, chunk_pos: [i32; 2], meshes: &mut Registry<Mesh>, renderer: &mut Renderer) {
        let (mesh_data, transform) = self.chunker.generate_chunk(chunk_pos);
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

    fn delete_chunk(&mut self, chunk_pos: [i32; 2], meshes: &mut Registry<Mesh>) {
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

    fn delete_obsolete(&mut self, meshes: &mut Registry<Mesh>, center: [f32; 2]) {
        if let Some(previous_center) = self.old_center {
            let previous_center_index = Self::position_to_chunk_index_2d(previous_center);
            for chunk_pos in ChunkArea::new(previous_center_index, self.radius as i32) {
                let center_index = Self::position_to_chunk_index_2d(center);
                if Self::outside_distance_2d(center_index, chunk_pos, self.radius) {
                    self.delete_chunk(chunk_pos, meshes);
                }
            }
        }
    }

    fn generate_new(&mut self, meshes: &mut Registry<Mesh>, renderer: &mut Renderer, new_center: [f32; 2]) {
        let center_index = Self::position_to_chunk_index_2d(new_center);
        for chunk_pos in ChunkArea::new(center_index, self.radius as i32) {
            if let Some(old_center) = self.old_center {
                let previous_center_index = Self::position_to_chunk_index_2d(old_center);
                if previous_center_index != center_index
                    && !Self::within_distance_2d(previous_center_index, chunk_pos, self.radius)
                {
                    println!("{:?}", chunk_pos);
                    self.generate_chunk(chunk_pos, meshes, renderer);
                }
            } else {
                println!("{:?}", chunk_pos);
                self.generate_chunk(chunk_pos, meshes, renderer);
            }
        }
    }

    pub fn update(&mut self, position: [f32; 3], renderer: &mut Renderer, meshes: &mut Registry<Mesh>) {
        let center = if let Some(old_center) = self.old_center {
            Self::move_to_posidtion_2d([position[0], position[2]], old_center, self.walking_window)
        } else {
            [position[0], position[2]]
        };
        self.delete_obsolete(meshes, center);
        self.generate_new(meshes, renderer, center);
        self.old_center = Some(center);
    }

    pub fn get_within_view_mesh_transform(&self, position: [f32; 2]) -> Vec<(Handle<Mesh>, Transform)> {
        let mut mesh_transforms = Vec::new();
        let position_index = Self::position_to_chunk_index_2d(position);
        for chunk_pos in ChunkArea::new(position_index, self.radius as i32) {
            if let (Some(mesh_handle), Some(chunk)) = (self.mesh_handles.get(chunk_pos), self.chunks.get(chunk_pos)) {
                mesh_transforms.push((mesh_handle, chunk.transform));
            }
        }
        mesh_transforms
    }
}
