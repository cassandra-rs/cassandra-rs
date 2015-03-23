#[repr(C)]
#[derive(Debug,Copy)]
pub enum CassCollectionType {
    LIST = 32isize,
    MAP = 33,
    SET = 34
}
