use cql_bindgen::CassFuture as _CassFuture;

use cql_ffi::error::CassError;
use cql_ffi::string::CassString;
use cql_ffi::result::CassResult;

use cql_bindgen::cass_future_wait;
use cql_bindgen::cass_future_error_code;
use cql_bindgen::cass_future_error_message;
use cql_bindgen::cass_future_get_result;

pub struct ResultFuture(pub *mut _CassFuture);

impl ResultFuture {
    pub fn wait(&mut self) -> Result<CassResult,CassError> {unsafe{cass_future_wait(self.0);self.error_code()}}
    pub fn error_code(&mut self) -> Result<CassResult,CassError> {unsafe{CassError::build(cass_future_error_code(self.0)).wrap(self.get())}}
    pub fn error_message(&mut self) -> CassString {unsafe{CassString(cass_future_error_message(self.0))}}
    pub fn get(&mut self) -> CassResult {unsafe{CassResult(cass_future_get_result(self.0))}}
}
