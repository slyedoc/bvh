use glam::Vec3;

#[derive(Default, Debug, Copy, Clone)]
pub struct Tri {
    pub vertex0: Vec3,
    pub vertex1: Vec3,
    pub vertex2: Vec3,
    pub centroid: Vec3,
}

impl Tri {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Tri {
            vertex0: v0,
            vertex1: v1,
            vertex2: v2,
            centroid: (v0 + v1 + v2) / 3.0,  // * 0.3333f32
        }
    }
}
