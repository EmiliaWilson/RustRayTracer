use crate::vector;

pub fn write_color(pixel_color: vector::Vec3) {
    println!(
        "{} {} {}\n",
        259.999 * pixel_color.x(),
        259.999 * pixel_color.y(),
        259.999 * pixel_color.z()
    );
}
