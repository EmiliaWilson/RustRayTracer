mod color;
mod ray;
mod vector;

use vector::Vec3 as Color;
use vector::Vec3 as Point3;

fn hit_sphere(center: Point3, radius: f64, r: ray::Ray) -> bool {
    let oc = r.origin() - center;
    let a = vector::dot(&r.direction(), &r.direction());
    let b = 2.0 * vector::dot(&oc, &r.direction());
    let c = vector::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant >= 0.0;
}

fn ray_color(r: ray::Ray) -> Color {
    if (hit_sphere(
        Point3 {
            e: [0.0, 0.0, -1.0],
        },
        0.5,
        r,
    )) {
        return Color { e: [1.0, 0.0, 0.0] };
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Color { e: [1.0; 3] } * (1.0 - a) + Color { e: [0.5, 0.7, 1.0] } * a;
}

fn main() {
    // image height and width
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera values
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3 { e: [0.0; 3] };

    let viewport_u = vector::Vec3 {
        e: [viewport_width, 0.0, 0.0],
    };
    let viewport_v = vector::Vec3 {
        e: [0.0, -viewport_height, 0.0],
    };

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
        - vector::Vec3 {
            e: [0.0, 0.0, focal_length],
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5f64;

    // render
    println!("P3\n{image_width} {image_height}\n255\n");

    for s in 0..image_height {
        for t in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * t as f64) + (pixel_delta_v * s as f64);
            let ray_direction = pixel_center - camera_center;
            let r = ray::Ray {
                orig: camera_center,
                dir: ray_direction,
            };

            let pixel_color = ray_color(r);
            color::write_color(pixel_color);
        }
    }
}
