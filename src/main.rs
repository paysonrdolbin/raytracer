mod vec3;
mod ray;
mod hittable;
mod sphere;
mod material;
mod camera;
mod lights;
mod common;

mod utils;

use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::Hittable;
use material::{Material, PhongMaterial, WhittedStyleMaterial};
use crate::camera::Camera;
use lights::{Light, DirectionalLight, AmbientLight};
use crate::hittable::HittableList;
use crate::utils::ray_color;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 1.0), // look_from
        Vec3::new(0.0, 0.0, 0.0), // look_at
        Vec3::new(0.0, 1.0, 0.0), // up
        90.0,
        aspect_ratio,);
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut world = HittableList::new();
    let samples = 50;
    let max_depth = 5;

    // ground sphere material
    let spherePhongWhite: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.8,
        0.1,
        0.3,
        Vec3::new(1.0, 1.0, 1.0), // Od
        Vec3::new(1.0, 1.0, 1.0), // Os
        4.0));

    // ground sphere
    world.add(Box::new(Sphere::new(
        Vec3::new(0.45, 0.0, -0.15), // position
        0.15,
        spherePhongWhite // material
    )));

    // glossy sphere material
    let spherePhongRed: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.6,
        0.3,
        0.1,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        32.0
    ));

    // glossy sphere
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -0.1),
        0.2,
        spherePhongRed
    )));

    let spherePhongeGreen: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
       0.7,
       0.2,
       0.1,
       Vec3::new(0.0, 1.0, 0.0),
       Vec3::new(0.5, 1.0, 0.5),
       64.0,
    ));

    world.add(Box::new(Sphere::new(
        Vec3::new(-0.6, 0.0, 0.0),
        0.3,
        spherePhongeGreen
    )));

    let spherePhongBlue: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.9,
        0.0,
        0.1,
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        16.0,
    ));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -10000.5, 0.0),
        10000.0,
        spherePhongBlue
    )));

    let lights = vec![
        Light::Directional(DirectionalLight::new(
            Vec3::new(1.0, 1.0, 1.0), // direction
            Vec3::new(1.0, 1.0, 1.0))), // color
        Light::Ambient(AmbientLight::new(Vec3::new(0.1, 0.1, 0.1))),
    ];

    println!("P3\n{} {}\n255", image_width, image_height); // PPM header

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..samples {
                let u = i as f64 / (image_width - 1) as f64;
                let v = j as f64 / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);

                pixel_color = pixel_color + ray_color(&r, &world, &lights, max_depth);
            }
            let mut final_color = pixel_color / samples as f64;
            // final_color.x = final_color.x.sqrt();
            // final_color.y = final_color.y.sqrt();
            // final_color.z = final_color.z.sqrt();
            println!("{}", final_color.write_color());
        }
    }
}
