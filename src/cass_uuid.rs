#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;

use cass_types::cass_uint64_t;
use cass_types::cass_uint8_t;

use cass_error::CassError;

#[repr(C)]
#[derive(Copy,Debug)]
pub struct CassUuid {
    pub time_and_version: cass_uint64_t,
    pub clock_seq_and_node: cass_uint64_t,
}
impl ::std::default::Default for CassUuid {
    fn default() -> CassUuid { unsafe { ::std::mem::zeroed() } }
}

pub enum CassUuidGen { }

extern "C" {
    pub fn cass_uuid_gen_new() -> *mut CassUuidGen;
    pub fn cass_uuid_gen_new_with_node(node: cass_uint64_t) -> *mut CassUuidGen;
    pub fn cass_uuid_gen_free(uuid_gen: *mut CassUuidGen);
    pub fn cass_uuid_gen_time(uuid_gen: *mut CassUuidGen, output: *mut CassUuid);
    pub fn cass_uuid_gen_random(uuid_gen: *mut CassUuidGen, output: *mut CassUuid);
    pub fn cass_uuid_gen_from_time(uuid_gen: *mut CassUuidGen, timestamp: cass_uint64_t, output: *mut CassUuid);
    pub fn cass_uuid_min_from_time(time: cass_uint64_t, output: *mut CassUuid);
    pub fn cass_uuid_max_from_time(time: cass_uint64_t, output: *mut CassUuid);
    pub fn cass_uuid_timestamp(uuid: CassUuid) -> cass_uint64_t;
    pub fn cass_uuid_version(uuid: CassUuid) -> cass_uint8_t;
    pub fn cass_uuid_string(uuid: CassUuid, output: *mut c_char);
    pub fn cass_uuid_from_string(str: *const ::libc::c_char, output: *mut CassUuid) -> CassError;
}
