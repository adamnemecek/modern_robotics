use crate::vec3::Vec3;
use std::ops::{Add, Sub, Mul};
use num_traits::{Zero, One};

#[derive(Copy, Clone, PartialEq)]
pub struct Vec6 {
    pub x: Vec3,
    pub y: Vec3,
}

impl Zero for Vec6 {
    fn zero() -> Self {
        Self { x: Vec3::zero(), y: Vec3::zero() } 
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Add for Vec6 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vec6 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output { x: self.x - other.x, y: self.y - other.y }
    }
}



