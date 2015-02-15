#![allow(dead_code)]

use std::default::Default;
use std::mem;

use cql_bindgen::CassDecimal as _CassDecimal;
use cql_bindgen::cass_decimal_init;
use cql_ffi::bytes::CassBytes;
use cql_ffi::types::cass_int32_t;

#[repr(C)]
#[derive(Copy)]
pub struct CassDecimal(pub _CassDecimal);

impl Default for CassDecimal {
    fn default() -> CassDecimal { unsafe { mem::zeroed() } }
}

impl CassDecimal {
    pub unsafe fn init(scale: cass_int32_t, varint: CassBytes) -> CassDecimal {CassDecimal(cass_decimal_init(scale,varint.0))}
}
