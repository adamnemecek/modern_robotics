pub trait Shaped {
    fn shape() -> (usize, usize);
}

pub trait SquareMatrix {

    

    type N;
    fn trace(&self) -> Self::N;

    type V;
    fn diag(v: Self::V) -> Self;

    fn transpose(&self) -> Self;

    fn det(&self) -> Self::N;
}
// 

// fn pinvadjoin(&self) -> Self;
// fn inv(&self) -> Self;
// fn pinv(&self) -> Self


pub trait Normed {
    type N;
    fn norm(self) -> Self::N;
}

pub trait Randomizable {
    fn rand() -> Self;
}

extern crate rand;
use rand::Rng;

impl Randomizable for f64 {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}

