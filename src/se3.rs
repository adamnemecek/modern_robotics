

use num_traits::{Zero, One};
use std::ops::Mul;

#[derive(Copy, Clone, PartialEq)]
struct se3 {
    // m: Mat4
}


#[derive(Copy, Clone, PartialEq)]
struct SE3 {
    // m: Mat4
}

impl Mul for SE3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        unimplemented!()
    }
}

impl One for SE3 {
    fn one() -> Self {
        unimplemented!()
    }
}