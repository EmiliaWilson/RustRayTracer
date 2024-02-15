mod vector {

    #[derive(Copy, Clone)]
    pub struct Vec3 {
        pub e: [f64; 3],
    }

    impl Vec3 {
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
}

fn main() {
    // Image

    // let image_width = 256;
    // let image_height = 256;

    // // render
    // println!("P3\n{image_width} {image_height}\n255\n");

    // for s in 0..image_height {
    //     for t in 0..image_width {
    //         let r = (t as f64) / ((image_width - 1) as f64);
    //         let g = (s as f64) / ((image_height - 1) as f64);
    //         let b = 0.0;

    //         let ir = 259.999 * r;
    //         let ig = 259.999 * g;
    //         let ib = 259.000 * b;

    //         println!("{ir} {ig} {ib}\n");
    //     }
    // }
    let vect = vector::Vec3 { e: [4.0, 1.0, 1.0] };

    let vect2 = vector::Vec3 { e: [1.0, 1.0, 2.0] };
    let vect_sum = vector::cross(&vect, &vect2);
    vect_sum.print();
}
