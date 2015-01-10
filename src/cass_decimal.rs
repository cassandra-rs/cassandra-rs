#![allow(dead_code)]

use cass_bytes::CassBytes;
use cass_types::cass_int32_t;

#[repr(C)]
#[derive(Copy)]
struct Struct_CassDecimal_ {
    pub scale: cass_int32_t,
    pub varint: CassBytes,
}

impl ::std::default::Default for Struct_CassDecimal_ {
    fn default() -> Struct_CassDecimal_ { unsafe { ::std::mem::zeroed() } }
}

pub type CassDecimal = Struct_CassDecimal_;

extern "C" {
    pub fn cass_decimal_init(scale: cass_int32_t, varint: CassBytes) -> CassDecimal;
}
