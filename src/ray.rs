use crate::vec3::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
    tm: f64
}

impl Ray {
    pub fn ray(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            orig: *origin,
            dir: *direction,
            tm: 0.0
        }
    }

    pub fn ray_time(origin: &Vec3, direction: &Vec3, time: &f64 ) -> Ray {
        Ray {
            orig: *origin,
            dir: *direction,
            tm: *time,
        }
    }

    pub fn set(&mut self, r: &Ray) {
        self.orig = r.origin();
        self.dir = r.direction();
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    
    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: &f64) -> Vec3{
        self.orig + *t * self.dir
    }

    pub fn set_origin(&mut self, orig: &Vec3) {
        self.orig = *orig;
    }

    pub fn set_direction(&mut self, dir: &Vec3) {
        self.dir = *dir;   
    }

    pub fn set_time(&mut self, time: &f64) {
        self.tm = *time;
    }
}
