pub trait SquareMatrix {
    type N;
    fn trace(&self) -> Self::N;

    type V;
    fn diag(v: Self::V) -> Self;

    fn transpose(&self) -> Self;

    
}
// 
// fn determinant(&self) -> N;
// fn pinvadjoin(&self) -> Self;
// fn inv(&self) -> Self;
// fn pinv(&self) -> Self


pub trait Normed {
    type N;
    fn norm(self) -> Self::N;
}

