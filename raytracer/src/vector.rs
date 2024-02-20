use std::cell::OnceCell;

use crate::utility::random_double;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f64 {
        return self.e[0];
    }

    pub fn y(&self) -> f64 {
        return self.e[1];
    }

    pub fn z(&self) -> f64 {
        return self.e[2];
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn unit_vector(&self) -> Vec3 {
        return *self / self.length();
    }

    pub fn random_unit_vector() -> Vec3 {
        return Self::random_in_unit_sphere().unit_vector();
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if dot(&on_unit_sphere, normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Self::new(
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        )
    }

    // currently only prints to stdout, might need to change
    pub fn print(&self) {
        println!("{} {} {}", self.e[0], self.e[1], self.e[2]);
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            e: [
                self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2],
            ],
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            e: [
                self.e[0] - _rhs.e[0],
                self.e[1] - _rhs.e[1],
                self.e[2] - _rhs.e[2],
            ],
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, ind: usize) -> &Self::Output {
        &self.e[ind]
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self::Output {
        if _rhs == 0.0 {
            panic!("Cannot divide by zero!");
        }
        Self {
            e: [self.e[0] / _rhs, self.e[1] / _rhs, self.e[2] / _rhs],
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] * _rhs, self.e[1] * _rhs, self.e[2] * _rhs],
        }
    }
}

impl std::ops::Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: i32) -> Self::Output {
        Self {
            e: [
                self.e[0] * _rhs as f64,
                self.e[1] * _rhs as f64,
                self.e[2] * _rhs as f64,
            ],
        }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2];
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    let new_e = [
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    ];
    return Vec3 { e: new_e };
}
