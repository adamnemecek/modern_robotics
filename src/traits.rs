pub trait SquareMatrix {
    type N;
    fn trace(&self) -> Self::N;

    // type Vec;
    // fn diag(v: Vec) -> Self;
}