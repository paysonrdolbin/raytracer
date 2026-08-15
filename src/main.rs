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
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut world = HittableList::new();
    let samples = 50;
    let max_depth = 5;

    // ---------- Materials ----------

    // matte gray ground
    let ground_mat: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.9, 0.1, 0.0,
        Vec3::new(0.7, 0.7, 0.7),
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        0.0
    ));

    // mirror material
    let mirror_mat: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.0, 0.1, 0.0,
        Vec3::new(0.9, 0.9, 0.9),
        Vec3::new(1.0, 1.0, 1.0),
        64.0,
        0.9
    ));

    // blue glossy
    let blue_gloss: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.8, 0.6, 0.2,
        Vec3::new(0.1, 0.3, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        32.0,
        0.1
    ));

    // red glossy
    let red_gloss: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.8, 0.7, 0.2,
        Vec3::new(1.0, 0.1, 0.1),
        Vec3::new(1.0, 1.0, 1.0),
        32.0,
        0.2
    ));

    // green matte
    let green_mat: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.9, 0.4, 0.0,
        Vec3::new(0.2, 0.8, 0.2),
        Vec3::new(0.2, 0.2, 0.2),
        8.0,
        0.0
    ));

    world.add(Box::new(Triangle::new(
        Vec3::new(-2.0, -0.6, -2.0),
        Vec3::new( 2.0, -0.6, -2.0),
        Vec3::new( 2.0, -0.6,  2.0),
        ground_mat.clone()
    )));

    world.add(Box::new(Triangle::new(
        Vec3::new(-2.0, -0.6, -2.0),
        Vec3::new( 2.0, -0.6,  2.0),
        Vec3::new(-2.0, -0.6,  2.0),
        ground_mat
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-0.4, -0.3, -3.0),
        0.3,
        mirror_mat
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.2, -0.35, -0.2),
        0.25,
        blue_gloss
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.7, -0.35, -0.9),
        0.3,
        red_gloss
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-0.8, -0.45, -0.5),
        0.12,
        green_mat.clone()
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-0.65, -0.45, -0.7),
        0.1,
        green_mat.clone()
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-0.9, -0.45, -0.75),
        0.09,
        green_mat
    )));

    let pyramid_mat: Box<dyn Material> = Box::new(WhittedStyleMaterial::new(
        0.8, 0.8, 0.3,
        Vec3::new(1.0, 0.8, 0.2),
        Vec3::new(1.0, 1.0, 1.0),
        32.0,
        0.1
    ));

    let p0 = Vec3::new(0.0, -0.2, -0.7);
    let p1 = Vec3::new(-0.2, -0.6, -0.9);
    let p2 = Vec3::new(0.2, -0.6, -0.9);
    let p3 = Vec3::new(0.0, -0.6, -0.5);

    // sides
    world.add(Box::new(Triangle::new(p0, p1, p2, pyramid_mat.clone())));
    world.add(Box::new(Triangle::new(p0, p2, p3, pyramid_mat.clone())));
    world.add(Box::new(Triangle::new(p0, p3, p1, pyramid_mat.clone())));

    // base
    world.add(Box::new(Triangle::new(p1, p2, p3, pyramid_mat)));



    let lights = vec![
        // Key light (top-right)
        Light::Directional(DirectionalLight::new(
            Vec3::new(1.0, 1.5, 1.0).unit_vector(),
            Vec3::new(1.0, 1.0, 1.0),
        )),

        // Fill light (left side)
        Light::Directional(DirectionalLight::new(
            Vec3::new(-1.0, 0.5, 0.3).unit_vector(),
            Vec3::new(0.4, 0.4, 0.5),
        )),

        // Ambient light
        Light::Ambient(AmbientLight::new(
            Vec3::new(0.08, 0.08, 0.08)
        )),
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
        Vec3::new(0.0, -0.25, -0.8), // look direction
        Vec3::new(0.0, 0.4, 1.6),   // camera position
        Vec3::new(0.0, 1.0, 0.0),    // up vector
        60.0,                        // field of view
        aspect_ratio,
    );

    println!("P3\n{} {}\n255\n", image_width, image_height); // PPM header

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
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

    eprint!("\nDone.\n");
}
