use std::ffi::CStr;
use libc::c_void;

use cassandra_sys::CassLogMessage;
use cassandra_sys::CassLogLevel;

// use cassandra_sys::cass_log_cleanup; @deprecated
use cassandra_sys::cass_log_level_string;
use cassandra_sys::cass_log_set_callback;
use cassandra_sys::cass_log_set_level;
// use cassandra_sys::cass_log_set_queue_size; @deprecated


#[repr(C)]
///The possible logging levels that can be set
pub struct LogLevel(CassLogLevel);


impl LogLevel {
    ///Gets the string for a log level.
    pub fn as_string(&self) -> String {
        unsafe { CStr::from_ptr(cass_log_level_string(self.0)).to_str().unwrap().to_owned() }
    }
}

/// A callback that's used to handle logging.
pub type CassLogCallback = Option<unsafe extern "C" fn(message: *const CassLogMessage, data: *mut c_void)>;

///Sets the log level.
///
///<b>Note:</b> This needs to be done before any call that might log, such as
///any of the cass_cluster_*() or cass_ssl_*() functions.
///<b>Default:</b> CASS_LOG_WARN
pub fn set_level(level: LogLevel) {
    unsafe { cass_log_set_level(level.0) }
}

///Sets a callback for handling logging events.
pub fn set_callback(callback: CassLogCallback, mut data: Vec<u8>) {
    unsafe { cass_log_set_callback(callback, &mut data as *mut _ as *mut c_void) }
}
