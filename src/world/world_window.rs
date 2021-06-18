use std::{collections::HashSet, iter::FromIterator};

pub struct WorldWindow {
    previous_chunks: Option<HashSet<[i32; 3]>>,
    center: [f32; 3],
    inner_size: [f32; 3],
    voxel_size: f32,
    chunk_size_in_voxels: u32,
    view_size: [f32; 3],
}

impl WorldWindow {
    pub fn new(
        position: [f32; 3],
        inner_size: [f32; 3],
        voxel_size: f32,
        chunk_size_in_voxels: u32,
        view_size: [f32; 3],
    ) -> Self {
        Self {
            previous_chunks: None,
            center: position,
            inner_size,
            voxel_size,
            chunk_size_in_voxels,
            view_size,
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

    /*    pub fn get_chunks_within(&mut self) -> (HashSet<[i32; 3]>) {
        let chunk_length = self.chunk_size as f32 * self.voxel_size;
        let x_range = Self::get_chunks_with_1d(self.center[0], self.view_size[0], chunk_length);
        let y_range = Self::get_chunks_with_1d(self.center[1], self.view_size[1], chunk_length);
        let z_range = Self::get_chunks_with_1d(self.center[2], self.view_size[2], chunk_length);
        let within_region = (z_range.0..=z_range.1)
            .flat_map(|z| (y_range.0..=y_range.1).flat_map(move |y| (x_range.0..=x_range.1).map(move |x| [x, y, z])))
            .collect::<HashSet<_>>();
        let (added, deleted) = if let Some(previous_chunks) = self.previous_chunks.clone() {
            let deleted = previous_chunks.difference(&within_region);
            let added = within_region.difference(&previous_chunks);
            (added.collect(), deleted.collect())
        } else {
            (HashSet::new(), HashSet::new())
        };
        self.previous_chunks = Some(within_region.clone());
        (added)
    }
    */
}
/*
    #[cfg(test)]
    mod tests {
        use crate::world::world_window::WorldWindow;

        #[test]
        fn move_to_position_test() {
            let mut window = WorldWindow::new([0.0, 0.0, 0.0], [4.0, 4.0, 4.0], 0.1, 32, [0.0, 0.0, 0.0]);
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

        #[test]
        fn get_chunks_with_1d_test() {
            let range = WorldWindow::get_chunks_with_1d(-1.1, 4.0, 1.0);
            assert_eq!(range.0, -4);
            assert_eq!(range.1, 0);
        }
        #[test]
        fn get_chunks_within() {
            let window = WorldWindow::new([0.0, 0.0, 0.0], [4.0, 4.0, 4.0], 0.1, 32, [12.9, 12.9, 12.9]);
            let chunks = window.get_chunks_within();
            let mut i = 0;
            for chunk in chunks {
                i = i + 1;
                println!("{} {:?}", i, chunk);
            }
        }
}
        */
