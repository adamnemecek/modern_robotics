pub trait SquareMatrix {
    type N;
    fn trace(&self) -> Self::N;

    type V;
    fn diag(v: Self::V) -> Self;
}