use std::ptr::addr_eq;
use crate::vec3::Vec3;
use crate::lights::Light;
use crate::hittable::{HitRecord, HittableList, Hittable};
use crate::Ray;
use crate::utils::ray_color;
pub struct PhongMaterial {
    Kd: f64,
    Ks: f64,
    Ka: f64,
    Od: Vec3,
    Os: Vec3,
    Kgls: f64,
}

pub struct WhittedStyleMaterial{
    Kd: f64,
    Ks: f64,
    Ka: f64,
    Od: Vec3,
    Os: Vec3,
    Kgls: f64,
}

pub trait Material {
    fn shade(&self, hit: &HitRecord, world: &HittableList, lights: &[Light], view_dir: Vec3, max_depth: i32) -> Vec3;
}

impl PhongMaterial {
    pub fn new(Kd: f64, Ks: f64, Ka: f64, Od: Vec3, Os: Vec3, Kgls: f64) -> Self {
        Self { Kd, Ks, Ka, Od, Os, Kgls }
    }
}

impl WhittedStyleMaterial {
    pub fn new(Kd: f64, Ks: f64, Ka: f64, Od: Vec3, Os: Vec3, Kgls: f64) -> Self {
        Self { Kd, Ks, Ka, Od, Os, Kgls }
    }
}

impl Material for PhongMaterial {

    fn shade(&self, hit: &HitRecord, world: &HittableList, lights: &[Light], view_dir: Vec3, max_depth: i32) -> Vec3 {
        let mut color = Vec3::new(0.0, 0.0, 0.0);
        for light in lights {
            match light {
                Light::Ambient(a) => {
                    let ambient = self.Ka * a.color * self.Od;
                    color = color + ambient;
                }
                Light::Directional(d) => {
                    let l = d.direction.unit_vector();

                    let shadow_origin = hit.point + hit.normal * 1e-5;
                    let shadow_ray = Ray::new(shadow_origin, l);

                    if world.hit(&shadow_ray, 0.001, f64::INFINITY).is_some(){
                        continue;
                    };

                    let v = view_dir.unit_vector();
                    let ndotl = hit.normal.dot(l).max(0.0);
                    let diffuse = self.Kd * d.color * self.Od * ndotl;
                    color = color + diffuse;
                    let r = (2.0 * hit.normal * (hit.normal.dot(l)) - l);
                    let vdotr = v.dot(r).max(0.0);
                    let specular = self.Ks * d.color * self.Os * vdotr.powf(self.Kgls);
                    color = color + specular;

                }

            }
        }
    color
    }

}

impl Material for WhittedStyleMaterial {
    fn shade(&self, hit: &HitRecord, world: &HittableList, lights: &[Light], view_dir: Vec3, max_depth: i32) -> Vec3 {
        let mut color = Vec3::new(0.0, 0.0, 0.0);
        // let mut is_any_light_visible = false;

        for light in lights {
            match light {
                Light::Ambient(a) => {
                    let ambient = self.Ka * a.color * self.Od;
                    color = color + ambient;
                }
                Light::Directional(d) => {
                    let l = d.direction.unit_vector();
                    let shadow_origin = hit.point + hit.normal * 1e-5;
                    let shadow_ray = Ray::new(shadow_origin, l);

                    if world.hit(&shadow_ray, 0.001, f64::INFINITY).is_some(){
                        continue;
                    };

                    // is_any_light_visible = true;

                    let v = view_dir.unit_vector();
                    let ndotl = hit.normal.dot(l).max(0.0);

                    // Diffuse
                    let diffuse = self.Kd * d.color * self.Od * ndotl;
                    color = color + diffuse;

                    // Specular
                    let r = (2.0 * hit.normal * (hit.normal.dot(l)) - l);
                    let vdotr = v.dot(r).max(0.0);
                    let specular = self.Ks * d.color * self.Os * vdotr.powf(self.Kgls);
                    color = color + specular;
                }
            }
        }

        // if max_depth > 0 {
        //     // Generate a random direction in the hemisphere around the normal
        //     // This simulates light bouncing off a rough/matte surface
        //     let scatter_direction = hit.normal + Vec3::random_in_unit_sphere().unit_vector();
        //
        //     // Ensure the direction is valid (not zero)
        //     let target = if scatter_direction.near_zero() { hit.normal } else { scatter_direction };
        //
        //     let diffuse_ray = Ray::new(hit.point + hit.normal * 1e-5, target.unit_vector());
        //
        //     // Recurse to find the color of the object being "seen" by this surface
        //     let indirect_color = ray_color(&diffuse_ray, world, lights, max_depth - 1);
        //
        //     // Add to total color, attenuated by the material's diffuse coefficient (Kd)
        //     color = color + (self.Kd * indirect_color);
        // }

        if max_depth > 0 {
            // r = d - 2n(d.dot(n)) where d is incoming ray
            let d = -view_dir.unit_vector();
            let reflect_dir = (d - 2.0 * hit.normal * d.dot(hit.normal)).unit_vector();

            let reflect_ray = Ray::new(hit.point + hit.normal * 1e-5, reflect_dir);
            let reflected_color = ray_color(&reflect_ray, world, lights, max_depth - 1);

            color = color + (self.Ks * reflected_color);

        }

        color
    }
}