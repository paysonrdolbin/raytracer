use crate::hittable::Hittable;
use crate::hittable::HittableList;
use crate::lights::Light;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn ray_color(ray: &Ray, world: &HittableList, lights: &[Light], max_depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        let view_dir = ray.direction.negate().unit_vector();
        return hit.material.shade(&hit, world, lights, view_dir, max_depth);
    } else{
        return Vec3::new(0.2, 0.2, 0.2); // background color
    }
}