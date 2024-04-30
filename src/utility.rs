use fastrand::*;

pub fn random_double() -> f64{
    // return a real in 0, 1
    fastrand::f64()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // return a real in [min, max]
    min + (max - min) * random_double()
}
