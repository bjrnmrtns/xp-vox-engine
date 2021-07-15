use crate::{
    mesh::{MeshData, Vertex},
    world::{vox3d::Vox3d, voxHeightMap::VoxHeightMap},
};

struct Descriptor {
    pub u: usize,
    pub v: usize,
    pub w: usize,
    pub step: i32,
    pub normal: [i32; 3],
    pub q: [i32; 3],
}

impl Descriptor {
    pub fn new(u: usize, v: usize, w: usize, step: i32, normal: [i32; 3], q: [i32; 3]) -> Self {
        Self {
            u,
            v,
            w,
            step,
            normal,
            q,
        }
    }
}

struct Mask {
    data: Vec<Option<u8>>,
    size_x: usize,
    size_y: usize,
}

impl Mask {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            data: vec![None; size_y * size_x],
            size_x,
            size_y,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color_id: Option<u8>) {
        assert!(x < self.size_x);
        assert!(y < self.size_y);
        self.data[y * self.size_x + x] = color_id;
    }

    pub fn get(&mut self, x: usize, y: usize) -> Option<u8> {
        self.data[y * self.size_x + x]
    }
}

pub fn greedy_mesh(vox: &Vox3d) -> Option<MeshData> {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let descriptors = [
        Descriptor::new(0, 1, 2, 1, [1, 0, 0], [0, 0, 0]),
        Descriptor::new(0, 1, 2, -1, [-1, 0, 0], [1, 0, 0]),
        Descriptor::new(1, 2, 0, 1, [0, 1, 0], [0, 0, 0]),
        Descriptor::new(1, 2, 0, -1, [0, -1, 0], [1, 0, 0]),
        Descriptor::new(2, 0, 1, 1, [0, 0, 1], [0, 0, 0]),
        Descriptor::new(2, 0, 1, -1, [0, 0, -1], [1, 0, 0]),
    ];

    let vox_size = [vox.x_size, vox.y_size, vox.z_size];

    for d in descriptors.iter() {
        let u = d.u;
        let v = d.v;
        let w = d.w;
        let normal = d.normal;
        let normal_outside = [-(normal[0] as f32), -(normal[1] as f32), -(normal[2] as f32)];

        for slice in 0..vox_size[u] {
            let slice = if d.step == 1 { slice } else { vox_size[u] - (slice + 1) };
            let mut cursor = [0, 0, 0];
            let no_voxel_back = (slice == 0 && d.step == 1) || (slice == vox_size[u] - 1 && d.step != 1);
            cursor[u] = slice;
            let mut mask = Mask::new(vox_size[v], vox_size[w]);
            for cursor_w in 0..vox_size[w] {
                for cursor_v in 0..vox_size[v] {
                    cursor[v] = cursor_v;
                    cursor[w] = cursor_w;
                    let voxel_back = if !no_voxel_back {
                        vox.get(
                            (cursor[0] as i32 - normal[0]) as usize,
                            (cursor[1] as i32 - normal[1]) as usize,
                            (cursor[2] as i32 - normal[2]) as usize,
                        )
                    } else {
                        None
                    };
                    let voxel = vox.get(cursor[0], cursor[1], cursor[2]);
                    let color_id = if voxel_back != None && voxel != None && voxel_back == voxel {
                        None
                    } else {
                        voxel
                    };
                    mask.set(cursor[v], cursor[w], color_id);
                }
            }
            for y in 0..vox_size[w] {
                for x in 0..vox_size[v] {
                    let color_id = mask.get(x, y);
                    if let Some(m) = color_id {
                        let mut width = 1;
                        while x + width < vox_size[v] && mask.get(x + width, y) == color_id {
                            width += 1;
                        }
                        let mut height = 1;
                        let mut done = false;
                        while y + height < vox_size[w] && !done {
                            let mut k = 0;
                            while k < width && !done {
                                if mask.get(x + k, y + height) == color_id {
                                    k += 1;
                                } else {
                                    done = true;
                                }
                            }
                            if !done {
                                height += 1;
                            }
                        }
                        let mut base = [0.0, 0.0, 0.0];
                        base[u] = slice as f32 / 10.0 + d.q[0] as f32 / 10.0;
                        base[v] = x as f32 / 10.0 + d.q[1] as f32 / 10.0;
                        base[w] = y as f32 / 10.0 + d.q[2] as f32 / 10.0;

                        let mut dv = [0.0, 0.0, 0.0];
                        dv[v] = width as f32 / 10.0;
                        let mut dw = [0.0, 0.0, 0.0];
                        dw[w] = height as f32 / 10.0;

                        let color = vox.get_color(m);
                        let count = vertices.len() as u32;
                        vertices.extend_from_slice(&[
                            Vertex::new([base[0], base[1], base[2]], normal_outside, color),
                            Vertex::new(
                                [
                                    base[0] + dv[0] + dw[0],
                                    base[1] + dv[1] + dw[1],
                                    base[2] + dv[2] + dw[2],
                                ],
                                normal_outside,
                                color,
                            ),
                            Vertex::new(
                                [base[0] + dv[0], base[1] + dv[1], base[2] + dv[2]],
                                normal_outside,
                                color,
                            ),
                            Vertex::new(
                                [base[0] + dw[0], base[1] + dw[1], base[2] + dw[2]],
                                normal_outside,
                                color,
                            ),
                        ]);
                        if d.step == 1 {
                            indices.extend_from_slice(&[count, count + 1, count + 2, count, count + 3, count + 1]);
                        } else {
                            indices.extend_from_slice(&[count, count + 2, count + 1, count, count + 1, count + 3]);
                        }
                        for yy in y..y + height {
                            for xx in x..x + width {
                                mask.set(xx, yy, None);
                            }
                        }
                    }
                }
            }
        }
    }
    if vox.touched {
        Some(MeshData { vertices, indices })
    } else {
        None
    }
}

