#![allow(dead_code)]

use cql_ffi::types::cass_uint8_t;

#[repr(C)]
#[derive(Copy)]
pub struct CassInet {
    pub address: [cass_uint8_t; 16us],
    pub address_length: cass_uint8_t,
}

impl ::std::default::Default for CassInet {
    fn default() -> CassInet { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn cass_inet_init_v4(address: *const cass_uint8_t) -> CassInet;
    pub fn cass_inet_init_v6(address: *const cass_uint8_t) -> CassInet;
}
