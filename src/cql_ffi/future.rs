#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::common::c95::c_void;

use cql_ffi::prepared::CassPrepared;
use cql_ffi::error::CassError;
use cql_ffi::string::CassString;
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
pub struct CassFutureCallback(_CassFutureCallback);

impl CassFuture {
    pub unsafe fn free(&mut self) {cass_future_free(self.0)}
    pub unsafe fn set_callback(&mut self, callback: CassFutureCallback, data: *mut c_void) -> Result<(),CassError> {CassError::build(cass_future_set_callback(self.0, callback.0, data))}
    pub unsafe fn ready(&mut self) -> bool {if (cass_future_ready(self.0)) > 0 {true} else {false}}
    pub unsafe fn wait(&mut self) {cass_future_wait(self.0)}
    pub unsafe fn wait_timed(&mut self, timeout_us: cass_duration_t) -> bool {if cass_future_wait_timed(self.0, timeout_us) > 0 {true} else {false}}
    pub unsafe fn get_result(&mut self) -> CassResult {CassResult(cass_future_get_result(self.0))}
    pub unsafe fn get_prepared(&mut self) -> CassPrepared {CassPrepared(cass_future_get_prepared(self.0))}
    pub unsafe fn error_code(&mut self) -> Result<(),CassError> {CassError::build(cass_future_error_code(self.0))}
    pub unsafe fn error_message(&mut self) -> CassString {CassString(cass_future_error_message(self.0))}
}
