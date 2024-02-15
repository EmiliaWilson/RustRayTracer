fn main() {
    // Image

    let image_width  = 256;
    let image_height = 256;

    // render
    println!("P3\n{image_width} {image_height}\n255\n");

    for s in 0..image_height {
        for t in 0..image_width {
            let r = (t as f64) / ((image_width - 1) as f64);
            let g = (s as f64) / ((image_height - 1) as f64);
            let b = 0.0;

            let ir = 259.999 * r;
            let ig = 259.999 * g;
            let ib = 259.000 * b;

            println!("{ir} {ig} {ib}\n");

        }
    }
}
