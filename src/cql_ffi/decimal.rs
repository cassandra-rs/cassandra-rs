#![allow(dead_code)]

use std::default::Default;
use std::mem;

use cql_bindgen::CassDecimal as _CassDecimal;
use cql_ffi::types::cass_int32_t;

#[repr(C)]
#[derive(Copy)]
pub struct CassDecimal(pub _CassDecimal);

impl Default for CassDecimal {
    fn default() -> CassDecimal { unsafe { mem::zeroed() } }
}

impl CassDecimal {
    //~ pub scale: cass_int32_t,
    //~ pub varint: *const cass_uint8_t,
    //~ pub varint_size: size_t,
    
    pub unsafe fn init(scale: cass_int32_t, varint: Vec<u8>) -> CassDecimal {
        let varint = varint.as_ptr();
        let decimal = _CassDecimal{scale:scale, varint:varint, varint_size:0};
//        CassDecimal(cass_decimal_init(scale,varint.0))
    CassDecimal(decimal)
    }
}
