use crate::hittable;
use crate::ray;
use crate::vector;

use std::rc::Rc;
use std::vec::Vec;

#[derive(Clone)]
struct HittableList {
    objects: Vec<Rc<dyn hittable::Hittable>>,
}

impl HittableList {
    fn new(objects: Vec<Rc<dyn hittable::Hittable>>) -> Self {
        HittableList { objects }
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, object: Rc<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(
        &mut self,
        r: &ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = hittable::HitRecord::new(
            vector::Vec3 { e: [0.0; 3] },
            vector::Vec3 { e: [0.0; 3] },
            0.0,
        );
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = &mut temp_rec;
            }
        }

        return hit_anything;
    }
}
