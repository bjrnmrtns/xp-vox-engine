use crate::transform::Transform;

pub trait Vox {
    fn get_size(&self) -> [usize; 3];
    fn get(&self, x: usize, y: usize, z: usize) -> Option<u8>;
    fn get_color(&self, color_id: u8) -> [f32; 3];
    fn get_y_min_offset(&self) -> f32;
    fn get_y_max_offset(&self) -> f32;
}
