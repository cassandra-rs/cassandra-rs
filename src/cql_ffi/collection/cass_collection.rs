#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub enum CassCollectionType {
    LIST = 32isize,
    MAP = 33,
    SET = 34
}
