#![allow(raw_pointer_derive)]
#![allow(dead_code)]

use cql_ffi::types::cass_byte_t;
use cql_ffi::types::cass_size_t;

pub use cql_bindgen::CassBytes as _CassBytes;

#[derive(Copy)]
pub struct CassBytes(pub _CassBytes);

use cql_bindgen::cass_bytes_init;

impl CassBytes {
    pub unsafe fn init(data: *const cass_byte_t, size: cass_size_t) -> CassBytes {CassBytes(cass_bytes_init(data,size))}
}
