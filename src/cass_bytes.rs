#![allow(raw_pointer_derive)]
#![allow(dead_code)]

use cass_types::cass_byte_t;
use cass_types::cass_size_t;

#[repr(C)]
#[derive(Copy)]
struct Struct_CassBytes_ {
    pub data: *const cass_byte_t,
    pub size: cass_size_t,
}

impl ::std::default::Default for Struct_CassBytes_ {
    fn default() -> Struct_CassBytes_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassBytes = Struct_CassBytes_;

extern "C" {
    pub fn cass_bytes_init(data: *const cass_byte_t, size: cass_size_t) -> CassBytes;
}
