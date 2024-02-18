use crate::ray;
use crate::vector;

use vector::Vec3 as Point3;

struct Hit_record {
    p: Point3,
    normal: vector::Vec3,
    t: f64,
}

impl Hit_record {
    fn new(p: Point3, normal: vector::Vec3, t: f64) -> Hit_record {
        Hit_record { p, normal, t }
    }

    fn set_normal_face(&mut self, r: &ray::Ray, outward_normal: &vector::Vec3) {
        //Sets the hit record normal vector.
        // NOTE: the param outward_normal is assumed to have unit length

        let front_face = vector::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

trait Hittable {
    fn hit(&self, r: &ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &Hit_record) -> bool;
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &Hit_record) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vector::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // find nearest root
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: vector::Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}
