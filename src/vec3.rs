use std::ops::{
    Add,
    Sub,
    Div,
    Mul,
    AddAssign,
    DivAssign,
    MulAssign,
    Neg,
};
use crate::utility::*;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
} 

impl Vec3 {
    pub fn vec3(e0: f64, e1: f64, e2: f64) -> Vec3{
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn identity() -> Vec3 {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn set(&mut self, x: &Vec3) {
        self.e[0] = x.e[0];
        self.e[1] = x.e[1];
        self.e[2] = x.e[2];
    }

    pub fn x(&self) -> f64 {
        self.e[0] 
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn set_x(&mut self, x: &f64) {
        self.e[0] = *x;
    }

    pub fn set_y(&mut self, y: &f64) {
        self.e[1] = *y;
    }

    pub fn set_z(&mut self, z: &f64) {
        self.e[2] = *z;
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Vec3 {
        Vec3::vec3(random_double(), random_double(), random_double())
    }
    
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::vec3(random_double_range(min, max), random_double_range(min, max),random_double_range(min, max)) 
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }

}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Vec3{
        Vec3 {
            e: [
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z()
            ]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z(),
            ]
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() * rhs.x(),
                self.y() * rhs.y(),
                self.z() * rhs.z()
            ]
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self * v.x(),
                self * v.y(),
                self * v.z(),
            ]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Vec3 {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Vec3 {
        (1.0 / t) * self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [
                -self.x(),
                -self.y(),
                -self.z()
            ]
        }   
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z()
            ]
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [
                self.x() * other,
                self.y() * other,
                self.z() * other
            ]
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0 / other  
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x()
        ]
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::vec3(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(&v, &n) * *n 
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&(uv * Vec3::vec3(-1.0, -1.0, -1.0)), &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    return r_out_perp + r_out_parallel;
}
