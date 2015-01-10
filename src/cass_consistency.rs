#![allow(dead_code)]
#![allow(non_camel_case_types)]

use libc::types::os::arch::c95::c_uint;

type Enum_CassConsistency_ = c_uint;
pub const CASS_CONSISTENCY_ANY: c_uint = 0;
pub const CASS_CONSISTENCY_ONE: c_uint = 1;
pub const CASS_CONSISTENCY_TWO: c_uint = 2;
pub const CASS_CONSISTENCY_THREE: c_uint = 3;
pub const CASS_CONSISTENCY_QUORUM: c_uint = 4;
pub const CASS_CONSISTENCY_ALL: c_uint = 5;
pub const CASS_CONSISTENCY_LOCAL_QUORUM: c_uint = 6;
pub const CASS_CONSISTENCY_EACH_QUORUM: c_uint = 7;
pub const CASS_CONSISTENCY_SERIAL: c_uint = 8;
pub const CASS_CONSISTENCY_LOCAL_SERIAL: c_uint = 9;
pub const CASS_CONSISTENCY_LOCAL_ONE: c_uint = 10;
pub type CassConsistency = Enum_CassConsistency_;
