use glam::Vec3;

use crate::ray::Ray;

pub struct Camera {
    pub position: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32, // vertical field of view
        aspect_ratio: f32,
    ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let viewport_height = 2.0 * half_height;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Self {
            position: origin,
            vertical,
            horizontal,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
            w,
            u,
            v,
        }
    }

    pub fn get_direction(&self, u: f32, v: f32) -> Vec3 {
        (self.lower_left_corner + u * self.horizontal + v * self.vertical - self.position)
            .normalize()
    }
}
