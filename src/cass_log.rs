#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::types::os::arch::c95::c_uint;
use libc::types::common::c95::c_void;
use libc::types::os::arch::c95::c_char;

use cass_types::cass_uint64_t;
use cass_string::CassString;

type Enum_CassLogLevel_ = c_uint;
pub const CASS_LOG_DISABLED: c_uint = 0;
pub const CASS_LOG_CRITICAL: c_uint = 1;
pub const CASS_LOG_ERROR: c_uint = 2;
pub const CASS_LOG_WARN: c_uint = 3;
pub const CASS_LOG_INFO: c_uint = 4;
pub const CASS_LOG_DEBUG: c_uint = 5;
pub const CASS_LOG_TRACE: c_uint = 6;
pub const CASS_LOG_LAST_ENTRY: c_uint = 7;
pub type CassLogLevel = Enum_CassLogLevel_;

pub type CassLogCallback =
    Option<extern "C" fn (
        time_ms: cass_uint64_t, severity: CassLogLevel,
        message: CassString,
        data: *mut c_void
    )>;

extern "C" {
    pub fn cass_log_level_string(log_level: CassLogLevel) -> *const c_char;
}
