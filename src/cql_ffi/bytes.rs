#![allow(raw_pointer_derive)]
#![allow(dead_code)]

use cql_ffi::types::cass_byte_t;
use cql_ffi::types::cass_size_t;

#[repr(C)]
#[derive(Copy,Debug)]
pub struct CassBytes {
    pub data: *const cass_byte_t,
    pub size: cass_size_t,
}

impl ::std::default::Default for CassBytes {
    fn default() -> CassBytes { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn cass_bytes_init(data: *const cass_byte_t, size: cass_size_t) -> CassBytes;
}
