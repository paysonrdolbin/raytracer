mod vec3;
mod ray;
mod hittable;
mod sphere;
mod material;
mod camera;
mod lights;
mod common;

mod utils;
mod triangle;

use image::codecs::pnm::ArbitraryTuplType::BlackAndWhiteAlpha;
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::Hittable;
use material::{Material, PhongMaterial, WhittedStyleMaterial};
use crate::camera::Camera;
use lights::{Light, DirectionalLight, AmbientLight};
use crate::hittable::HittableList;
use crate::utils::ray_color;
use crate::common::random_double;
use crate::triangle::Triangle;

fn main() {
    let aspect_ratio = 1.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut world = HittableList::new();
    let samples = 50;
    let max_depth = 5;

    // white sphere material
    let spherePhongWhite: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.8,
        0.1,
        0.3,
        Vec3::new(1.0, 1.0, 1.0), // Od
        Vec3::new(1.0, 1.0, 1.0), // Os
        4.0,
        0.0
    ));

    // white sphere placement
    world.add(Box::new(Sphere::new(
        Vec3::new(0.5, 0.0, -0.15), // position
        0.05,
        spherePhongWhite // material
    )));

    // red sphere material
    let spherePhongRed: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.8,
        0.8,
        0.1,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.5, 1.0, 0.5),
        32.0,
        0.0
    ));

    // red sphere placement
    world.add(Box::new(Sphere::new(
        Vec3::new(0.3, 0.0, -0.1),
        0.08,
        spherePhongRed
    )));

    // green sphere material
    let spherePhongGreen: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.7,
        0.5,
        0.1,
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.5, 1.0, 0.5),
        64.0,
        0.0
    ));

    // green sphere placement
    world.add(Box::new(Sphere::new(
        Vec3::new(-0.6, 0.0, 0.0),
        0.3,
        spherePhongGreen
    )));

    // reflective sphere material
    let spherePhongReflective: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.0,
        0.1,
        0.1,
        Vec3::new(0.75, 0.75, 0.75),
        Vec3::new(1.0, 1.0, 1.0),
        10.0,
        0.9
    ));

    // refelctive sphere placement
    world.add(Box::new(Sphere::new(
        Vec3::new(0.1, -0.55, 0.25),
        0.3,
        spherePhongReflective
    )));

    // blue triangle material
    let trianglePhongBlue: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.9,
        0.9,
        0.1,
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        32.0,
        0.0
    ));

    // blue triangle placement
    world.add(Box::new(Triangle::new(
        Vec3::new(0.3, -0.3, -0.4),
        Vec3::new(0.0, 0.3, -0.1),
        Vec3::new(-0.3, -0.3, 0.2),
        trianglePhongBlue
    )));

    // yellow triangle material
    let trianglePhongYellow: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.9,
        0.5,
        0.1,
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        4.0,
        0.0
    ));

    // yellow triangle placement
    world.add(Box::new(Triangle::new(
        Vec3::new(-0.2, 0.1, 0.1),
        Vec3::new(-0.2, -0.5, 0.2),
        Vec3::new(-0.2, 0.1, -0.3),
        trianglePhongYellow
    )));

    let lights = vec![
        Light::Directional(DirectionalLight::new(
            Vec3::new(1.0, 0.0, 0.0), // direction
            Vec3::new(1.0, 1.0, 1.0))), // color
        Light::Ambient(AmbientLight::new(Vec3::new(0.1, 0.1, 0.1))),
    ];

    // loop for camera movement

    // let total_frames = 1;
    //
    // for frame in 0..total_frames {
    //     let t = frame as f64 / total_frames as f64;
    //
    //     // smoother ease in/out
    //     let t = t * t * (3.0 - 2.0 * t);
    //
    //     // full orbit
    //     let angle = t * 2.0 * std::f64::consts::PI;
    //
    //     // subtle radius variation (breathing camera)
    //     let base_radius = 1.0;
    //     let radius = base_radius + 0.2 * (2.0 * angle).sin();
    //
    //     // vertical arc motion
    //     let height = 0.3 * (angle * 0.5).sin();
    //
    //     let look_at = Vec3::new(0.0, 0.0, 0.0);
    //
    //     let look_from = Vec3::new(
    //         radius * angle.sin(),
    //         height,
    //         radius * angle.cos(),
    //     );
    //
    //     // subtle roll
    //     let roll_amount = 0.1 * (angle * 0.5).sin();
    //     let look_up = Vec3::new(
    //         roll_amount.sin(),
    //         roll_amount.cos(),
    //         0.0,
    //     );
    //
    //     let camera = Camera::new(
    //         look_from,
    //         look_at,
    //         look_up,
    //         90.0,
    //         aspect_ratio,
    //     );
    //
    //     let filename = format!("frame_{:03}.ppm", frame);
    //     let mut file = std::fs::File::create(&filename).expect("Failed to create file");
    //     use std::io::Write;
    //
    //     writeln!(file, "P3").unwrap();
    //     writeln!(file, "{} {}", image_width, image_height).unwrap();
    //     writeln!(file, "255").unwrap();
    //
    //     for j in (0..image_height).rev() {
    //         for i in 0..image_width {
    //             let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
    //
    //             for _s in 0..samples {
    //                 let u = (i as f64 + random_double()) / (image_width - 1) as f64;
    //                 let v = (j as f64 + random_double()) / (image_height - 1) as f64;
    //
    //                 let r = camera.get_ray(u, v);
    //                 pixel_color = pixel_color + ray_color(&r, &world, &lights, max_depth);
    //             }
    //
    //             let final_color = pixel_color / samples as f64;
    //             writeln!(file, "{}", final_color.write_color()).unwrap();
    //         }
    //     }
    //
    //     println!("Finished frame {}", frame);
    // }

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio
    );

    println!("P3\n{} {}\n255", image_width, image_height); // PPM header

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..samples {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

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
