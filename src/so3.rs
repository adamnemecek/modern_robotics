
use std::ops::{Add, Sub, Mul};
// use crate::se3::
#[derive(Copy, Clone, PartialEq)]
struct so3 {
    // m: Mat3
}

impl Mul<Self> for so3 {
    type Output = Self;

    fn mul(self, other: so3) -> Self::Output {
        unimplemented!()
    }
}

impl Mul<SO3> for so3 {
    type Output = so3;

    fn mul(self, other: SO3) -> Self::Output {
        unimplemented!()
    }
}

// impl Mul<f64> for so3 {

// }


#[derive(Copy, Clone, PartialEq)]
struct SO3 {
    // m: Mat3
}

