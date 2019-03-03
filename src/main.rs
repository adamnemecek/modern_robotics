

// //mod util;
// //mod imp;
// //mod lie;

// mod vec;
// mod mat;
// mod lie;

// use std::f32;

// use lie::{SO3};
// use vec::{Vec3};
// use mat::{Mat3};

// //use crate::util::{todo};





// struct Ang3 {

// }



// impl From<Vec3> for Ang3 {
//     fn from(v: Vec3) -> Self {
// //        todo()
//         panic!("")
//     }

// }



// pub struct RotMat3 {
//     m: Mat3
// }

// impl RotMat3 {
//     fn log() -> Self {
// //        todo()
//         panic!("")
//     }
// }

// fn max1() -> i32 {
//     let v: Vec<i32> = vec![0, 1];
//     let q = v.max();
//     println!("{}", q);
//     10
// }

// fn main() {
// //    println!("Hello, world!");
//     println!("{}",max1())
// }

// typedef double mr_SO3_t[9];  // 3x3 matrix
// typedef double mr_so3_t[9];  // 3x3 matrix
// typedef double mr_SE3_t[16]; // 4x4 matrix
// typedef double mr_se3_t[16]; // 4x4 matrix
// typedef double mr_vec3_t[3]; // 3-vector (e.g., angular velocity)
// typedef double mr_vec6_t[6]; // 6-vector (e.g., wrench or twist)
// typedef double mr_mat6_t[36];// 6x6 matrix

use std::ops::{Add, Sub};
use num_traits::Zero;

//// Vec4

#[derive(Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Zero for Vec3 {
    fn zero() -> Self {
        Self { x: f64::zero(), y: f64::zero(), z: f64::zero()} 
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

//// Vec4
#[derive(Copy, Clone, PartialEq)]
pub struct Vec4 {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output { w: self.w + other.w, x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Zero for Vec4 {
    fn zero() -> Self {
        Self { w: f64::zero(), x: f64::zero(), y: f64::zero(), z: f64::zero()} 
    }

    fn is_zero(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vec6 {
    pub x: Vec3,
    pub y: Vec3,
}

impl Add for Vec6 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vec6 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Zero for Vec6 {
    fn zero() -> Self {
        Self { x: Vec3::zero(), y: Vec3::zero() } 
    }

    fn is_zero(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Mat3 {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3
}

impl Add for Mat3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub w: Vec4,
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4
}

// struct se3 {
//     data
// }


// typedef double mr_SO3_t[9];  // 3x3 matrix
// typedef double mr_so3_t[9];  // 3x3 matrix
// typedef double mr_SE3_t[16]; // 4x4 matrix
// typedef double mr_se3_t[16]; // 4x4 matrix

struct so3 {
    m: Mat3
}

struct SO3 {
    m: Mat3
}

struct se3 {
    m: Mat4
}

struct SE3 {
    m: Mat4
}


// fn add(a: &[i64], )
// struct Vec3 {
//     data: [i64; 3]
// }


// impl Vec3 {
    
    
//     fn new(v: [i64; 3]) -> Self {
//         Self{ 0: v }
//     }

//     fn from(x: i64, y: i64, z: i64) {
//         unimplemented!()
//     }
// }

// impl Add for Vec3 {
    // fn add(&self, other: &Self) -> Self {
        // Self::from()
    // }
// }




// struct so3(Vec3, Vec3, Vec3);

// fn a(v: Vec3) {
//     v.0[2];
// }

fn main() {
    // let a = Vec3::new(0,2,3);
    // // a.0
}