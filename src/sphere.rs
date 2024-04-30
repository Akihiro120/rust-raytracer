use crate::ray::Ray;
use crate::vec3::{
    Vec3,
    dot,};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::*;

pub struct Sphere {
    center1: Vec3,
    radius: f64,
    mat: Materials,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    pub fn sphere(_center: Vec3, _radius: f64, _material: Materials) -> Sphere {
        Sphere {
            center1: _center,
            radius: _radius,
            mat: _material,
            is_moving: false,
            center_vec: _center,
        }
    }

    pub fn sphere_moving(_center1: Vec3, _center2: Vec3, _radius: f64, _material: Materials) -> Sphere {
        Sphere {
            center1: _center1,
            radius: _radius,
            mat: _material,
            is_moving: true,
            center_vec: _center2 - _center1,  
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center1 + time * self.center_vec
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center: Vec3;
        if self.is_moving {
            center = self.center(r.time()); 
        } else {
            center = self.center1;
        }
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {return false;}
        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in an acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.set_t(&root);
        rec.set_p(&r.at(&rec.t()));
        let outward_normal = (rec.p() - center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);
        rec.set_mat(&self.mat);

        return true;
    }
}
