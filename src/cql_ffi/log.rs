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

pub type CassLogCallback =
    ::std::option::Option<extern "C" fn (
        message: *const CassLogMessage,
        data: *mut ::libc::c_void
    )>;
