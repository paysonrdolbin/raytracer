use crate::vec3::Vec3;
use crate::lights::Light;
use crate::hittable::{HitRecord, HittableList, Hittable};
use crate::{common, Ray};
use crate::utils::ray_color;
#[derive(Clone)]
pub struct PhongMaterial {
    Kd: f64,
    Ks: f64,
    Ka: f64,
    Od: Vec3,
    Os: Vec3,
    Kgls: f64,
}

#[derive(Clone)]
pub struct WhittedStyleMaterial{
    Kd: f64,
    Ks: f64,
    Ka: f64,
    Od: Vec3,
    Os: Vec3,
    Kgls: f64,
    refl: f64,
}

#[derive(Clone)]
pub struct Dialectric {
    ior: f64,
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
    pub fn new(Kd: f64, Ks: f64, Ka: f64, Od: Vec3, Os: Vec3, Kgls: f64, refl: f64) -> Self {
        Self { Kd, Ks, Ka, Od, Os, Kgls, refl }
    }
}

impl Dialectric {
    pub fn new(ior: f64) -> Dialectric {
        Dialectric {
            ior: ior }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
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

        if max_depth > 0 {
            // Generate a random direction in the hemisphere around the normal
            // This simulates light bouncing off a rough/matte surface
            let scatter_direction = hit.normal + Vec3::random_in_unit_sphere().unit_vector();

            // Ensure the direction is valid (not zero)
            let target = if scatter_direction.near_zero() { hit.normal } else { scatter_direction };

            let diffuse_ray = Ray::new(hit.point + hit.normal * 1e-5, target.unit_vector());

            // Recurse to find the color of the object being "seen" by this surface
            let indirect_color = 0.5 * ray_color(&diffuse_ray, world, lights, max_depth - 1);

            // Add to total color, attenuated by the material's diffuse coefficient (Kd)
            color = color + (self.Kd * indirect_color);
        }

        // if max_depth > 0 {
        //     // r = d - 2n(d.dot(n)) where d is incoming ray
        //     let d = -view_dir.unit_vector();
        //     let reflect_dir = (d - 2.0 * hit.normal * d.dot(hit.normal)).unit_vector();
        //
        //     let reflect_ray = Ray::new(hit.point + hit.normal * 1e-5, reflect_dir);
        //     let reflected_color = ray_color(&reflect_ray, world, lights, max_depth - 1);
        //
        //     color = color + (self.refl * reflected_color);
        //
        // }

        color
    }
}

impl Material for Dialectric {
    fn shade(&self, hit: &HitRecord, world: &HittableList, lights: &[Light], view_dir: Vec3, max_depth: i32) -> Vec3 {
        let mut color = Vec3::new(0.0, 0.0, 0.0);

        if max_depth > 0 {
            let refraction_ratio = if hit.front_face {
                1.0 / self.ior
            } else {
                self.ior
            };

            let unit_direction = -view_dir.unit_vector();
            let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let direction = if cannot_refract
                || Self::reflectance(cos_theta, refraction_ratio) > common::random_double()
            {
                Vec3::reflect(&unit_direction, hit.normal)
            } else {
                Vec3::refract(&unit_direction, hit.normal, refraction_ratio)
            };

            // let refracted = unit_direction.refract(hit.normal, refraction_ratio);

            let scattered = Ray::new(hit.point, direction);
            color = ray_color(&scattered, world, lights, max_depth - 1);
        }
        color
    }
}