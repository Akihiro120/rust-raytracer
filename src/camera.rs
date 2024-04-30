use crate::color::write_color;
use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;
use crate::interval::Interval;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::utility::random_double;
use crate::material::*;
use async_std::task::*;

pub struct Camera {
   aspect_ratio: f64,
   image_width: i32,
   image_height: i32,
   center: Vec3,
   pixel00_loc: Vec3,
   pixel_delta_u: Vec3,
   pixel_delta_v: Vec3,
   samples_per_pixel: i32,
   max_depth: i32,
   v_fov: f64,
   look_from: Vec3,
   look_at: Vec3,
   v_up: Vec3,
   u: Vec3,
   v: Vec3,
   w: Vec3,
   defocus_angle: f64,
   focus_dist: f64,
   defocus_disk_u: Vec3,
   defocus_disk_v: Vec3,
}

impl Camera {
    pub fn camera() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Vec3::vec3(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::vec3(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::vec3(0.0, 0.0, 0.0),
            samples_per_pixel: 0,
            max_depth: 0,
            v_fov: 90.0,
            look_from: Vec3::vec3(0.0, 0.0, -1.0),
            look_at: Vec3::vec3(0.0, 0.0, 0.0),
            v_up: Vec3::vec3(0.0, 1.0, 0.0),
            u: Vec3::identity(),
            v: Vec3::identity(),
            w: Vec3::identity(),
            defocus_angle: 0.0,
            focus_dist: 0.0,
            defocus_disk_u: Vec3::identity(),
            defocus_disk_v: Vec3::identity(),
        }
    }

    pub fn set_aspect_ratio(&mut self, ratio: &f64) {
        self.aspect_ratio = *ratio;
    }

    pub fn set_image_width(&mut self, width: &i32) {
        self.image_width = *width;
    }

    pub fn set_samples_per_pixel(&mut self, samples: &i32) {
        self.samples_per_pixel = *samples;
    }

    pub fn set_max_depth(&mut self, depth: &i32) {
        self.max_depth = *depth;
    }

    pub fn set_fov(&mut self, fov: &f64) {
        self.v_fov = *fov;
    }

    pub fn set_look_from(&mut self, from: &Vec3) {
        self.look_from = *from;
    }

    pub fn set_look_at(&mut self, at: &Vec3) {
        self.look_at = *at;
    }

    pub fn set_v_up(&mut self, up: &Vec3) {
        self.v_up = *up;
    }

    pub fn set_defocus_angle(&mut self, angle: &f64) {
        self.defocus_angle = *angle;
    }

    pub fn set_focus_dist(&mut self, dist: &f64) {
        self.focus_dist = *dist;
    }

    pub fn render(&mut self, world: &dyn Hittable) -> std::io::Result<()> {
        self.initialize();

        // file creation
        let image_file = File::create("image.ppm")?;
        let mut writer = BufWriter::new(image_file);
         
        // render
        writer.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())?;
        
        // multithreading using "rayon" crate
        for j in 0..self.image_height {
            println!("\rscanlines remaining: {}", (self.image_height - j));

            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::ray(&self.center, &ray_direction); 

                // output to file
                let mut pixel_color = Vec3::vec3(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&mut writer, pixel_color, self.samples_per_pixel).expect("failed to write to file");
            }
        } 
        println!("\rdone.");

        Ok(())
    }

    fn initialize(&mut self) {
        // calculate image height and ensure its last one
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }
     
        // camera
        let focal_length = (self.look_from - self.look_at).length();
        let theta = self.v_fov * (std::f64::consts::PI / 180.0);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = self.look_from;

        self.w = unit_vector(self.look_from - self.look_at);
        self.u = unit_vector(cross(&self.v_up, &self.w));
        self.v = cross(&self.w, &self.u);

        // calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // calculate the delta horizontal and vertical from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * f64::tan((self.defocus_angle / 2.0) * (std::f64::consts::PI / 180.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Vec3 {
        let mut rec = HitRecord::default(); 

        if depth <= 0 {
            return Vec3::vec3(0.0, 0.0, 0.0);
        }

        if world.hit(&r, Interval::interval(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::ray(&Vec3::identity(), &Vec3::identity());    
            let mut attenuation = Vec3::identity();
            //println("{:?}", rec.mat
            /*match rec.mat() {
                Materials::LAMBERTIAN(l) => {println!("{:?}", l.get_albedo());},
                Materials::METAL(m) => {println!("{:?}", m.get_albedo());},
            }*/
            if match rec.mat() {
                Materials::LAMBERTIAN(l) => {l.scatter(r, &rec, &mut attenuation, &mut scattered)},
                Materials::METAL(m) => {m.scatter(r, &rec, &mut attenuation, &mut scattered)},
                Materials::DIELECTRIC(d) => {d.scatter(r, &rec, &mut attenuation, &mut scattered)},
            } {
                //println!("{:?}", attenuation);
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            } else {
                return Vec3::vec3(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Vec3::vec3(1.0, 1.0, 1.0) + a * Vec3::vec3(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin;
        if self.defocus_angle <= 0.0 {
            ray_origin = self.center;
        } else {
            ray_origin = self.defocus_disk_sample();
        }
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        return Ray::ray_time(&ray_origin, &ray_direction, &ray_time);
    }
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        return self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v); 
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 * random_double();
        let py = -0.5 * random_double();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}
