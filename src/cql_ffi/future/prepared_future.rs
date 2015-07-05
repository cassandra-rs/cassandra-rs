use std::mem;
use std::slice;
use std::str;

use cql_bindgen::CassFuture as _CassFuture;

use cql_ffi::prepared::CassPrepared;
use cql_ffi::error::CassError;

use cql_bindgen::cass_future_free;
use cql_bindgen::cass_future_wait;
use cql_bindgen::cass_future_get_prepared;
use cql_bindgen::cass_future_error_message;
use cql_bindgen::cass_future_error_code;

pub struct PreparedFuture(pub *mut _CassFuture);

impl Drop for PreparedFuture {
    fn drop(&mut self) {unsafe{
        cass_future_free(self.0)
    }}
}

impl PreparedFuture {

    pub fn wait(&mut self) -> Result<CassPrepared,CassError> {unsafe{cass_future_wait(self.0);self.error_code()}}

    pub fn error_code(&mut self) -> Result<CassPrepared,CassError> {unsafe{
        CassError::build(cass_future_error_code(self.0)).wrap(self.get())
    }}

    pub fn error_message(&mut self) -> String {unsafe{
        let message = mem::zeroed();
        let message_length = mem::zeroed();
        cass_future_error_message(self.0, message, message_length);

        let slice = slice::from_raw_parts(message as *const u8,message_length as usize);
        str::from_utf8(slice).unwrap().to_string()
    }}

    pub fn get(&mut self) -> CassPrepared {unsafe{
        CassPrepared(cass_future_get_prepared(self.0))
    }}

}
