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
use cql_bindgen::cass_future_error_code;
use cql_bindgen::cass_future_get_result;
use cql_bindgen::cass_future_get_prepared;
use cql_bindgen::cass_future_custom_payload_item;
use cql_bindgen::cass_future_custom_payload_item_count;
use cql_bindgen::cass_future_get_error_result;
use cql_bindgen::cass_future_set_callback;
use cql_bindgen::CassFutureCallback as _CassFutureCallback;
use cql_ffi::error::CassErrorResult;

use libc::c_void;

use cql_bindgen::CASS_OK;

pub struct Future(pub *mut _Future);
pub struct FutureCallback(pub _CassFutureCallback);

impl Drop for Future {
    ///Frees a future instance. A future can be freed anytime.
    fn drop(&mut self) { unsafe { cass_future_free(self.0) } }
}

impl Future {
    ///Sets a callback that is called when a future is set
    pub fn set_callback(&mut self, callback: FutureCallback, data: *mut c_void) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_future_set_callback(self.0, callback.0, data)).wrap(self) }
    }

    ///Gets the set status of the future.
    pub fn ready(&mut self) -> bool { unsafe { cass_future_ready(self.0) > 0 } }

    /// Wait for the future to be set with either a result or error.
    ///
    ///Important: Do not wait in a future callback. Waiting in a future
    ///callback will cause a deadlock.
    pub fn wait(self) -> Result<(), CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    ///Wait for the future to be set or timeout.
    pub fn wait_timed(&mut self, timeout_us: u64) -> bool { unsafe { cass_future_wait_timed(self.0, timeout_us) > 0 } }

    /// Gets the result of a successful future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn get_result(&self) -> CassResult { unsafe { CassResult(cass_future_get_result(self.0)) } }

    /// Gets the error result from a future that failed as a result of a server error. If the
    ///future is not ready this method will wait for the future to be set.
    pub fn get_error_result(&self) -> CassErrorResult {
        unsafe { CassErrorResult(cass_future_get_error_result(self.0)) }
    }

    ///Gets the error code from future. If the future is not ready this method will
    // wait for the future to be set.
    fn error_code(self) -> Result<(), CassError> {
        unsafe { CassError::build(cass_future_error_code(self.0)).wrap(()) }
    }

    ///Gets the error message from future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn error_message(&mut self) -> String {
        unsafe {
            let message = mem::zeroed();
            let message_length = mem::zeroed();
            cass_future_error_message(self.0, message, message_length);

            let slice: &[u8] = slice::from_raw_parts(message as *const u8, message_length as usize);
            str::from_utf8(slice).unwrap().to_owned()
        }
    }


    /// Gets a the number of custom payload items from a response future. If the future is not
    /// ready this method will wait for the future to be set.
    pub fn payload_item_count(&self) -> u64 { unsafe { cass_future_custom_payload_item_count(self.0) } }

    ///Gets a custom payload item from a response future at the specified index. If the future is not
    ///ready this method will wait for the future to be set.
    pub fn payload_item(&self, index: u64) -> Result<(String, String), u32> {
        unsafe {
            let name = mem::zeroed();
            let name_length = mem::zeroed();
            let value = mem::zeroed();
            let value_length = mem::zeroed();
            match cass_future_custom_payload_item(self.0, index, name, name_length, value, value_length) {
                CASS_OK => {
                    let name: &[u8] = slice::from_raw_parts(name as *const u8, name_length as usize);
                    let value: &[u8] = slice::from_raw_parts(value as *const u8, value_length as usize);
                    Ok((str::from_utf8(name).unwrap().to_owned(),
                        str::from_utf8(value).unwrap().to_owned()))
                }
                err => Err(err),
            }

        }
    }
}

#[must_use]
pub struct ResultFuture(pub *mut _Future);

impl Drop for ResultFuture {
    fn drop(&mut self) { unsafe { cass_future_free(self.0) } }
}

impl ResultFuture {
    pub fn wait(&mut self) -> Result<CassResult, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    pub fn error_code(&mut self) -> Result<CassResult, CassError> {
        unsafe { CassError::build(cass_future_error_code(self.0)).wrap(self.get()) }
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


    pub fn get(&mut self) -> CassResult { unsafe { CassResult(cass_future_get_result(self.0)) } }
}


pub struct PreparedFuture(pub *mut _Future);

impl Drop for PreparedFuture {
    fn drop(&mut self) { unsafe { cass_future_free(self.0) } }
}

impl PreparedFuture {
    pub fn wait(&mut self) -> Result<PreparedStatement, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    pub fn error_code(&mut self) -> Result<PreparedStatement, CassError> {
        unsafe { CassError::build(cass_future_error_code(self.0)).wrap(self.get()) }
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

    pub fn get(&mut self) -> PreparedStatement { unsafe { PreparedStatement(cass_future_get_prepared(self.0)) } }
}
