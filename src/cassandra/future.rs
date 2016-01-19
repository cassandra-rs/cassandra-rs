use std::mem;
use std::str;
use std::slice;

use cassandra::session::Session;
use cassandra::error::CassError;
use cassandra::result::CassResult;
use cassandra::prepared::PreparedStatement;

use cassandra_sys::CassFuture as _Future;
// use cassandra_sys::CassResult as _CassResult;
use cassandra_sys::cass_future_free;
use cassandra_sys::cass_future_error_message;
use cassandra_sys::cass_future_wait_timed;
use cassandra_sys::cass_future_wait;
use cassandra_sys::cass_future_ready;
use cassandra_sys::cass_future_error_code;
use cassandra_sys::cass_future_get_result;
use cassandra_sys::cass_future_get_prepared;
use cassandra_sys::cass_future_custom_payload_item;
use cassandra_sys::cass_future_custom_payload_item_count;
use cassandra_sys::cass_future_get_error_result;
use cassandra_sys::cass_future_set_callback;
use cassandra_sys::CassFutureCallback as _CassFutureCallback;
use cassandra_sys::CassSession as _Session;
use cassandra::error::CassErrorResult;
use cassandra::error;
use cassandra::result;
use cassandra::prepared;
use cassandra::util::Protected;

use cassandra::session;
use libc::c_void;

use cassandra_sys::CASS_OK;

///A CQL Future representing the status of any asynchronous calls to Cassandra
pub struct Future(*mut _Future);

///A callback registered to execute when the future returns
pub struct FutureCallback(_CassFutureCallback);

impl Drop for Future {
    ///Frees a future instance. A future can be freed anytime.
    fn drop(&mut self) {
        unsafe { cass_future_free(self.0) }
    }
}

impl Future {
    ///Sets a callback that is called when a future is set
    pub fn set_callback(&mut self, callback: FutureCallback, data: *mut c_void) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_future_set_callback(self.0, callback.0, data), None).wrap(self) }
    }

    ///Gets the set status of the future.
    pub fn ready(&mut self) -> bool {
        unsafe { cass_future_ready(self.0) > 0 }
    }

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
    pub fn wait_timed(&mut self, timeout_us: u64) -> bool {
        unsafe { cass_future_wait_timed(self.0, timeout_us) > 0 }
    }

    /// Gets the result of a successful future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn get_result(&self) -> CassResult {
        unsafe { CassResult::build((cass_future_get_result(self.0))) }
    }

    /// Gets the error result from a future that failed as a result of a server error. If the
    ///future is not ready this method will wait for the future to be set.
    pub fn get_error_result(&self) -> CassErrorResult {
        unsafe { CassErrorResult::build(cass_future_get_error_result(self.0)) }
    }

    ///Gets the error code from future. If the future is not ready this method will
    // wait for the future to be set.
    fn error_code(self) -> Result<(), CassError> {
        unsafe { CassError::build(cass_future_error_code(self.0), None).wrap(()) }
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
    pub fn payload_item_count(&self) -> u64 {
        unsafe { cass_future_custom_payload_item_count(self.0) }
    }

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
///The future result of an operation.
///It can represent a result if the operation completed successfully or an
///error if the operation failed. It can be waited on, polled or a callback
///can be attached.
pub struct ResultFuture(*mut _Future);

impl Drop for ResultFuture {
    fn drop(&mut self) {
        unsafe { cass_future_free(self.0) }
    }
}

impl ResultFuture {
    ///Blocks until the future returns or times out
    pub fn wait(&mut self) -> Result<CassResult, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    ///Gets the error code from future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn error_code(&mut self) -> Result<CassResult, CassError> {
        unsafe {
            match self.get() {
                Some(result) => CassError::build(cass_future_error_code(self.0), None).wrap(result),
                None => panic!("FIXME"),
            }
        }
    }

    ///Gets the error message from future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn error_message(&mut self) -> String {
        unsafe {
            let message = mem::zeroed();
            let message_length = mem::zeroed();
            cass_future_error_message(self.0, message, message_length);

            let slice = slice::from_raw_parts(message as *const u8, message_length as usize);
            str::from_utf8(slice).unwrap().to_owned()
        }
    }



    ///Gets the result of a successful future. If the future is not ready this method will
    ///wait for the future to be set.
    ///a None response indicates that there was an error
    pub fn get(&mut self) -> Option<CassResult> {
        unsafe {
            let result = cass_future_get_result(self.0);
            if result.is_null() { None } else { Some((CassResult::build(result))) }
        }
    }
}


