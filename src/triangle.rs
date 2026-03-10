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
        let edge1 = self.v2 - self.v1;
        let edge2 = self.v3 - self.v2;
        let edge3 = self.v1 - self.v3;

        let n = edge1.cross(edge2).unit_vector();
        let d = ray.direction;
        let o = ray.origin;
        let distance = -n.dot(self.v1);

        let denom = n.x * d.x + n.y * d.y + n.z * d.z;

        if denom.abs() < 1e-7 { return None; }

        let t = -(n.x * o.x + n.y * o.y + n.z * o.z + distance) / denom;

        if t < t_min || t > t_max { return None; }

        let point = ray.at(t);

        let c1 = edge1.cross(point - self.v1).dot(n);
        let c2 = edge2.cross(point - self.v2).dot(n);
        let c3 = edge3.cross(point - self.v3).dot(n);

        if c1 > 0.0 && c2 > 0.0 && c3 > 0.0 {
            let mut hit_record = HitRecord { point, normal: n, t: t, front_face: false, material: &*self.material };
            hit_record.set_face_normal(ray, n);
            return Some(hit_record);
        } else{
            None
        }

    }
}