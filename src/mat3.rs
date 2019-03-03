use std::ops::{Add, Sub, Mul};
use num_traits::{Zero, One};
use crate::vec3::Vec3;


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
        Self::Output { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Mat3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl One for Mat3 {
    fn one() -> Self {
        unimplemented!()
    }
}

impl Mul for Mat3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        unimplemented!()
    }
}

// impl 