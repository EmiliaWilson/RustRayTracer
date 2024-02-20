use crate::color;
use crate::hittable;
use crate::interval;
use crate::ray;
use crate::utility;
use crate::vector;

use core::f64::INFINITY;
use core::f64::NEG_INFINITY;

use vector::Vec3 as Point3;
use vector::Vec3 as Color;

pub struct Camera {
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: vector::Vec3,
    pixel_delta_v: vector::Vec3,
}

impl Camera {
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
    pub const IMAGE_WIDTH: i32 = 400;

    pub fn new() -> Camera {
        Camera {
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: vector::Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: vector::Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &mut dyn hittable::Hittable) {
        self.initialize();

        // render
        println!("P3\n{} {}\n255\n", Self::IMAGE_WIDTH, self.image_height);

        for s in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - s);
            for t in 0..Self::IMAGE_WIDTH {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * t as f64)
                    + (self.pixel_delta_v * s as f64);
                let ray_direction = pixel_center - self.center;
                let r = ray::Ray {
                    orig: self.center,
                    dir: ray_direction,
                };

                let pixel_color = Self::ray_color(r, world);
                color::write_color(pixel_color);
            }
        }

        eprintln!("\rDone.       \n");
    }

    fn initialize(&mut self) {
        let calc_image_height = Self::IMAGE_WIDTH as f64 / Self::ASPECT_RATIO;
        self.image_height = if calc_image_height < 1.0 {
            1
        } else {
            calc_image_height as i32
        };
        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (Self::IMAGE_WIDTH as f64 / self.image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = vector::Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = vector::Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / Self::IMAGE_WIDTH as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = camera_center
            - vector::Vec3 {
                e: [0.0, 0.0, focal_length],
            }
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5f64;
    }

    fn ray_color(r: ray::Ray, world: &mut dyn hittable::Hittable) -> Color {
        let mut hit_record = hittable::HitRecord::new(
            vector::Vec3 { e: [0.0; 3] },
            vector::Vec3 { e: [0.0; 3] },
            0.0,
        );

        if world.hit(&r, interval::Interval::new(0.0, INFINITY), &mut hit_record) {
            return (hit_record.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return Color { e: [1.0; 3] } * (1.0 - a) + Color { e: [0.5, 0.7, 1.0] } * a;
    }
}
