use glam::Vec3;

pub struct WorldWindow {
    center: [f32; 3],
    inner_size: [f32; 3],
    voxel_size: f32,
    chunk_size: u32,
}

impl WorldWindow {
    pub fn new(position: [f32; 3], inner_size: [f32; 3], voxel_size: f32, chunk_size: u32) -> Self {
        Self {
            center: position,
            inner_size,
            voxel_size,
            chunk_size,
        }
    }

    pub fn get_center(&self) -> [f32; 3] {
        self.center
    }

    fn move_to_posidtion_1d(position: f32, center: f32, inner_size: f32) -> f32 {
        let offset = position - center;
        if offset < -inner_size / 2.0 {
            position + inner_size / 2.0
        } else if offset > inner_size / 2.0 {
            position - inner_size / 2.0
        } else {
            center
        }
    }

    fn position_to_chunk_index_1d(position: f32, chunk_length: f32) -> i32 {
        (position / chunk_length).floor() as i32
    }

    fn get_chunks_with_1d(center: f32, view_size: f32, chunk_length: f32) -> (i32, i32) {
        let left = center - view_size / 2.0;
        let right = center + view_size / 2.0;
        (
            Self::position_to_chunk_index_1d(left, chunk_length),
            Self::position_to_chunk_index_1d(right, chunk_length),
        )
    }

    pub fn move_to_position(&mut self, position: [f32; 3]) {
        self.center = [
            Self::move_to_posidtion_1d(position[0], self.center[0], self.inner_size[0]),
            Self::move_to_posidtion_1d(position[1], self.center[1], self.inner_size[1]),
            Self::move_to_posidtion_1d(position[2], self.center[2], self.inner_size[2]),
        ];
    }

    pub fn get_chunks_within(&self, view_size: [f32; 3]) -> Vec<[i32; 3]> {
        let mut output = Vec::new();
        let chunk_length = self.chunk_size as f32 * self.voxel_size;
        let x_range = Self::get_chunks_with_1d(self.center[0], view_size[0], chunk_length);
        let y_range = Self::get_chunks_with_1d(self.center[1], view_size[1], chunk_length);
        let z_range = Self::get_chunks_with_1d(self.center[2], view_size[2], chunk_length);
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::world::world_window::WorldWindow;
    use glam::{IVec3, Vec3};

    #[test]
    fn world_window_test() {
        let mut window = WorldWindow::new([0.0, 0.0, 0.0], [4.0, 4.0, 4.0], 0.1, 32);
        window.move_to_position([-3.0, -3.0, -3.0]);
        let center = window.get_center();
        assert_eq!(center[0], -1.0);
        window.move_to_position([3.0, -3.0, -3.0]);
        let center = window.get_center();
        assert_eq!(center[0], 1.0);
        window.move_to_position([2.0, -3.0, -3.0]);
        let center = window.get_center();
        assert_eq!(center[0], 1.0);
    }
}
