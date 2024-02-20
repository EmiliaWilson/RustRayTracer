use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double(min: f64, max: f64) -> f64 {
    return min + (max - min) * rand::random::<f64>();
}