pub fn greedy_mesh_base(vox: &VoxHeightMap) -> Option<MeshData> {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let descriptors = [
        Descriptor::new(0, 1, 2, 1, [1, 0, 0], [0, 0, 0]),
        Descriptor::new(0, 1, 2, -1, [-1, 0, 0], [1, 0, 0]),
        Descriptor::new(1, 2, 0, 1, [0, 1, 0], [0, 0, 0]),
        Descriptor::new(1, 2, 0, -1, [0, -1, 0], [1, 0, 0]),
        Descriptor::new(2, 0, 1, 1, [0, 0, 1], [0, 0, 0]),
        Descriptor::new(2, 0, 1, -1, [0, 0, -1], [1, 0, 0]),
    ];

    let vox_size = [vox.x_size, 32, vox.z_size];

    for d in descriptors.iter() {
        let u = d.u;
        let v = d.v;
        let w = d.w;
        let normal = d.normal;
        let normal_outside = [-(normal[0] as f32), -(normal[1] as f32), -(normal[2] as f32)];

        for slice in 0..vox_size[u] {
            let slice = if d.step == 1 { slice } else { vox_size[u] - (slice + 1) };
            let mut cursor = [0, 0, 0];
            let no_voxel_back = (slice == 0 && d.step == 1) || (slice == vox_size[u] - 1 && d.step != 1);
            cursor[u] = slice;
            let mut mask = Mask::new(vox_size[v], vox_size[w]);
            for cursor_w in 0..vox_size[w] {
                for cursor_v in 0..vox_size[v] {
                    cursor[v] = cursor_v;
                    cursor[w] = cursor_w;
                    let voxel_back = if !no_voxel_back {
                        vox.get(
                            (cursor[0] as i32 - normal[0]) as usize,
                            (cursor[1] as i32 - normal[1]) as usize,
                            (cursor[2] as i32 - normal[2]) as usize,
                        )
                    } else {
                        None
                    };
                    let voxel = vox.get(cursor[0], cursor[1], cursor[2]);
                    let color_id = if voxel_back != None && voxel != None && voxel_back == voxel {
                        None
                    } else {
                        voxel
                    };
                    mask.set(cursor[v], cursor[w], color_id);
                }
            }
            for y in 0..vox_size[w] {
                for x in 0..vox_size[v] {
                    let color_id = mask.get(x, y);
                    if let Some(m) = color_id {
                        let mut width = 1;
                        while x + width < vox_size[v] && mask.get(x + width, y) == color_id {
                            width += 1;
                        }
                        let mut height = 1;
                        let mut done = false;
                        while y + height < vox_size[w] && !done {
                            let mut k = 0;
                            while k < width && !done {
                                if mask.get(x + k, y + height) == color_id {
                                    k += 1;
                                } else {
                                    done = true;
                                }
                            }
                            if !done {
                                height += 1;
                            }
                        }
                        let mut base = [0.0, 0.0, 0.0];
                        base[u] = slice as f32 / 10.0 + d.q[0] as f32 / 10.0;
                        base[v] = x as f32 / 10.0 + d.q[1] as f32 / 10.0;
                        base[w] = y as f32 / 10.0 + d.q[2] as f32 / 10.0;

                        let mut dv = [0.0, 0.0, 0.0];
                        dv[v] = width as f32 / 10.0;
                        let mut dw = [0.0, 0.0, 0.0];
                        dw[w] = height as f32 / 10.0;

                        let color = VoxHeightMap::get_color(m);
                        let count = vertices.len() as u32;
                        vertices.extend_from_slice(&[
                            Vertex::new([base[0], base[1], base[2]], normal_outside, color),
                            Vertex::new(
                                [
                                    base[0] + dv[0] + dw[0],
                                    base[1] + dv[1] + dw[1],
                                    base[2] + dv[2] + dw[2],
                                ],
                                normal_outside,
                                color,
                            ),
                            Vertex::new(
                                [base[0] + dv[0], base[1] + dv[1], base[2] + dv[2]],
                                normal_outside,
                                color,
                            ),
                            Vertex::new(
                                [base[0] + dw[0], base[1] + dw[1], base[2] + dw[2]],
                                normal_outside,
                                color,
                            ),
                        ]);
                        if d.step == 1 {
                            indices.extend_from_slice(&[count, count + 1, count + 2, count, count + 3, count + 1]);
                        } else {
                            indices.extend_from_slice(&[count, count + 2, count + 1, count, count + 1, count + 3]);
                        }
                        for yy in y..y + height {
                            for xx in x..x + width {
                                mask.set(xx, yy, None);
                            }
                        }
                    }
                }
            }
        }
    }
    Some(MeshData { vertices, indices })
}
