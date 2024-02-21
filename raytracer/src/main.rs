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

use vector::Vec3 as Color;
use vector::Vec3 as Point3;

fn main() {
    // Materials
    let mut mat_ground =
        material::Material::Lambertian(material::Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mut mat_center =
        material::Material::Lambertian(material::Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mut mat_left = material::Material::Dielectric(material::Dielectric::new(1.5));
    let mut mat_right =
        material::Material::Metal(material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // World
    let world_list = Vec::new();
    let mut world = hittable_list::HittableList::new(world_list);

    world.add(Box::new(hittable::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));

    world.add(Box::new(hittable::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        mat_center,
    )));

    world.add(Box::new(hittable::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));

    world.add(Box::new(hittable::Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

    // Camera
    let mut cam = camera::Camera::new();
    cam.render(&mut world);
}
