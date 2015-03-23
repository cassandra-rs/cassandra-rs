use std::mem;
use std::str;
use std::slice;

use cql_ffi::error::CassError;
use cql_ffi::types::cass_duration_t;
use cql_bindgen::CassFuture as _CassFuture;
use cql_bindgen::cass_future_free;
use cql_bindgen::cass_future_error_message;
use cql_bindgen::cass_future_wait_timed;
use cql_bindgen::cass_future_wait;
use cql_bindgen::cass_future_ready;
use cql_bindgen::cass_future_error_code;

pub struct CassFuture(pub *mut _CassFuture);

impl Drop for CassFuture {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl CassFuture {
    unsafe fn free(&mut self) {
        cass_future_free(self.0)
    }
    
    //pub unsafe fn set_callback<'a>(&'a mut self, callback: CassFutureCallback, data: *mut c_void) -> Result<&'a Self,CassError> {CassError::build(cass_future_set_callback(self.0, callback.0, data)).wrap(self)}

    pub unsafe fn ready(&mut self) -> bool {
        if (cass_future_ready(self.0)) > 0 {true} else {false}
    }

    pub fn wait(self) -> Result<Self,CassError> {unsafe{
        cass_future_wait(self.0);self.error_code()
    }}

    pub unsafe fn wait_timed(&mut self, timeout_us: cass_duration_t) -> bool {
        if cass_future_wait_timed(self.0, timeout_us) > 0 {true} else {false}
    }

    unsafe fn error_code(self) -> Result<Self,CassError> {
        CassError::build(cass_future_error_code(self.0)).wrap(self)
    }

    pub fn error_message(&mut self) -> String {unsafe{
        let message = mem::zeroed();
        let message_length = mem::zeroed();
        cass_future_error_message(self.0, message, message_length);

        let slice = slice::from_raw_parts(message as *const u8,message_length as usize);
        str::from_utf8(slice).unwrap().to_string()
    }}

}
