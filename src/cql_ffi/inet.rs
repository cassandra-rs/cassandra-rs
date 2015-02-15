#![allow(dead_code)]

use cql_ffi::types::cass_uint8_t;
use cql_bindgen::CassInet as _CassInet;
use cql_bindgen::cass_inet_init_v4;
use cql_bindgen::cass_inet_init_v6;

#[repr(C)]
#[derive(Copy)]
pub struct CassInet(pub _CassInet);

impl ::std::default::Default for CassInet {
    fn default() -> CassInet { unsafe { ::std::mem::zeroed() } }
}

impl CassInet {
    pub unsafe fn cass_inet_init_v4(address: *const cass_uint8_t) -> CassInet {CassInet(cass_inet_init_v4(address))}
    pub unsafe fn cass_inet_init_v6(address: *const cass_uint8_t) -> CassInet {CassInet(cass_inet_init_v6(address))}
}
