use crate::{mesh::MeshData, physics::CollisionShape, registry::Handle, transform::Transform};

pub struct Entity {
    pub mesh_handle: Handle<MeshData>,
    pub collision_shape: Option<CollisionShape>,
    pub transform: Transform,
}
