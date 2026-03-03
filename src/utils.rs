use crate::hittable::Hittable;
use crate::hittable::HittableList;
use crate::lights::Light;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn ray_color(ray: &Ray, world: &HittableList, lights: &[Light], max_depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        let view_dir = ray.direction.negate().unit_vector();
        return hit.material.shade(&hit, world, lights, view_dir, max_depth);
    } else {
        // Sky gradient
        let unit_dir = ray.direction.unit_vector();
        let t = 0.5 * (unit_dir.y + 1.0);   // map y from [-1,1] to [0,1]

        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue  = Vec3::new(0.5, 0.7, 1.0);

        return (1.0 - t) * white + t * blue;
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    } else if x > max {
        return max
    }
    x
}

