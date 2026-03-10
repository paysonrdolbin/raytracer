use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Triangle {
    v1: Vec3,
    v2: Vec3,
    v3: Vec3,
    material: Box<dyn Material>,
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, material: Box<dyn Material>) -> Self {
        Self { v1, v2, v3, material }
    }
}

impl Hittable for Triangle {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        const EPSILON: f64 = 0.0000001;
        let edge1 = self.v2 - self.v1;
        let edge2 = self.v3 - self.v1;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -EPSILON && a < EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v1;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(q);

        if t > t_min && t < t_max {
            let point = ray.at(t);
            let normal = edge1.cross(edge2).unit_vector();
            let mut rec = HitRecord {
                t,
                point,
                normal,
                front_face: false, // We'll set this properly next
                material: &*self.material,
            };
            rec.set_face_normal(ray, normal);
            Some(rec)
        } else {
            None
        }
    }
}