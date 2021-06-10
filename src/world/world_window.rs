use glam::Vec3;

pub struct WorldWindow {
    center: [f32; 3],
    inner_size: [f32; 3],
    outer_size: [f32; 3],
}

impl WorldWindow {
    pub fn new(position: [f32; 3], inner_size: [f32; 3], outer_size: [f32; 3]) -> Self {
        Self {
            center: position,
            inner_size,
            outer_size,
        }
    }

    pub fn get_center(&self) -> [f32; 3] {
        self.center
    }

    pub fn update_position(&mut self, position: [f32; 3]) {
        let offset = position[0] - self.center[0];
        if offset < -self.inner_size[0] / 2.0 {
            self.center[0] = position[0] + self.inner_size[0] / 2.0;
        } else if offset > self.inner_size[0] / 2.0 {
            self.center[0] = position[0] - self.inner_size[0] / 2.0;
        }
        let offset = position[1] - self.center[1];
        if offset < -self.inner_size[1] / 2.0 {
            self.center[1] = position[1] + self.inner_size[1] / 2.0;
        } else if offset > self.inner_size[1] / 2.0 {
            self.center[1] = position[1] - self.inner_size[1] / 2.0;
        }
        let offset = position[1] - self.center[1];
        if offset < -self.inner_size[1] / 2.0 {
            self.center[1] = position[1] + self.inner_size[1] / 2.0;
        } else if offset > self.inner_size[1] / 2.0 {
            self.center[1] = position[1] - self.inner_size[1] / 2.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::world::world_window::WorldWindow;
    use glam::{IVec3, Vec3};

    #[test]
    fn world_window_test() {
        let mut window = WorldWindow::new([0.0, 0.0, 0.0], [4.0, 4.0, 4.0], [10.0, 10.0, 10.0]);
        window.update_position([-3.0, -3.0, -3.0]);
        let center = window.get_center();
        assert_eq!(center[0], -1.0);
        window.update_position([3.0, -3.0, -3.0]);
        let center = window.get_center();
        assert_eq!(center[0], 1.0);
        window.update_position([2.0, -3.0, -3.0]);
        let center = window.get_center();
        assert_eq!(center[0], 1.0);
    }
}
