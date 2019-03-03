use std::ops::{Add, Sub, Mul};
use num_traits::{Zero, One};
use crate::traits::{Normed, Randomizable};

//// Vec4
#[derive(Copy, Clone, PartialEq)]
pub struct Vec4 {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Zero for Vec4 {
    fn zero() -> Self {
        Self { w: f64::zero(), x: f64::zero(), y: f64::zero(), z: f64::zero()} 
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output { w: self.w + other.w, 
                       x: self.x + other.x, 
                       y: self.y + other.y, 
                       z: self.z + other.z }
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output { w: self.w - other.w, 
                       x: self.x - other.x, 
                       y: self.y - other.y, 
                       z: self.z - other.z }
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self::Output { w: self.w * other, 
                       x: self.x * other, 
                       y: self.y * other, 
                       z: self.z * other }
    }
}

impl Mul<Self> for Vec4 {
    type Output = Self;
    fn mul(self, other: Vec4) -> Self::Output {
        Self::Output { w: self.w * other.w, 
                       x: self.x * other.x, 
                       y: self.y * other.y, 
                       z: self.z * other.z }
    }
}

impl From<f64> for Vec4 {
    fn from(v: f64) -> Self {
        Self { w: v, x: v, y: v, z: v }
    }
}

impl Vec4 {
    fn sum(&self) -> f64 {
        self.w + self.x + self.y + self.z 
    }
}
impl Normed for Vec4 {
    type N = f64;

    fn norm(self) -> Self::N {
        (self * self).sum()
    }
}

impl Randomizable for Vec4 {
    fn rand() -> Self {
        Self{w: f64::rand(), x: f64::rand(), y: f64::rand(), z: f64::rand()}
    }
}
