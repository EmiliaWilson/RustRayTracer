use crate::hittable;
use crate::interval;
use crate::ray;
use crate::vector;

use std::boxed::Box;
use std::vec::Vec;

pub struct HittableList {
    objects: Vec<Box<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn hittable::Hittable>>) -> Self {
        HittableList { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(
        &mut self,
        r: &ray::Ray,
        ray_t: interval::Interval,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = hittable::HitRecord::new(
            vector::Vec3 { e: [0.0; 3] },
            vector::Vec3 { e: [0.0; 3] },
            0.0,
        );
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter_mut() {
            if object.hit(
                r,
                interval::Interval::new(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}
