

// fn svd_1d()


use crate::traits::{SquareMatrix, Shaped};

pub struct SVD {

}

impl SVD {
    pub fn new<A: SquareMatrix + Shaped>(a: A, k: Option<usize>) -> Self {
        unimplemented!()
        // let shape = A::shape();

        // let k = k.unwrap_or(shape.0.min(shape.1));

        // for i in 0..k {
        //     let m = A.clone();

        // }
        
        // Self {}
    }
}
