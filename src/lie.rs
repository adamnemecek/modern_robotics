


//use crate::util::{todo};
use crate::vec::{Vec3};

use crate::mat::{Mat3};

pub struct SO3 {
    m: Mat3,
}

impl From<Vec3> for SO3 {
    fn from(v: Vec3) -> Self {
//        todo()
        panic!("")
    }
}

impl SO3 {
    fn exp(self) -> Self {
//        todo()
        panic!("")
    }
}
