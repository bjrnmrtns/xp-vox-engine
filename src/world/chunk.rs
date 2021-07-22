use crate::{physics::PhysicsHandle, transform::Transform};

#[derive(Clone)]
pub struct Chunk {
    pub location: [i32; 2],
    pub physics_handle: PhysicsHandle,
    pub transform: Transform,
    pub requested: bool,
}
