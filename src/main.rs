

use std::f32;
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

pub struct Mat3 {
    r1: Vec3,
    r2: Vec3,
    r3: Vec3,
}

impl Div<f32> for Vec3{
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Self::Output {x: self.x / other, y: self.y / other, z: self.z / other}
    }
}


fn main() {
    println!("Hello, world!");
}
