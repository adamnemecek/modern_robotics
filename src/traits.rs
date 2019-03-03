pub trait SquareMatrix {
    type N;
    fn trace(&self) -> Self::N;
}