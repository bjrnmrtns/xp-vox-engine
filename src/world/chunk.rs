use crate::transform::Transform;

#[derive(Clone)]
pub struct Chunk {
    pub location: [i32; 2],
    pub transform: Transform,
    pub requested: bool,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            location: [x, z],
            transform: Transform::identity(),
            requested: true,
        }
    }
}
