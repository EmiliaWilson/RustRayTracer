use crate::color;
use crate::hittable;
use crate::interval;
use crate::material;
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
    pub look_from: Point3,
    pub look_at: Point3,
    v_up: vector::Vec3,
    u: vector::Vec3,
    v: vector::Vec3,
    w: vector::Vec3,
}

impl Camera {
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
    pub const IMAGE_WIDTH: i32 = 400;
    pub const VFOV: i32 = 20;
    pub const SAMPLES_PER_PIXEL: i32 = 100;
    pub const MAX_DEPTH: i32 = 50;
    pub fn new() -> Camera {
        Camera {
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: vector::Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: vector::Vec3::new(0.0, 0.0, 0.0),
            look_from: vector::Vec3::new(0.0, 0.0, -1.0),
            look_at: vector::Vec3::new(0.0, 0.0, 0.0),
            v_up: vector::Vec3::new(0.0, 1.0, 0.0),
            u: vector::Vec3::new(0.0, 0.0, 0.0),
            v: vector::Vec3::new(0.0, 0.0, 0.0),
            w: vector::Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &mut dyn hittable::Hittable) {
        self.initialize();

        // render
        println!("P3\n{} {}\n255\n", Self::IMAGE_WIDTH, self.image_height);

        for s in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - s);
            for t in 0..Self::IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..Self::SAMPLES_PER_PIXEL {
                    let r = self.get_ray(t, s);
                    pixel_color = pixel_color + Self::ray_color(r, Self::MAX_DEPTH, world);
                }
                color::write_color(pixel_color, Self::SAMPLES_PER_PIXEL);
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
        self.center = self.look_from;

        let focal_length = (self.look_from - self.look_at).length();
        let theta = utility::degrees_to_radians(Self::VFOV as f64);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width =
            viewport_height * (Self::IMAGE_WIDTH as f64 / self.image_height as f64);

        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = vector::cross(&self.v_up, &self.w).unit_vector();
        self.v = vector::cross(&self.w, &self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        self.pixel_delta_u = viewport_u / Self::IMAGE_WIDTH as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5f64;
    }

    fn ray_color(r: ray::Ray, depth: i32, world: &mut dyn hittable::Hittable) -> Color {
        let mut hit_record = hittable::HitRecord::new(
            vector::Vec3 { e: [0.0; 3] },
            vector::Vec3 { e: [0.0; 3] },
            material::Material::Lambertian(material::Lambertian::new(Color::new(0.0, 0.0, 0.0))),
            0.0,
            true,
        );

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if world.hit(
            &r,
            interval::Interval::new(0.001, INFINITY),
            &mut hit_record,
        ) {
            let mut scattered = ray::Ray::new(
                vector::Vec3::new(0.0, 0.0, 0.0),
                vector::Vec3::new(0.0, 0.0, 0.0),
            );
            let mut attenuation = Color::new(0.0, 0.0, 0.0);

            if hit_record
                .mat
                .scatter(&r, &hit_record, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return Color { e: [1.0; 3] } * (1.0 - a) + Color { e: [0.5, 0.7, 1.0] } * a;
    }

    fn get_ray(&self, i: i32, j: i32) -> ray::Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return ray::Ray::new(ray_origin, ray_direction);
    }

    fn pixel_sample_square(&self) -> vector::Vec3 {
        let px = -0.5 + utility::random_double(0.0, 1.0);
        let py = -0.5 + utility::random_double(0.0, 1.0);
        return (self.pixel_delta_u * px) + (self.pixel_delta_v * py);
    }
}
