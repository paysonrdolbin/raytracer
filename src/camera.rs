use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub look_at: Vec3,
    pub look_from: Vec3,
    pub up: Vec3,
    pub focal_length: f64,
    pub sensor_height: f64,
    pub aspectRatio: f64,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}
impl Camera {
    pub fn new( look_at: Vec3,
                look_from: Vec3,
                up: Vec3,
                focal_length: f64,
                sensor_height: f64,
                aspectRatio: f64
    ) -> Camera {
        let h = sensor_height / (2.0 * focal_length);
        let viewport_height = 1.0 * h;
        let viewport_width = aspectRatio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            look_at,
            look_from,
            up,
            focal_length,
            sensor_height,
            aspectRatio,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.look_from,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
