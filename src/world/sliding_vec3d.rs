pub struct Vec3dSliding<T> {
    data: Vec<T>,
    size: [usize; 3],
    max: [i32; 3],
}

impl<T: Default + Clone> Vec3dSliding<T> {
    pub fn new(size: [usize; 3]) -> Self {
        Self {
            data: vec![T::default(); size[0] * size[1] * size[2]],
            size,
            max: [
                i32::MAX / (size[0] as i32 * 2) * size[0] as i32,
                i32::MAX / (size[1] as i32 * 2) * size[1] as i32,
                i32::MAX / (size[2] as i32 * 2) * size[2] as i32,
            ],
        }
    }

    fn slide_position(&self, pos: i32, index: usize) -> usize {
        (self.max[index] + pos) as usize % self.size[index]
    }

    pub fn set(&mut self, pos: [i32; 3], value: T) {
        let x = self.slide_position(pos[0], 0);
        let y = self.slide_position(pos[1], 1);
        let z = self.slide_position(pos[2], 2);
        self.data[z * self.size[1] * self.size[0] + y * self.size[0] + x] = value;
    }

    pub fn get(&self, pos: [i32; 3]) -> T {
        let x = self.slide_position(pos[0], 0);
        let y = self.slide_position(pos[1], 1);
        let z = self.slide_position(pos[2], 2);
        self.data[z * self.size[1] * self.size[0] + y * self.size[0] + x].clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::world::sliding_vec3d::Vec3dSliding;

    #[test]
    fn sliding_position_test() {
        let mut slid_win: Vec3dSliding<i32> = Vec3dSliding::new([1, 1, 1]);
        assert_eq!(0, slid_win.slide_position(0, 0));
        assert_eq!(0, slid_win.slide_position(1, 0));
        assert_eq!(0, slid_win.slide_position(2, 0));
        assert_eq!(0, slid_win.slide_position(-1, 0));
    }

    #[test]
    fn sliding_position_test2() {
        let mut slid_win: Vec3dSliding<i32> = Vec3dSliding::new([2, 2, 2]);
        assert_eq!(0, slid_win.slide_position(0, 0));
        assert_eq!(1, slid_win.slide_position(1, 0));
        assert_eq!(0, slid_win.slide_position(2, 0));
        assert_eq!(1, slid_win.slide_position(-1, 0));
    }

    #[test]
    fn slide_set_test() {
        let mut slid_win = Vec3dSliding::new([5, 5, 5]);
        let pos = [0, 0, 0];
        slid_win.set(pos, 3);
        let pos = [100, 100, 0];
        assert_eq!(3, slid_win.get(pos));
        slid_win.set(pos, 8);
        assert_eq!(8, slid_win.get(pos));
    }
}
