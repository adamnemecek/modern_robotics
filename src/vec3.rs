use std::ops::{Add, Sub, Mul};
use num_traits::{Zero, One};

#[derive(Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Zero for Vec3 {
    fn zero() -> Self {
        Self { x: f64::zero(), y: f64::zero(), z: f64::zero()} 
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl From<f64> for Vec3 {
    fn from(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }
}