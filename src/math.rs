use std::{ops::{self, DerefMut, Deref}, fmt::Display};

use rand::{prelude::ThreadRng, Rng};
use rgraphics::colors::Color;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    elems: [f64; 3],
}

impl Vec3 {
    pub fn x(&self) -> f64 {self.elems[0]}
    pub fn y(&self) -> f64 {self.elems[1]}
    pub fn z(&self) -> f64 {self.elems[2]}

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    pub fn length_squared(&self) -> f64 {
        self.elems[0] * self.elems[0] + self.elems[1] * self.elems[1] + self.elems[2] * self.elems[2]
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { elems: [x, y, z] }
    }
    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 { elems: [ self.y() * v.z() - self.z() * v.y(),
                        self.z() * v.x() - self.x() * v.z(),
                        self.x() * v.y() - self.y() * v.x()] }
    }
    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
    }
    pub fn to_color(&self) -> Color {
        Color::new(self.x() as f32, self.y() as f32, self.z() as f32, 1.0)
    }
    pub fn rand_vec() -> Vec3 {
        let mut rnd = rand::thread_rng();
        Vec3 { elems: [rnd.gen_range(0.0..1.0), rnd.gen_range(0.0..1.0), rnd.gen_range(0.0..1.0)] }
    }
    pub fn rand_range_vec(min: f64, max: f64) -> Vec3 {
        let mut rnd = rand::thread_rng();
        Vec3 { elems: [rnd.gen_range(min..max),rnd.gen_range(min..max),rnd.gen_range(min..max) ] }
    }
    pub fn rand_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::new(0.0, 0.0, 0.0);
        loop {
            p = Vec3::rand_range_vec(-1.0, 1.0);
            if p.length_squared() >= 1.0 {continue;}
            break;
        }
        p
    }
    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        (f64::abs(self.x()) < s) && (f64::abs(self.y()) < s) && (f64::abs(self.z()) < s)
    }
    pub fn reflect(&self, v: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(&v) * *v
    }
    pub fn refract(&self, n: &Vec3, amount: f64) -> Vec3 {
        let ct = f64::min((-*self).dot(n), 1.0);
        let r_perp = amount * (*self + ct * *n);
        let r_paralell = -f64::sqrt(f64::abs(1.0 - r_perp.length_squared())) * *n;
        r_perp + r_paralell
    }
}

unsafe impl Sync for Vec3 {}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x(), self.y(), self.z())
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {elems: [-self.x(), -self.y(), -self.z()]}
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= 3 {panic!("Index {} is greater than Vec3 length.", index)}
        &self.elems[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= 3 {panic!("Index {} is greater than Vec3 length.", index)}
        &mut self.elems[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.elems[0] += rhs.x();
        self.elems[1] += rhs.y();
        self.elems[2] += rhs.z();
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {elems: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()]}
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3 {elems: [self.x() + rhs, self.y() + rhs, self.z() + rhs]}
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {elems: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]}
    }
}

impl ops::Sub<Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self - rhs.x(), self - rhs.y(), self - rhs.z())
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {elems: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()]}
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {elems: [self.x() * rhs, self.y() * rhs, self.z() * rhs]}
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x() * self, rhs.y() * self, rhs.z() * self)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {elems: [self.x() / rhs, self.y() / rhs, self.z() / rhs]}
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.elems[0] *= rhs.x();
        self.elems[1] *= rhs.y();
        self.elems[2] *= rhs.z();
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.elems[0] /= rhs.x();
        self.elems[1] /= rhs.y();
        self.elems[2] /= rhs.z();
    }
}



pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}


impl Ray {
    pub fn new(o: Vec3, dir: Vec3) -> Ray {
        Ray {origin: o, direction: dir}
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub type Point = Vec3;