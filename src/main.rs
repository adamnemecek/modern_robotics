

//mod util;
//mod imp;
//mod lie;

mod vec;
mod mat;
mod lie;

use std::f32;

use lie::{SO3};
use vec::{Vec3};
use mat::{Mat3};

//use crate::util::{todo};





struct Ang3 {

}



impl From<Vec3> for Ang3 {
    fn from(v: Vec3) -> Self {
//        todo()
        panic!("")
    }

}



pub struct RotMat3 {
    m: Mat3
}

impl RotMat3 {
    fn log() -> Self {
//        todo()
        panic!("")
    }
}


fn main() {
    println!("Hello, world!");
}
