#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::common::c95::c_void;

use cql_ffi::prepared::CassPrepared;
use cql_ffi::error::CassError;
use cql_ffi::string::CassString;
use cql_ffi::session::CassSession;
use cql_ffi::result::CassResult;
use cql_ffi::types::cass_duration_t;
use cql_bindgen::CassFutureCallback as _CassFutureCallback;
use cql_bindgen::CassFuture as _CassFuture;
use cql_bindgen::cass_future_free;
use cql_bindgen::cass_future_error_message;
use cql_bindgen::cass_future_get_prepared;
use cql_bindgen::cass_future_get_result;
use cql_bindgen::cass_future_wait_timed;
use cql_bindgen::cass_future_wait;
use cql_bindgen::cass_future_ready;
use cql_bindgen::cass_future_set_callback;
use cql_bindgen::cass_future_error_code;

pub struct CassFuture(pub *mut _CassFuture);
pub struct PreparedFuture(pub *mut _CassFuture);
pub struct ResultFuture(pub *mut _CassFuture);
pub struct SessionFuture(pub *mut _CassFuture, pub CassSession);
pub struct CassFutureCallback(_CassFutureCallback);

impl PreparedFuture {
    pub fn wait(&mut self) -> Result<CassPrepared,CassError> {unsafe{cass_future_wait(self.0);self.error_code()}}
    pub fn error_code(&mut self) -> Result<CassPrepared,CassError> {unsafe{CassError::build(cass_future_error_code(self.0)).wrap(self.get())}}
    pub fn error_message(&mut self) -> CassString {unsafe{CassString(cass_future_error_message(self.0))}}
    pub fn get(&mut self) -> CassPrepared {unsafe{CassPrepared(cass_future_get_prepared(self.0))}}

}

impl ResultFuture {
    pub fn wait(&mut self) -> Result<CassResult,CassError> {unsafe{cass_future_wait(self.0);self.error_code()}}
    pub fn error_code(&mut self) -> Result<CassResult,CassError> {unsafe{CassError::build(cass_future_error_code(self.0)).wrap(self.get())}}
    pub fn error_message(&mut self) -> CassString {unsafe{CassString(cass_future_error_message(self.0))}}
    pub fn get(&mut self) -> CassResult {unsafe{CassResult(cass_future_get_result(self.0))}}
}

impl Drop for CassFuture {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl SessionFuture {
    pub fn wait(self) -> Result<CassSession,CassError> {unsafe{cass_future_wait(self.0);self.error_code()}}
    unsafe fn error_code(self) -> Result<CassSession,CassError> {CassError::build(cass_future_error_code(self.0)).wrap(self.1)}
}

impl CassFuture {
    unsafe fn free(&mut self) {cass_future_free(self.0)}
    pub unsafe fn set_callback<'a>(&'a mut self, callback: CassFutureCallback, data: *mut c_void) -> Result<&'a Self,CassError> {CassError::build(cass_future_set_callback(self.0, callback.0, data)).wrap(self)}
    pub unsafe fn ready(&mut self) -> bool {if (cass_future_ready(self.0)) > 0 {true} else {false}}
    pub fn wait(self) -> Result<Self,CassError> {unsafe{cass_future_wait(self.0);self.error_code()}}
    pub unsafe fn wait_timed(&mut self, timeout_us: cass_duration_t) -> bool {if cass_future_wait_timed(self.0, timeout_us) > 0 {true} else {false}}
    unsafe fn error_code(self) -> Result<Self,CassError> {CassError::build(cass_future_error_code(self.0)).wrap(self)}
    pub unsafe fn error_message(&mut self) -> CassString {CassString(cass_future_error_message(self.0))}
}
