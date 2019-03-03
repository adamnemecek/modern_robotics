

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


mod vec3;
mod vec4;
mod vec6;
mod mat3;
mod mat4;

mod se3;
mod so3;
mod traits;

mod svd;


//// Vec4







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