#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(raw_pointer_derive)]

use libc::types::os::arch::c95::c_uint;
use libc::types::os::arch::c95::c_char;

use cass_types::cass_uint64_t;
use cass_types::cass_size_t;

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
    ::std::option::Option<extern "C" fn (
        message: *const CassLogMessage,
        data: *mut ::libc::c_void
    )>;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_CassLogMessage_ {
    pub time_ms: cass_uint64_t,
    pub severity: CassLogLevel,
    pub file: *const ::libc::c_char,
    pub line: ::libc::c_int,
    pub function: *const ::libc::c_char,
    pub message: [::libc::c_char; 256us],
}
impl ::std::default::Default for Struct_CassLogMessage_ {
    fn default() -> Struct_CassLogMessage_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassLogMessage = Struct_CassLogMessage_;

extern "C" {
    pub fn cass_log_level_string(log_level: CassLogLevel) -> *const c_char;
    pub fn cass_log_cleanup();
    pub fn cass_log_set_level(log_level: CassLogLevel);
    pub fn cass_log_set_callback(callback: CassLogCallback, data: *mut ::libc::c_void);
    pub fn cass_log_set_queue_size(queue_size: cass_size_t);

}
