use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();

            let temp = (-b - sqrtd) / a;
            if temp < t_max && temp > t_min {
                let point = ray.at(temp);
                let normal = (point - self.center) * (1.0 / self.radius);
                let mut hit_record = HitRecord { point, normal, t: temp, front_face: false, material: &*self.material };
                hit_record.set_face_normal(ray, normal);
                return Some(hit_record);
            }

            let temp = (-b + sqrtd) / a;
            if temp < t_max && temp > t_min {
                let point = ray.at(temp);
                let normal = (point - self.center) * (1.0 / self.radius);
                let mut hit_record = HitRecord { point, normal, t: temp, front_face: false, material: &*self.material };
                hit_record.set_face_normal(ray, normal);
                return Some(hit_record);
            }
        }

        None
    }
}