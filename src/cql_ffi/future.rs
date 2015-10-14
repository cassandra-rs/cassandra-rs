use std::mem;
use std::str;
use std::slice;

use cql_ffi::error::CassError;
use cql_ffi::result::CassResult;
use cql_ffi::prepared::PreparedStatement;

use cql_bindgen::CassFuture as _Future;
use cql_bindgen::cass_future_free;
use cql_bindgen::cass_future_error_message;
use cql_bindgen::cass_future_wait_timed;
use cql_bindgen::cass_future_wait;
use cql_bindgen::cass_future_ready;
//use cql_bindgen::cass_future_set_callback;
use cql_bindgen::cass_future_error_code;
use cql_bindgen::cass_future_get_result;
use cql_bindgen::cass_future_get_prepared;

pub struct Future(pub *mut _Future);

impl Drop for Future {
    fn drop(&mut self) {
        unsafe {
            cass_future_free(self.0)
        }
    }
}

impl Future {

//    pub unsafe fn set_callback(&mut self, callback: FutureCallback, data: *mut c_void)
//        -> Result<&Self,CassError> {
//        CassError::build(cass_future_set_callback(self.0, callback.0, data)).wrap(self)
//    }

    pub fn ready(&mut self) -> bool {
        unsafe {
            cass_future_ready(self.0) > 0
        }
    }

    pub fn wait(self) -> Result<Self, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    pub fn wait_timed(&mut self, timeout_us: u64) -> bool {
        unsafe {
            cass_future_wait_timed(self.0, timeout_us) > 0
        }
    }

    fn error_code(self) -> Result<Self, CassError> {
        unsafe {
            CassError::build(cass_future_error_code(self.0)).wrap(self)
        }
    }

    pub fn error_message(&mut self) -> String {
        unsafe {
            let message = mem::zeroed();
            let message_length = mem::zeroed();
            cass_future_error_message(self.0, message, message_length);

            let slice: &[u8] = slice::from_raw_parts(message as *const u8, message_length as usize);
            str::from_utf8(slice).unwrap().to_owned()
        }
    }

}

pub struct ResultFuture(pub *mut _Future);

impl Drop for ResultFuture {
    fn drop(&mut self) {
        unsafe {
            cass_future_free(self.0)
        }
    }
}

impl ResultFuture {

    pub fn wait(&mut self) -> Result<CassResult, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    pub fn error_code(&mut self) -> Result<CassResult, CassError> {
        unsafe {
            CassError::build(cass_future_error_code(self.0)).wrap(self.get())
        }
    }

    pub fn error_message(&mut self) -> String {
        unsafe {
            let message = mem::zeroed();
            let message_length = mem::zeroed();
            cass_future_error_message(self.0, message, message_length);

            let slice = slice::from_raw_parts(message as *const u8, message_length as usize);
            str::from_utf8(slice).unwrap().to_owned()
        }
    }


    pub fn get(&mut self) -> CassResult {
        unsafe {
            CassResult(cass_future_get_result(self.0))
        }
    }
}


pub struct PreparedFuture(pub *mut _Future);

impl Drop for PreparedFuture {
    fn drop(&mut self) {
        unsafe {
            cass_future_free(self.0)
        }
    }
}

impl PreparedFuture {

    pub fn wait(&mut self) -> Result<PreparedStatement, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    pub fn error_code(&mut self) -> Result<PreparedStatement, CassError> {
        unsafe {
            CassError::build(cass_future_error_code(self.0)).wrap(self.get())
        }
    }

    pub fn error_message(&mut self) -> String {
        unsafe {
            let message = mem::zeroed();
            let message_length = mem::zeroed();
            cass_future_error_message(self.0, message, message_length);

            let slice = slice::from_raw_parts(message as *const u8, message_length as usize);
            str::from_utf8(slice).unwrap().to_owned()
        }
    }

    pub fn get(&mut self) -> PreparedStatement {
        unsafe {
            PreparedStatement(cass_future_get_prepared(self.0))
        }
    }

}
