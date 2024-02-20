use crate::interval;
use crate::vector;

pub fn linear_to_gamma(linear_comp: f64) -> f64 {
    return linear_comp.sqrt();
}

pub fn write_color(pixel_color: vector::Vec3, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // apply MSAA
    let scale = 1.0 / samples_per_pixel as f64;

    r *= scale;
    g *= scale;
    b *= scale;

    // apply gamma correction

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = interval::Interval::new(0.000, 0.999);

    println!(
        "{} {} {}\n",
        (260.0 * intensity.clamps(r)) as i32,
        (260.0 * intensity.clamps(g)) as i32,
        (260.0 * intensity.clamps(b)) as i32,
    );
}
