use crate::transform::Transform;

pub trait Vox {
    fn get_size(&self) -> [usize; 3];
    fn get(&self, x: usize, y: usize, z: usize) -> Option<u8>;
    fn get_color(&self, color_id: u8) -> [f32; 3];
    fn get_y_offset(&self) -> f32;
}
