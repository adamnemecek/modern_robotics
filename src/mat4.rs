use std::ops::{Add, Sub, Mul};
use num_traits::{Zero, One};
use crate::vec4::Vec4;
use crate::traits::SquareMatrix;


#[derive(Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub w: Vec4,
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4
}

impl Zero for Mat4 {
    #[inline]
    fn zero() -> Self {
        Self { w: Vec4::zero(), x: Vec4::zero(), y: Vec4::zero(), z: Vec4::zero()} 
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Add for Mat4 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self::Output { w: self.w + other.w, 
                       x: self.x + other.x, 
                       y: self.y + other.y, 
                       z: self.z + other.z }
    }
}

impl Sub for Mat4 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::Output { w: self.w - other.w,
                       x: self.x - other.x, 
                       y: self.y - other.y, 
                       z: self.z - other.z }
    }
}

impl One for Mat4 {
    #[inline]
    fn one() -> Self {
        let mut s = Self::zero();
        s.w.w = f64::one();
        s.x.x = f64::one();
        s.y.y = 1.0;
        s
    }
}

impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        unimplemented!()
    }
}

impl SquareMatrix for Mat4 {
    type N = f64;
    fn trace(&self) -> Self::N {
        self.w.w + self.x.x + self.y.y + self.z.z
    }

    type V = Vec4;
    fn diag(v: Self::V) -> Self {
        let mut s = Self::zero();
        s.w.w = v.w;
        s.x.x = v.x;
        s.y.y = v.y;
        s.z.z = v.z;
        s
    }
}