///The future result of an prepared statement.
///It can represent a result if the operation completed successfully or an
///error if the operation failed. It can be waited on, polled or a callback
///can be attached.
pub struct PreparedFuture(*mut _Future);

impl Drop for PreparedFuture {
    fn drop(&mut self) {
        unsafe { cass_future_free(self.0) }
    }
}

impl PreparedFuture {
    /// Wait for the future to be set with either a result or error.
    ///
    ///Important: Do not wait in a future callback. Waiting in a future
    ///callback will cause a deadlock.
    pub fn wait(&mut self) -> Result<PreparedStatement, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    ///Gets the error code from future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn error_code(&mut self) -> Result<PreparedStatement, CassError> {
        unsafe { CassError::build(cass_future_error_code(self.0), None).wrap(self.get()) }
    }

    ///Gets the error message from future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn get(&mut self) -> PreparedStatement {
        unsafe { PreparedStatement::build(cass_future_get_prepared(self.0)) }
    }
}

///The future result of an attempt to create a new Cassandra session
///It can be waited on, polled or a callback
///can be attached.
pub struct SessionFuture(*mut _Future);

impl SessionFuture {
    ///blocks until the session connects or errors out
    pub fn wait(&mut self) -> Result<(),CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    ///Gets the error code from future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn error_code(&self) -> Result<(), CassError> {
        unsafe {
            match self.get() {
                Some(result) => CassError::build(cass_future_error_code(self.0), None).wrap(()),
                None => panic!("FIXME"),
            }
        }
    }

    ///Gets the result of a successful future. If the future is not ready this method will
    ///wait for the future to be set.
    ///a None response indicates that there was an error
    pub fn get(&self) -> Option<CassResult> {
        unsafe {
            let result = cass_future_get_result(self.0);
            println!("result is null: {}", result.is_null());
            if result.is_null() { None } else { Some(CassResult::build(result)) }
        }
    }
}


///The future result of a session close statement.
///It can represent a result if the operation completed successfully or an
///error if the operation failed. It can be waited on, polled or a callback
///can be attached.
pub struct CloseFuture(*mut _Future);

impl Protected<*mut _Future> for Future {
    fn inner(&self) -> *mut _Future {
        self.0
    }
    fn build(inner: *mut _Future) -> Self {
        Future(inner)
    }
}

impl Protected<*mut _Future> for PreparedFuture {
    fn inner(&self) -> *mut _Future {
        self.0
    }
    fn build(inner: *mut _Future) -> Self {
        PreparedFuture(inner)
    }
}

impl Protected<*mut _Future> for ResultFuture {
    fn inner(&self) -> *mut _Future {
        self.0
    }
    fn build(inner: *mut _Future) -> Self {
        ResultFuture(inner)
    }
}

impl Protected<*mut _Future> for SessionFuture {
    fn inner(&self) -> *mut _Future {
        self.0
    }
    fn build(inner: *mut _Future) -> Self {
        SessionFuture(inner)
    }
}

impl Protected<*mut _Future> for CloseFuture {
    fn inner(&self) -> *mut _Future {
        self.0
    }
    fn build(inner: *mut _Future) -> Self {
        CloseFuture(inner)
    }
}


impl Drop for CloseFuture {
    fn drop(&mut self) {
        unsafe { cass_future_free(self.0) }
    }
}

impl CloseFuture {
    /// Wait for the future to be set with either a result or error.
    ///
    ///Important: Do not wait in a future callback. Waiting in a future
    ///callback will cause a deadlock.
    pub fn wait(&self) -> Result<PreparedStatement, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    ///Gets the error code from future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn error_code(&self) -> Result<PreparedStatement, CassError> {
        unsafe { CassError::build(cass_future_error_code(self.0), None).wrap(self.get()) }
    }

    ///Gets the error message from future. If the future is not ready this method will
    ///wait for the future to be set.
    pub fn get(&self) -> PreparedStatement {
        unsafe { PreparedStatement::build(cass_future_get_prepared(self.0)) }
    }
}
