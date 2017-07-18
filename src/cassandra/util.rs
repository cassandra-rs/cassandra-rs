/// Interconvert between external and internal representation.
/// We can freely do this within this crate, but must not allow
/// our clients to do this.
pub(crate) trait Protected<T> {
    fn build(inner: T) -> Self;
    fn inner(&self) -> T;
}
