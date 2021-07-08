use crate::{mesh::MeshData, physics::CollisionShape, registry::Handle, renderer::Mesh, transform::Transform};

pub struct Entity {
    pub mesh_handle: Handle<Mesh>,
    pub collision_shape: Option<CollisionShape>,
    pub transform: Transform,
}
