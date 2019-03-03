use std::ops::{Add, Sub, Mul, Div};
use num_traits::{Zero, One};
use crate::mat4::Mat4;
use crate::traits::{Normed, Randomizable, OuterProduct};
use std::fmt;

//// Vec4
#[derive(Copy, Clone, PartialEq)]
pub struct Vec4 {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Vec4 {
    fn sum(&self) -> f64 {
        self.w + self.x + self.y + self.z 
    }

    #[inline]
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }
}

impl Zero for Vec4 {
    fn zero() -> Self {
        Self::new(f64::zero(), f64::zero(), f64::zero(), f64::zero()) 
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.w + other.w, 
                          self.x + other.x, 
                          self.y + other.y, 
                          self.z + other.z)
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.w - other.w, 
                          self.x - other.x, 
                          self.y - other.y, 
                          self.z - other.z)
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

impl Div<f64> for Vec4 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self::Output { w: self.w / other, 
                       x: self.x / other, 
                       y: self.y / other, 
                       z: self.z / other }
    }
}

/// outer product
impl Mul<Self> for Vec4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::Output { w: self.w * other.w, 
                       x: self.x * other.x, 
                       y: self.y * other.y, 
                       z: self.z * other.z }
    }
}

impl OuterProduct for Vec4 {
    type Output = Mat4;
    fn outer(self, other: Self) -> Self::Output {
        Self::Output { w: other * self.w,
                       x: other * self.x,
                       y: other * self.y,
                       z: other * self.z }
    }
}

impl fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " [{:?}, {:?}, {:?}, {:?}]", self.w, self.x, self.y, self.z)
    }
}

#[test]
pub fn test_vec4_outer() {
    let a = Vec4::new(1.0,2.0,3.0, 4.0);
    let b = Vec4::new(2.0,3.0,4.0, 5.0);
    let result = a.outer(b);
    let expected = Mat4::new(Vec4::new(2.0, 3.0, 4.0, 5.0),
                            Vec4::new(4.0, 6.0, 8.0, 10.0),
                            Vec4::new(6.0, 9.0, 12.0, 15.0),
                            Vec4::new(8.0, 12.0, 16.0, 20.0));
    assert_eq!(expected, result);
}

impl From<f64> for Vec4 {
    fn from(v: f64) -> Self {
        Self { w: v, x: v, y: v, z: v }
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
