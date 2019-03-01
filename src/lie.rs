
#![feature(range_contains)]

//use crate::util::{todo};
use crate::vec::{Vec3};
use std::ops::{Range};

use crate::mat::{Mat3};



pub struct so3 {
    pub m: Mat3
}

impl From<Vec3> for so3 {
    fn from(v: Vec3) -> Self {
        let r1 = Vec3{x: 0.0, y: -v.z, z: v.y};
        let r2 = Vec3{x: v.z, y: 0.0, z: -v.x};
        let r3 = Vec3{x: -v.y, y: v.x, z: 0.0};
        let m = Mat3{r1: r1, r2: r2,r3: r3};
        Self{m: m}
    }

}

pub struct SO3 {
    m: Mat3,
}




//impl From<Vec3> for SO3 {
//    fn from(v: Vec3) -> Self {
////        todo()
//        panic!("")
//    }
//}

impl SO3 {
    fn exp(self) -> Self {
//        todo()
//        let q = (2...3).contains(&10);
        panic!("")
    }
}

pub struct se3 {

}

pub struct SE3 {

}