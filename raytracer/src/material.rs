use crate::hittable;
use crate::ray;
use crate::utility;
use crate::vector;

use vector::Vec3 as Color;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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
            Material::Dielectric(mat) => mat.scatter(r_in, rec, attenuation, scattered),
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Metal { albedo, fuzz }
        } else {
            Metal { albedo, fuzz: 1.0 }
        }
    }

    pub fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let reflected = vector::reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = ray::Ray::new(
            rec.p,
            reflected + vector::Vec3::random_unit_vector() * self.fuzz,
        );
        *attenuation = self.albedo;
        return vector::dot(&scattered.direction(), &rec.normal) > 0.0;
    }
}
#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    pub fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = vector::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > utility::random_double(0.0, 1.0)
        {
            vector::reflect(&unit_direction, &rec.normal)
        } else {
            vector::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = ray::Ray::new(rec.p, direction);
        return true;
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}
