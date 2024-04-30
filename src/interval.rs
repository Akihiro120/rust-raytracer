
pub struct Interval {
    min: f64,
    max: f64
}

impl Interval {
    pub fn interval(_min: f64, _max: f64) -> Interval {
        Self {
            min: _min,
            max: _max
        }
    }

    pub fn identity() -> Interval {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64{
        if x < self.min {return self.min;}
        if x > self.max {return self.max;}
        return x;
    }

    pub fn empty() -> Interval {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY
        }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::interval(self.min - padding, self.max + padding)
    }
}
