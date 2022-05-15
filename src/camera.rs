use glam::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub width: u32,
    pub height: u32,
    pub x_step: Vec3,
    pub y_step: Vec3,
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        position: Vec3,
        look_at: Vec3,
        vup: Vec3,       
        fov: f32, // in degrees
    ) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let theta = fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let viewport_height = 2.0 * half_height;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (position - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        
        let lower_left_corner = position - horizontal / 2.0 - vertical / 2.0 - w;
        let x_step = horizontal / width as f32;
        let y_step = vertical / height as f32;

        Self {
            width,
            height,
            position,
            vertical,
            horizontal,
            lower_left_corner,
            w,
            u,
            v,
            x_step,
            y_step,
        }
    }

}
