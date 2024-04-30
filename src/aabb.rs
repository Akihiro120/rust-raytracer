use crate::interval::Interval;
use crate::vec3::*;
use crate::ray::Ray;

#[derive(Copy, Clone)]
struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn identity() -> AABB {
        Self {
            x: Interval::identity(),
            y: Interval::identity(),
            z: Interval::identity(),
        }
    }

    pub fn aabb(a: &Vec3, b: &Vec3) -> AABB {
        Self {
            x: Interval::interval(f64::min(a.x(), b.x()), f64::max(a.x(), b.x())),
            y: Interval::interval(f64::min(a.y(), b.y()), f64::max(a.y(), b.y())),
            z: Interval::interval(f64::min(a.z(), b.z()), f64::max(a.z(), b.z())),
        }
    }

    pub fn axis(&self, n: i32) -> Interval {
        if n == 1 {
            return self.y;
        } 
        if n == 2 {
            return self.z;
        }
        return self.x;
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        for a in 0..3 {
            let t0 = f64::min((self.axis(a).min() - r.origin()[a]) / r.direction()[a],
                              (self.axis(a).max() - r.origin()[a]) / r.direction()[a]);
            let t1 = f64::max((self.axis(a).min() - r.origin()[a]) / r.direction()[a],
                              (self.axis(a).max() - r.origin()[a]) / r.direction()[a]);

            ray_t.min() = f64::max(t0, ray_t.min());
            ray_t.max() = f64::min(t1, ray_t.max());
            if ray_t.max() <= ray_t.min() {
                return false;
            }
        }
        return true;
    }
}
