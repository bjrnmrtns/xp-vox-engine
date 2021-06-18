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

    pub fn set(&mut self, x: i32, y: i32, z: i32, value: T) {
        let x = self.slide_position(x, 0);
        let y = self.slide_position(y, 1);
        let z = self.slide_position(z, 2);
        self.data[z * self.size[1] * self.size[0] + y * self.size[0] + x] = value;
    }

    pub fn get(&mut self, x: i32, y: i32, z: i32) -> T {
        let x = self.slide_position(x, 0);
        let y = self.slide_position(y, 1);
        let z = self.slide_position(z, 2);
        self.data[z * self.size[1] * self.size[0] + y * self.size[0] + x].clone()
    }
}
