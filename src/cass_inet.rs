#![allow(dead_code)]

use cass_types::cass_uint8_t;

#[repr(C)]
#[derive(Copy)]
struct Struct_CassInet_ {
    pub address: [cass_uint8_t; 16us],
    pub address_length: cass_uint8_t,
}

impl ::std::default::Default for Struct_CassInet_ {
    fn default() -> Struct_CassInet_ { unsafe { ::std::mem::zeroed() } }
}

pub type CassInet = Struct_CassInet_;

extern "C" {
    pub fn cass_inet_init_v4(address: *const cass_uint8_t) -> CassInet;
    pub fn cass_inet_init_v6(address: *const cass_uint8_t) -> CassInet;
}
