use crate::cassandra_sys::CassLogLevel_;
use crate::cassandra_sys::CassLogMessage;
// use cassandra_sys::cass_log_cleanup; @deprecated
use crate::cassandra::util::Protected;
use crate::cassandra_sys::cass_log_set_callback;
use crate::cassandra_sys::cass_log_set_level;
use crate::cassandra_sys::CassLogCallback;
// use cassandra_sys::cass_log_set_queue_size; @deprecated

#[cfg(feature = "slog")]
use slog;
use std::borrow::Borrow;
use std::boxed::Box;
use std::ffi::CStr;
use std::os::raw;
use std::ptr;

/// The possible logging levels that can be set.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Hash)]
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

/// Sets the log level.
///
/// <b>Note:</b> This needs to be done before any call that might log, such as
/// any of the cass_cluster_*() or cass_ssl_*() functions.
/// <b>Default:</b> WARN
pub fn set_level(level: LogLevel) {
    unsafe { cass_log_set_level(level.inner()) }
}

/// unset the logger that receives all Cassandra driver logs.
pub fn unset_logger() {
    unsafe { cass_log_set_callback(None, ptr::null_mut()) }
}

#[cfg(feature = "slog")]
/// Called by Cassandra for every log if logging is enabled. Passes the log to the configured
/// slog logger.
unsafe extern "C" fn slog_callback(log: *const CassLogMessage, data: *mut raw::c_void) {
    let log = &*log;
    let logger: &slog::Logger = &*(data as *const _);

    let message: &str = &CStr::from_ptr(log.message.as_ptr()).to_string_lossy();
    let time_ms: u64 = log.time_ms;
    let file: &str = &CStr::from_ptr(log.file).to_string_lossy();
    let line: i32 = log.line;
    let function: &str = &CStr::from_ptr(log.function).to_string_lossy();
    let kv = o!(
        "time_ms" => time_ms,
        "file" => file,
        "line" => line,
        "function" => function
    );

    // Issue the correct level of log call. Sadly even though the `log!` macro exists,
    // it's fundamental to slog that the log level is statically known for a given invocation.
    // We can't do that in this case, so we have to use this tedious workaround.
    match log.severity {
        CassLogLevel_::CASS_LOG_DISABLED | CassLogLevel_::CASS_LOG_CRITICAL => {
            crit!(logger, "{}", message; kv)
        }
        CassLogLevel_::CASS_LOG_ERROR => error!(logger, "{}", message; kv),
        CassLogLevel_::CASS_LOG_WARN => warn!(logger, "{}", message; kv),
        CassLogLevel_::CASS_LOG_INFO => info!(logger, "{}", message; kv),
        CassLogLevel_::CASS_LOG_DEBUG => debug!(logger, "{}", message; kv),
        CassLogLevel_::CASS_LOG_TRACE | CassLogLevel_::CASS_LOG_LAST_ENTRY => {
            trace!(logger, "{}", message; kv)
        }
    };
}

#[doc(hidden)]
#[deprecated(note = "Please use set_slog_logger instead")]
#[cfg(feature = "slog")]
/// Set or unset a logger to receive all Cassandra driver logs.
pub fn set_logger(logger: Option<slog::Logger>) {
    if let Some(logger) = logger {
        set_slog_logger(logger);
    } else {
        unset_logger();
    }
}

#[cfg(feature = "slog")]
/// Set a [slog](https://docs.rs/slog/latest/slog) logger to receive all Cassandra driver logs.
pub fn set_slog_logger(logger: slog::Logger) {
    unsafe {
        // Pass ownership to C. In fact we leak the logger; it never gets freed.
        // We don't expect this to be called many times, so we're not worried.
        let data = Box::new(logger);
        cass_log_set_callback(Some(slog_callback), Box::into_raw(data) as _)
    }
}

#[cfg(feature = "log")]
/// Called by Cassandra for every log if logging is enabled. Passes the log to the configured
/// log logger.
unsafe extern "C" fn log_callback(log: *const CassLogMessage, _data: *mut raw::c_void) {
    use log::{logger, Level, Record};

    let log = &*log;
    let message: &str = &CStr::from_ptr(log.message.as_ptr()).to_string_lossy();
    let file = &CStr::from_ptr(log.file).to_string_lossy();
    let line = log.line as u32;
    let function = &CStr::from_ptr(log.function).to_string_lossy();
    let module_and_function_name = function_definition_to_module_name(function).unwrap_or(function);

    let level = match log.severity {
        CassLogLevel_::CASS_LOG_DISABLED | CassLogLevel_::CASS_LOG_CRITICAL => Level::Error,
        CassLogLevel_::CASS_LOG_ERROR => Level::Error,
        CassLogLevel_::CASS_LOG_WARN => Level::Warn,
        CassLogLevel_::CASS_LOG_INFO => Level::Info,
        CassLogLevel_::CASS_LOG_DEBUG => Level::Debug,
        CassLogLevel_::CASS_LOG_TRACE | CassLogLevel_::CASS_LOG_LAST_ENTRY => Level::Trace,
    };

    logger().log(
        &Record::builder()
            .level(level)
            .args(format_args!("{}", message))
            .line(Some(line))
            .file(Some(file))
            .module_path(Some(module_and_function_name))
            .target(module_and_function_name)
            .build(),
    );
}

/// Extract the module name from a cpp function definition
fn function_definition_to_module_name(definition: &str) -> Option<&str> {
    // definition strings look like:
    // void datastax::internal::core::ControlConnection::handle_refresh_keyspace(datastax::internal::core::RefreshKeyspaceCallback*))
    let mut definition_iter = definition.split(' ');
    // skip the return type
    definition_iter.next()?;
    // return the module + function name
    // we include the function name with the module because we may as well keep the extra information and it doesnt impair log readability like the return type + args do
    definition_iter.next()?.split('(').next()
}

#[cfg(feature = "log")]
/// Set [log](https://docs.rs/log/latest/log) to receive all Cassandra driver logs.
/// By default [tracing](https://docs.rs/tracing/latest/tracing) will pick up logs emitted by `log`, so also use this if you are a tracing user.
pub fn set_log_logger() {
    unsafe { cass_log_set_callback(Some(log_callback), ptr::null_mut()) }
}
