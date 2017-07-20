use cassandra_sys::CassLogLevel_;
use cassandra_sys::CassLogMessage;
// use cassandra_sys::cass_log_cleanup; @deprecated
use cassandra_sys::cass_log_set_callback;
use cassandra_sys::cass_log_set_level;
use cassandra::util::Protected;
// use cassandra_sys::cass_log_set_queue_size; @deprecated

use std::ffi::CStr;
use std::os::raw;
use slog;

/// The possible logging levels that can be set.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
#[allow(missing_docs)] // Meanings are defined in CQL documentation.
#[allow(non_camel_case_types)] // Names are traditional.
pub enum LogLevel {
    DISABLED,
    CRITICAL,
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE,
    LAST_ENTRY,
}

enhance_nullary_enum!(LogLevel, CassLogLevel_, {
    (DISABLED, CASS_LOG_DISABLED, "DISABLED"),
    (CRITICAL, CASS_LOG_CRITICAL, "CRITICAL"),
    (ERROR, CASS_LOG_ERROR, "ERROR"),
    (WARN, CASS_LOG_WARN, "WARN"),
    (INFO, CASS_LOG_INFO, "INFO"),
    (DEBUG, CASS_LOG_DEBUG, "DEBUG"),
    (TRACE, CASS_LOG_TRACE, "TRACE"),
    (LAST_ENTRY, CASS_LOG_LAST_ENTRY, "LAST_ENTRY"),
});

/// A callback that's used to handle logging.
pub type CassLogCallback = Option<unsafe extern "C" fn(message: *const CassLogMessage, data: *mut raw::c_void)>;

/// Sets the log level.
///
/// <b>Note:</b> This needs to be done before any call that might log, such as
/// any of the cass_cluster_*() or cass_ssl_*() functions.
/// <b>Default:</b> WARN
pub fn set_level(level: LogLevel) { unsafe { cass_log_set_level(level.inner()) } }

/// Sets a logger to receive all Cassandra driver logs.
pub fn set_logger(logger: slog::Logger) {
    unsafe {
        // TODO cass_log_set_callback(callback, &mut data as *mut _ as *mut raw::c_void)
    }
}
