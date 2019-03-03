use std::ops::{Add, Sub, Mul};
use num_traits::{Zero, One, Inv};
use crate::vec4::Vec4;
use crate::traits::{SquareMatrix, Shaped};
use std::fmt;


#[derive(Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub w: Vec4,
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4
}

impl Mat4 {
    pub fn new(w: Vec4, x: Vec4, y: Vec4, z: Vec4) -> Self {
        Self {w, x, y, z}
    }
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
        Self::diag(Vec4::from(f64::one()))
    }
}

impl Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Mul<f64> for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f64) -> Self::Output {
        Self::Output { w: self.w * other, 
                       x: self.x * other, 
                       y: self.y * other, 
                       z: self.z * other }
    }
}

impl Inv for Mat4 {
    type Output = Self;

    fn inv(self) -> Self::Output {
        let det = self.det();
        if det.is_zero() {
            
        }
        self
    }
}

impl fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " [{:?}, {:?}, {:?}, {:?}]", self.w, self.x, self.y, self.z)
    }
}

impl Shaped for Mat4 {
    fn shape() -> (usize, usize) {
        (4, 4)
    }
}

impl SquareMatrix for Mat4 {
    type N = f64;

    #[inline]
    fn trace(&self) -> Self::N {
        self.w.w + self.x.x + self.y.y + self.z.z
    }

    type V = Vec4;

    #[inline]
    fn diag(v: Self::V) -> Self {
        let mut s = Self::zero();
        s.w.w = v.w;
        s.x.x = v.x;
        s.y.y = v.y;
        s.z.z = v.z;
        s
    }

    #[inline]
    fn transpose(&self) -> Self {
        // let x = Vec4 { w }
        unimplemented!()
    }

    fn det(&self) -> Self::N {
        unimplemented!()
    }
}