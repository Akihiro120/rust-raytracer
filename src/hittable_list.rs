use crate::hittable::*;
use crate::vec3::*;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::*;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn hittable_list() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
     fn  hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in self.objects.iter() {
            if object.hit(&r, Interval::interval(ray_t.min(), closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                rec.set_p(&temp_rec.p());
                rec.set_t(&temp_rec.t());
                rec.set_normal(&temp_rec.normal());
                rec.set_mat(&temp_rec.mat());
                rec.set_front_face(&temp_rec.front_face());
            }
        }

        return hit_anything;
     }
}
