use std::ops::{Add, Sub, Mul, Div};
use num_traits::{Zero, One};
use crate::traits::{Normed, Randomizable};

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

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self::Output { x: self.x * other, 
                       y: self.y * other, 
                       z: self.z * other }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self::Output { x: self.x / other, 
                       y: self.y / other, 
                       z: self.z / other }
    }
}


impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Self::Output {
        Self::Output { x: self.x * other.x, 
                       y: self.y * other.y, 
                       z: self.z * other.z }
    }
}

impl From<f64> for Vec3 {
    fn from(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }
}

impl Vec3 {
    fn sum(&self) -> f64 {
        self.x + self.y + self.z 
    }
}
impl Normed for Vec3 {
    type N = f64;

    fn norm(self) -> Self::N {
        (self * self).sum()
    }
}

impl Randomizable for Vec3 {
    fn rand() -> Self {
        Self{x: f64::rand(), y: f64::rand(), z: f64::rand()}
    }
}
