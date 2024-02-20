use core::f64::INFINITY;
use core::f64::NEG_INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }
}

const EMPTY: Interval = Interval {
    min: INFINITY,
    max: NEG_INFINITY,
};

const UNIVERSE: Interval = Interval {
    min: NEG_INFINITY,
    max: INFINITY,
};
