#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::common::c95::c_void;

use cass_prepared::CassPrepared;
use cass_error::CassError;
use cass_string::CassString;
use cass_result::CassResult;
use cass_types::cass_bool_t;
use cass_types::cass_duration_t;

#[derive(Show)]
pub enum CassFuture { }

pub type CassFutureCallback =
    Option<extern "C" fn (
        future: *mut CassFuture,
        data: *mut c_void
    )>;

extern "C" {
    pub fn cass_future_free(future: *mut CassFuture);
    pub fn cass_future_set_callback(future: *mut CassFuture, callback: CassFutureCallback, data: *mut c_void) -> CassError;
    pub fn cass_future_ready(future: *mut CassFuture) -> cass_bool_t;
    pub fn cass_future_wait(future: *mut CassFuture);
    pub fn cass_future_wait_timed(future: *mut CassFuture, timeout_us: cass_duration_t) -> cass_bool_t;
    pub fn cass_future_get_result(future: *mut CassFuture) -> *const CassResult;
    pub fn cass_future_get_prepared(future: *mut CassFuture) -> *const CassPrepared;
    pub fn cass_future_error_code(future: *mut CassFuture) -> CassError;
    pub fn cass_future_error_message(future: *mut CassFuture) -> CassString;
}
