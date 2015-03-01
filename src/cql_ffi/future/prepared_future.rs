use cql_bindgen::CassFuture as _CassFuture;

use cql_ffi::prepared::CassPrepared;
use cql_ffi::error::CassError;
use cql_ffi::string::CassString;

use cql_bindgen::cass_future_wait;
use cql_bindgen::cass_future_get_prepared;
use cql_bindgen::cass_future_error_message;
use cql_bindgen::cass_future_error_code;

pub struct PreparedFuture(pub *mut _CassFuture);

impl PreparedFuture {
    pub fn wait(&mut self) -> Result<CassPrepared,CassError> {unsafe{cass_future_wait(self.0);self.error_code()}}
    pub fn error_code(&mut self) -> Result<CassPrepared,CassError> {unsafe{CassError::build(cass_future_error_code(self.0)).wrap(self.get())}}
    pub fn error_message(&mut self) -> CassString {unsafe{CassString(cass_future_error_message(self.0))}}
    pub fn get(&mut self) -> CassPrepared {unsafe{CassPrepared(cass_future_get_prepared(self.0))}}

}
