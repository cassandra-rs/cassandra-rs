#[repr(C)]

use std::convert::Into;

#[derive(Debug,Copy,Clone)]
pub enum CassCollectionType {
    LIST = 32,
    MAP = 33,
    SET = 34
}

impl Into<i64> for CassCollectionType {
    fn into(self) -> i64 {
        self as i64
    }
}
