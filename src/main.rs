use std::fs::File;
use std::io::{BufWriter, Write};
use crate::vec3::*;
use crate::material::*;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod utility;
mod material;
mod aabb;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::utility::*;

fn main() { 
    // world
    let mut world = HittableList::hittable_list();
    
    let ground_material = Materials::METAL(Metal::new(Vec3::vec3(0.3, 0.3, 0.3), 0.0001));
    world.add(Box::new(Sphere::sphere(Vec3::vec3(0.0, -1000.0, 0.0), 1000.0, ground_material)));


    for a in -3..3 {
        for b in -3..3 {
            let choose_mat = random_double();
            let center = Vec3::vec3(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if choose_mat < 0.8 {
                // diffuse
                let albedo = Vec3::random() * Vec3::random();
                let sphere_material = Materials::LAMBERTIAN(Lambertian::new(albedo));
                let center2 = center + Vec3::vec3(0.0, random_double_range(0.0, 0.5), 0.0);
                world.add(Box::new(Sphere::sphere_moving(center, center2, 0.2, sphere_material)));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Vec3::random_range(0.5, 1.0);
                let fuzz = random_double_range(0.0, 0.5);
                let sphere_material = Materials::METAL(Metal::new(albedo, fuzz));
                world.add(Box::new(Sphere::sphere(center, 0.2, sphere_material)));
            } else {
                // glass
                let sphere_material = Materials::DIELECTRIC(Dielectric::new(1.5));
                world.add(Box::new(Sphere::sphere(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Materials::DIELECTRIC(Dielectric::new(1.5));
    world.add(Box::new(Sphere::sphere(Vec3::vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Materials::LAMBERTIAN(Lambertian::new(Vec3::vec3(-4.0, 1.0, 0.0)));
    world.add(Box::new(Sphere::sphere(Vec3::vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Materials::METAL(Metal::new(Vec3::vec3(0.2, 0.2, 0.5), 0.01));
    world.add(Box::new(Sphere::sphere(Vec3::vec3(4.0, 1.0, 0.0), 1.0, material3)));

    // camera
    let mut cam = Camera::camera();
    cam.set_aspect_ratio(&(16.0 / 9.0));
    cam.set_image_width(&400);
    cam.set_samples_per_pixel(&100);
    cam.set_max_depth(&50);
    cam.set_fov(&20.0);
    cam.set_look_from(&Vec3::vec3(13.0, 2.0, 3.0));
    cam.set_look_at(&Vec3::vec3(0.0, 0.0, 0.0));
    cam.set_v_up(&Vec3::vec3(0.0, 1.0, 0.0));
    cam.set_defocus_angle(&0.6);
    cam.set_focus_dist(&10.0);
    cam.render(&world).expect("Failed to Render the Image to File");
}
