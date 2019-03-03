
use std::ops::{Add, Sub, Mul};
// use crate::se3::
#[derive(Copy, Clone, PartialEq)]
struct so3 {
    // m: Mat3
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

