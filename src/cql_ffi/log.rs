#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(raw_pointer_derive)]

use cql_bindgen::CassLogMessage;

#[repr(C)]
#[derive(Copy,Clone)]
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

