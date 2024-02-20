use crate::hittable;
use crate::ray;
use crate::vector;

use vector::Vec3 as Color;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        match self {
            Material::Lambertian(mat) => mat.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(mat) => mat.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }

    pub fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + vector::Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = ray::Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }

    pub fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let reflected = vector::reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = ray::Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}
