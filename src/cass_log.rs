#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(raw_pointer_derive)]

use libc::types::os::arch::c95::c_uint;
use libc::types::os::arch::c95::c_char;

use cass_types::cass_uint64_t;
use cass_types::cass_size_t;

#[repr(C)]
#[derive(Copy)]
pub enum CassLogLevel {
    DISABLED = 0,
    CRITICAL = 1,
    ERROR = 2,
    WARN = 3,
    INFO = 4,
    DEBUG = 5,
    TRACE = 6,
    LAST_ENTRY = 7
}

pub type CassLogCallback =
    ::std::option::Option<extern "C" fn (
        message: *const CassLogMessage,
        data: *mut ::libc::c_void
    )>;

#[repr(C)]
#[derive(Copy)]
pub struct CassLogMessage {
    pub time_ms: cass_uint64_t,
    pub severity: CassLogLevel,
    pub file: *const ::libc::c_char,
    pub line: ::libc::c_int,
    pub function: *const ::libc::c_char,
    pub message: [::libc::c_char; 256us],
}
impl ::std::default::Default for CassLogMessage {
    fn default() -> CassLogMessage { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn cass_log_level_string(log_level: CassLogLevel) -> *const c_char;
    pub fn cass_log_cleanup();
    pub fn cass_log_set_level(log_level: CassLogLevel);
    pub fn cass_log_set_callback(callback: CassLogCallback, data: *mut ::libc::c_void);
    pub fn cass_log_set_queue_size(queue_size: cass_size_t);

}
