mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
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
    // World
    let world_list = Vec::new();
    let mut world = hittable_list::HittableList::new(world_list);
    world.add(Box::new(hittable::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));

    world.add(Box::new(hittable::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let mut cam = camera::Camera::new();
    cam.render(&mut world);
}
