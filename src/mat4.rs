use std::ops::{Add, Sub, Mul};
use num_traits::{Zero, One};
use crate::vec4::Vec4;


#[derive(Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub w: Vec4,
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4
}
