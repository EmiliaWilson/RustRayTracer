mod color;
mod vector;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // render
    println!("P3\n{image_width} {image_height}\n255\n");

    for s in 0..image_height {
        for t in 0..image_width {
            let pixel_color = vector::Vec3 {
                e: [
                    t as f64 / (image_width - 1) as f64,
                    s as f64 / (image_height - 1) as f64,
                    0.0,
                ],
            };
            color::write_color(pixel_color);
        }
    }
}
