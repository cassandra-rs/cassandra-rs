pub trait Protected<T> {
    fn build(inner: T) -> Self;
    fn inner(&self) -> T;
}
