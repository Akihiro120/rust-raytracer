use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::*;
use crate::utility::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        false
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Lambertian {
        Self {
            albedo: a,
        }
    }

    pub fn get_albedo(&self) -> Vec3 {
        self.albedo
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal() + random_unit_vector();
        
        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        let _scattered = Ray::ray_time(&rec.p(), &scatter_direction, &r_in.time());
        scattered.set_origin(&_scattered.origin());
        scattered.set_direction(&_scattered.direction());

        let _attenuation = self.get_albedo();
        attenuation.set_x(&_attenuation.x());
        attenuation.set_y(&_attenuation.y());
        attenuation.set_z(&_attenuation.z());
        //println!("{:?}", attenuation);
        return true;
    }

}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(a: Vec3, f: f64) -> Metal {
        Self {
            albedo: a,
            fuzz: f,
        }
    }

    fn get_albedo(&self) -> Vec3 {
        self.albedo
    }

    fn get_fuzz(&self) -> f64 {
        self.fuzz
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal());  

        let _scattered = Ray::ray_time(&rec.p(), &(reflected + self.get_fuzz() * random_unit_vector()), &r_in.time());
        scattered.set_origin(&_scattered.origin());
        scattered.set_direction(&_scattered.direction());

        let _attenuation = self.get_albedo();
        attenuation.set_x(&_attenuation.x());
        attenuation.set_y(&_attenuation.y());
        attenuation.set_z(&_attenuation.z());
        return true;
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(_ir: f64) -> Dielectric {
        Self {
            ir: _ir
        }
    }
    
    pub fn get_ir(&self) -> f64 {
        return self.ir;
    }

    pub fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        attenuation.set(&Vec3::vec3(1.0, 1.0, 1.0)); 
        let refraction_ratio;
        if rec.front_face() {
            refraction_ratio = 1.0 / self.get_ir();
        } else {
            refraction_ratio = self.get_ir();
        }
        
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = f64::min(dot(&(unit_direction * Vec3::vec3(-1.0, -1.0, -1.0)), &rec.normal()), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = reflect(&unit_direction, &rec.normal());
        } else {
            direction = refract(unit_direction, rec.normal(), refraction_ratio);
        }

        let _scattered = Ray::ray_time(&rec.p(), &direction, &r_in.time());
        scattered.set_origin(&_scattered.origin());
        scattered.set_direction(&_scattered.direction());
        return true;
    }
}

#[derive(Copy, Clone)]
pub enum Materials {
    LAMBERTIAN(Lambertian),
    METAL(Metal),
    DIELECTRIC(Dielectric),
}

impl Default for Materials {
    fn default() -> Self{Materials::LAMBERTIAN(Lambertian{
        albedo: Vec3::vec3(0.0, 0.0, 0.0)
    })}
}
