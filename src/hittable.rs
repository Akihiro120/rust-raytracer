use crate::ray::Ray;
use crate::vec3::{Vec3, dot};
use crate::interval::Interval;
use crate::material::*;

pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    mat: Materials,
    t: f64,
    front_face: bool
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            p: Vec3::vec3(0.0, 0.0, 0.0),
            normal: Vec3::vec3(0.0, 0.0, 0.0),
            mat: Materials::default(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = *outward_normal * Vec3::vec3(-1.0, -1.0, -1.0);
        }
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
       self.t 
    }

    pub fn mat(&self) -> Materials {
       self.mat
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn set_front_face(&mut self, front: &bool) {
        self.front_face = *front;
    }

    pub fn set_mat(&mut self, mat: &Materials) {
        self.mat = *mat;
    }

    pub fn set_p(&mut self, p: &Vec3) {
        self.p = *p;
    }

    pub fn set_normal(&mut self, normal: &Vec3) {
        self.normal = *normal;
    }

    pub fn set_t(&mut self, t: &f64) {
        self.t = *t;
    }  
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
