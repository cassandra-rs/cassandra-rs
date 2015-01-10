#![allow(dead_code)]

use libc::types::os::arch::c95::c_char;

use cass_types::cass_uint64_t;
use cass_types::cass_uint8_t;

pub type CassUuid = [cass_uint8_t; 16us];

extern "C" {
    pub fn cass_uuid_generate_time(output: CassUuid);
    pub fn cass_uuid_from_time(time: cass_uint64_t, output: CassUuid);
    pub fn cass_uuid_min_from_time(time: cass_uint64_t, output: CassUuid);
    pub fn cass_uuid_max_from_time(time: cass_uint64_t, output: CassUuid);
    pub fn cass_uuid_generate_random(output: CassUuid);
    pub fn cass_uuid_timestamp(uuid: CassUuid) -> cass_uint64_t;
    pub fn cass_uuid_version(uuid: CassUuid) -> cass_uint8_t;
    pub fn cass_uuid_string(uuid: CassUuid, output: *mut c_char);
}
