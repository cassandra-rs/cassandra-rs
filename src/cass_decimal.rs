#![allow(dead_code)]

use cass_bytes::CassBytes;
use cass_types::cass_int32_t;

#[repr(C)]
#[derive(Copy)]
pub struct CassDecimal {
    pub scale: cass_int32_t,
    pub varint: CassBytes,
}

impl ::std::default::Default for CassDecimal {
    fn default() -> CassDecimal { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn cass_decimal_init(scale: cass_int32_t, varint: CassBytes) -> CassDecimal;
}
