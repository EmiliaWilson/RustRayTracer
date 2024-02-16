use crate::vector;

use vector::Vec3 as Point3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: vector::Vec3,
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        return self.orig;
    }

    pub fn direction(&self) -> vector::Vec3 {
        return self.dir;
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.orig + self.dir * t;
    }
}
