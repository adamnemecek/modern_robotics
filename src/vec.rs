
//mod lie;
//mod crate::util;
//mod crate::util::;

//use crate::util::{todo};
use crate::lie::{SO3};

use std::ops::Div;


#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(self) -> Self {
        self / self.norm()
    }
}

impl From<SO3> for Vec3 {
    fn from(v: SO3) -> Self {
//        todo()
        panic!("")
    }
}



impl Div<f32> for Vec3{
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self::Output {x: self.x / other, y: self.y / other, z: self.z / other}
    }
}





pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}