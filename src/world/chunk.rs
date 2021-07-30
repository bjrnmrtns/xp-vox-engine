use crate::{physics::PhysicsHandle, registry::Handle, renderer::Mesh, transform::Transform};

#[derive(Clone)]
pub struct ChunkData {
    pub physics_handle: PhysicsHandle,
    pub mesh_handle: Handle<Mesh>,
    pub transform: Transform,
}

#[derive(Clone)]
pub struct Chunk {
    pub location: [i32; 2],
    pub chunk_data: Vec<ChunkData>,
    pub requested: bool,
}
