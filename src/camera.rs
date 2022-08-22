use crate::math::*;




pub struct Camera {
    origin: Point,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
    aspect_ratio: f64,
    view_dir: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_width: f64, origin: Vec3, look_at: Vec3) -> Camera {
        Camera {
            origin,
            focal_length: 1.0,
            viewport_width,
            viewport_height: aspect_ratio * viewport_width,
            aspect_ratio,
            view_dir: (look_at - origin).normalized(),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        
        let adjusted_v = (self.viewport_height / 2.0) - (self.viewport_height * v);
        let adjusted_u = (-self.viewport_width / 2.0) - (-self.viewport_width * u);
        
        let right = (self.view_dir + Vec3::new(0.0, 1.0, 0.0)).cross(&self.view_dir);
        let up = self.view_dir.cross(&right);

        let dir = Vec3::new(
            adjusted_u,
            adjusted_v,
            1.0
        ).normalized();

        let dir = dir.x() * right + dir.y() * up + dir.z() * self.view_dir;

        Ray::new(self.origin, dir)
    }
}