use std::ops::{Add, Sub, Mul};
use num_traits::{Zero, One};
use crate::vec3::Vec3;
use crate::traits::SquareMatrix;


#[derive(Copy, Clone, PartialEq)]
pub struct Mat3 {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3
}

impl Zero for Mat3 {
    fn zero() -> Self {
        Self { x: Vec3::zero(), y: Vec3::zero(), z: Vec3::zero()} 
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Add for Mat3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output { x: self.x + other.x, 
                       y: self.y + other.y, 
                       z: self.z + other.z }
    }
}

impl Sub for Mat3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output { x: self.x - other.x, 
                       y: self.y - other.y, 
                       z: self.z - other.z }
    }
}

impl One for Mat3 {
    fn one() -> Self {
        Self::diag(Vec3::from(f64::one()))
    }
}

impl Mul for Mat3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        unimplemented!()
    }
}

impl SquareMatrix for Mat3 {
    type N = f64;
    fn trace(&self) -> Self::N {
        self.x.x + self.y.y + self.z.z
    }

    type V = Vec3;
    fn diag(v: Self::V) -> Self {
        let mut s = Self::zero();
        s.x.x = v.x;
        s.y.y = v.y;
        s.z.z = v.z;
        s
    }
}

// impl 