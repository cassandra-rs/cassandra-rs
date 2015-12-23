use std::ffi::CStr;
use libc::c_void;

use cql_bindgen::CassLogMessage;

use cql_bindgen::cass_log_cleanup;
use cql_bindgen::cass_log_level_string;
use cql_bindgen::cass_log_set_callback;
use cql_bindgen::cass_log_set_level;
use cql_bindgen::cass_log_set_queue_size;

#[repr(C)]
pub enum LogLevel {
    DISABLED = 0,
    CRITICAL = 1,
    ERROR = 2,
    WARN = 3,
    INFO = 4,
    DEBUG = 5,
    TRACE = 6,
    LAST_ENTRY = 7,
}

impl LogLevel {
    pub fn level_string(self) -> String {
        unsafe { CStr::from_ptr(cass_log_level_string(self as u32)).to_str().unwrap().to_owned() }
    }
}
pub type CassLogCallback = ::std::option::Option<extern "C" fn(message: *const CassLogMessage,
                                                                 data: *mut ::libc::c_void)
                                                                >;

pub fn cleanup() { unsafe { cass_log_cleanup() } }
pub fn set_level(level: LogLevel) { unsafe { cass_log_set_level(level as u32) } }
pub fn set_queue_size(size: u64) { unsafe { cass_log_set_queue_size(size) } }
pub fn set_callback(callback: CassLogCallback, mut data: Vec<u8>) {
    unsafe { cass_log_set_callback(callback, &mut data as *mut _ as *mut c_void) }
}
