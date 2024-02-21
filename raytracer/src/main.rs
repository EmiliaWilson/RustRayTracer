mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod utility;
mod vector;

use core::f64::INFINITY;
use core::f64::NEG_INFINITY;
use std::boxed::Box;
use std::vec::Vec;

use utility::random_double;
use vector::Vec3 as Color;
use vector::Vec3 as Point3;

fn main() {
    // Materials
    let mut mat_ground =
        material::Material::Lambertian(material::Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    // World
    let world_list = Vec::new();
    let mut world = hittable_list::HittableList::new(world_list);

    world.add(Box::new(hittable::Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utility::random_double(0.0, 1.0);
            let center = Point3::new(
                a as f64 + 0.9 * random_double(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * random_double(0.0, 1.0),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let mat = material::Material::Lambertian(material::Lambertian::new(albedo));
                    world.add(Box::new(hittable::Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let mat = material::Material::Metal(material::Metal::new(albedo, fuzz));
                    world.add(Box::new(hittable::Sphere::new(center, 0.2, mat)));
                } else {
                    let mat = material::Material::Dielectric(material::Dielectric::new(1.5));
                    world.add(Box::new(hittable::Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let mat1 = material::Material::Dielectric(material::Dielectric::new(1.5));
    world.add(Box::new(hittable::Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        mat1,
    )));

    let mat2 = material::Material::Lambertian(material::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(hittable::Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));

    let mat3 = material::Material::Metal(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(hittable::Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        mat3,
    )));

    // Camera
    let mut cam = camera::Camera::new();
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&mut world);
}
