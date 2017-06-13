use Session;

use cassandra::error::{CassError, CassErrorResult};
use cassandra::prepared::PreparedStatement;
use cassandra::result::CassResult;
use cassandra::util::Protected;
use cassandra_sys::CASS_OK;

use cassandra_sys::CassFuture as _Future;
use cassandra_sys::CassFutureCallback as _CassFutureCallback;
use cassandra_sys::cass_future_custom_payload_item;
use cassandra_sys::cass_future_custom_payload_item_count;
use cassandra_sys::cass_future_error_code;
use cassandra_sys::cass_future_error_message;
// use cassandra_sys::CassResult as _CassResult;
use cassandra_sys::cass_future_free;
use cassandra_sys::cass_future_get_error_result;
use cassandra_sys::cass_future_get_prepared;
use cassandra_sys::cass_future_get_result;
use cassandra_sys::cass_future_ready;
use cassandra_sys::cass_future_set_callback;
use cassandra_sys::cass_future_wait;
use cassandra_sys::cass_future_wait_timed;

use cassandra_sys::cass_true;
use errors::*;
use std::mem;
use std::os::raw;
use std::slice;
use std::str;
use std::result;
use futures;

/// A CQL Future representing the status of any asynchronous calls to Cassandra
#[derive(Debug)]
pub struct Future(*mut _Future);

/// A callback registered to execute when the future returns
#[derive(Debug)]
pub struct FutureCallback(_CassFutureCallback);

impl Drop for Future {
    /// Frees a future instance. A future can be freed anytime.
    fn drop(&mut self) { unsafe { cass_future_free(self.0) } }
}

impl Future {
    /// Sets a callback that is called when a future is set
    pub unsafe fn set_callback(&mut self, callback: FutureCallback, data: *mut raw::c_void) -> Result<&mut Self> {
        cass_future_set_callback(self.0, callback.0, data).to_result(self).chain_err(|| "")
    }

    /// Gets the set status of the future.
    pub fn ready(&mut self) -> bool { unsafe { cass_future_ready(self.0) == cass_true } }

    /// Wait for the future to be set with either a result or error.
    ///
    /// Important: Do not wait in a future callback. Waiting in a future
    /// callback will cause a deadlock.
    pub fn wait(self) -> Result<()> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    /// Wait for the future to be set or timeout.
    pub fn wait_timed(&mut self, timeout_us: u64) -> bool {
        unsafe { cass_future_wait_timed(self.0, timeout_us) == cass_true }
    }

    /// Gets the result of a successful future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn get_result(&self) -> CassResult {
        unsafe {
            let ret = cass_future_get_result(self.0);
            if ret.is_null() {
                panic!("Attempted to get result of failed future")
            } else {
                CassResult::build(ret)
            }
        }
    }

    /// Gets the error result from a future that failed as a result of a server error. If the
    /// future is not ready this method will wait for the future to be set.
    pub fn get_error_result(&self) -> CassErrorResult {
        unsafe {
            let ret = cass_future_get_error_result(self.0);
            if ret.is_null() {
                panic!("Attempted to get error result of successful future")
            } else {
                CassErrorResult::build(ret)
            }
        }
    }

    /// Gets the error code from future. If the future is not ready this method will
    // wait for the future to be set.
    fn error_code(self) -> Result<()> { unsafe { cass_future_error_code(self.0).to_result(()).chain_err(|| "") } }

    /// Gets the error message from future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn error_message(&mut self) -> String {
        unsafe {
            let message = mem::zeroed();
            let message_length = mem::zeroed();
            cass_future_error_message(self.0, message, message_length);

            let slice: &[u8] = slice::from_raw_parts(message as *const u8, message_length as usize);
            str::from_utf8(slice).expect("must be utf8").to_owned()
        }
    }


    /// Gets a the number of custom payload items from a response future. If the future is not
    /// ready this method will wait for the future to be set.
    pub fn payload_item_count(&self) -> usize { unsafe { cass_future_custom_payload_item_count(self.0) } }

    /// Gets a custom payload item from a response future at the specified index. If the future is not
    /// ready this method will wait for the future to be set.
    pub fn payload_item(&self, index: usize) -> Result<(String, String)> {
        unsafe {
            let name = mem::zeroed();
            let name_length = mem::zeroed();
            let value = mem::zeroed();
            let value_length = mem::zeroed();
            match cass_future_custom_payload_item(self.0, index, name, name_length, value, value_length) {
                CASS_OK => {
                    Ok((str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize))
                        .expect("must be utf8")
                        .to_owned(),
                        str::from_utf8(slice::from_raw_parts(value as *const u8, value_length as usize))
                        .expect("must be utf8")
                        .to_owned()))
                }
                err => Err(err.to_result("").unwrap().into()),
            }

        }
    }
}

#[must_use]
/// The future result of an operation.
/// It can represent a result if the operation completed successfully or an
/// error if the operation failed. It can be waited on, polled or a callback
/// can be attached.
#[derive(Debug)]
pub struct ResultFuture {
    inner: *mut _Future,
    state: FutureTarget,
}

/// The target of a C++ Cassandra driver callback.
///
/// The C++ Cassandra driver calls the callback in the following two scenarios only:
///
/// * When the future is completed, if a callback is set.
/// * When a callback is set, if the future is completed.
///
/// Given a future can only be completed once, and a callback can only be set once, enforced
/// by internal locking, it is clear that we should expect at most one callback to occur.
///
/// The important thing to ensure is that Rust has not freed the target of that callback when
/// it occurs. The simplest way to achieve that is only to free the callback target once the
/// callback has occurred. Since the callback target is owned by the future, and the future is
/// typically freed after completion, that means we must only complete after we receive the
/// callback.
///
/// TODO However there is a remaining concern - if the caller abandons the future it may be freed
/// before the callback arrives.
#[derive(Debug)]
struct FutureTarget {
    inner: FutureState,
}

/// The state of the Cassandra future.
///
#[derive(Debug)]
enum FutureState {
    /// The future has been created but no callback has yet been installed.
    Created,

    /// The future has been created and a callback has been installed, invoking this task.
    Awaiting(futures::task::Task),

    /// The future has called back to indicate completion.
    Ready,
}

impl Drop for ResultFuture {
    fn drop(&mut self) { unsafe { cass_future_free(self.inner) };
        println!("********* future ------------ - droppingfuture {:p}", self as *mut ResultFuture);
    }
}

impl ResultFuture {
    /// Sets a callback that is called when a future is set
    unsafe fn set_callback(&mut self, callback: FutureCallback, data: *mut raw::c_void) -> Result<&mut Self> {
        cass_future_set_callback(self.inner, callback.0, data).to_result(self).chain_err(|| "while setting callback")
    }

    /// Gets the set status of the future.
    fn ready(&mut self) -> bool { unsafe { cass_future_ready(self.inner) == cass_true } }

    /// Blocks until the future returns or times out
    pub fn wait(&mut self) -> Result<CassResult> {
        unsafe {
            cass_future_wait(self.inner);
            self.error_code()
        }
    }

    /// Gets the error code from future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn error_code(&mut self) -> Result<CassResult> {
        unsafe {
            let x = self.get();
            let error_code = cass_future_error_code(self.inner);
            match (x, error_code) {
                (Some(x), _) => Ok(x),
                (None, err) => match err.to_result(()) {
                    Ok(_) => unimplemented!(),
                    Err(e) => Err(Error::with_chain(e, ErrorKind::CassandraError)),
                }
            }
        }
    }

    /// Gets the error message from future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn error_message(&mut self) -> String {
        unsafe {
            let message = mem::zeroed();
            let message_length = mem::zeroed();
            cass_future_error_message(self.inner, message, message_length);

            let slice = slice::from_raw_parts(message as *const u8, message_length as usize);
            str::from_utf8(slice).expect("must be utf8").to_owned()
        }
    }

    /// Gets the result from a future (whether success or error). If the future is
    /// not ready this method will wait for the future to be set.
    fn get_completion(&self) -> Result<CassResult> {
        unsafe {
            let ret = cass_future_get_result(self.inner);
            if ret.is_null() {
                let ret = cass_future_get_error_result(self.inner);
                if ret.is_null() {
                    panic!("Unexpected double null");
                } else {
                    CassErrorResult::build(ret).to_result(())
                        .chain_err(|| ErrorKind::CassandraError)
                        .map(|_| panic!("must fail"))
                }
            } else {
                Ok(CassResult::build(ret))
            }
        }
    }

    /// Gets the result of a successful future. If the future is not ready this method will
    /// wait for the future to be set.
    /// a None response indicates that there was an error
    pub fn get(&mut self) -> Option<CassResult> {
        unsafe {
            let result = cass_future_get_result(self.inner);
            if result.is_null() {
                None
            } else {
                Some((CassResult::build(result)))
            }
        }
    }
}

/// Callback which wakes the task waiting on this future.
/// Called by the C++ driver when the future is ready,
/// with a pointer to the `ResultFuture`.
unsafe extern "C" fn notify_task(_c_future: *mut _Future, data: *mut ::std::os::raw::c_void) {
    println!("********* future ------------ - awaking future {:p}", data);
    let future_target: &mut FutureTarget = &mut *(data as *mut FutureTarget);
    // The future is now ready, so transition to the appropriate state.
    // TODO Does this need a memory barrier to ensure we see the Awaiting state?
    let state = mem::replace(&mut future_target.inner, FutureState::Ready);
    if let FutureState::Awaiting(ref task) = state {
        println!("                                      actual {:p}", task);
        task.notify();
    } else {
        /// This can never happen.
        panic!("Callback invoked before callback set");
    }
}

impl futures::Future for ResultFuture {
    type Item = CassResult;
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        println!("Polling   future {:p}", self);
        match self.state.inner {
            FutureState::Created => {
                // No task yet - schedule a callback. But as an optimization, if it's ready
                // already, complete now without scheduling a callback.
                if self.ready() {
                    // Future is ready; wrap success in `Ok(Ready)` or report failure as `Err`.
                    println!("    Ready future {:p}", self);
                    self.get_completion().map(futures::Async::Ready)
                } else {
                    // Future is not ready; park this task and arrange to be called back when it is.
                    // If callback cannot be sent, report immediate `Err`.
                    // The C code will call the callback immediately if the future is ready, so we
                    // don't need to worry about the race between `self.ready()` and
                    // `self.set_callback`.
                    self.state.inner = FutureState::Awaiting(futures::task::current());
                    unsafe {
                        let data = (&mut self.state as *mut FutureTarget) as *mut ::std::os::raw::c_void;
                        println!(" NotReady future {:p} - parking future {:p}", self, data);
                        self.set_callback(FutureCallback(Some(notify_task)), data)
                    }.map(|_| futures::Async::NotReady)
                }
            },
            FutureState::Awaiting(_) => {
                // Callback already scheduled; don't set it again (C doesn't support it anyway),
                // but be sure to swizzle the new task into place.
                // Do NOT check self.ready(); if there is a task in place, completion must come via
                // the callback or else we risk dropping the future (and hence the task) before the
                // callback arrives.
                println!("  UnReady future {:p} - swizzling task", self);
                self.state.inner = FutureState::Awaiting(futures::task::current());
                Ok(futures::Async::NotReady)
            },
            FutureState::Ready => {
                // Future has been marked ready by callback. Safe to return now.
                println!(" Complete future {:p}", self);
                self.get_completion().map(futures::Async::Ready)
            }
        }
    }
}

/// The future result of an prepared statement.
/// It can represent a result if the operation completed successfully or an
/// error if the operation failed. It can be waited on, polled or a callback
/// can be attached.
#[derive(Debug)]
pub struct PreparedFuture(*mut _Future);

impl Drop for PreparedFuture {
    fn drop(&mut self) { unsafe { cass_future_free(self.0) } }
}

impl PreparedFuture {
    /// Wait for the future to be set with either a result or error.
    ///
    /// Important: Do not wait in a future callback. Waiting in a future
    /// callback will cause a deadlock.
    pub fn wait(&mut self) -> Result<PreparedStatement> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    /// Gets the error code from future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn error_code(&mut self) -> Result<PreparedStatement> {
        unsafe { cass_future_error_code(self.0).to_result(self.get()).chain_err(|| "") }
    }

    /// Gets the error message from future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn get(&mut self) -> PreparedStatement { unsafe { PreparedStatement::build(cass_future_get_prepared(self.0)) } }
}

#[derive(Debug)]
pub struct ConnectFuture(*mut _Future);

impl Protected<*mut _Future> for ConnectFuture {
    fn inner(&self) -> *mut _Future { self.0 }
    fn build(inner: *mut _Future) -> Self { ConnectFuture(inner) }
}

impl Drop for ConnectFuture {
    fn drop(&mut self) { unsafe { cass_future_free(self.0) } }
}
/// The future result of an attempt to create a new Cassandra session
/// It can be waited on, polled or a callback
/// can be attached.
#[derive(Debug)]
pub struct SessionFuture(*mut _Future);

impl SessionFuture {
    /// blocks until the session connects or errors out
    pub fn wait(&mut self) -> Result<()> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    /// Gets the error code from future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn error_code(&self) -> Result<()> { unsafe { cass_future_error_code(self.0).to_result(()).chain_err(|| "") } }

    /// Gets the result of a successful future. If the future is not ready this method will
    /// wait for the future to be set.
    /// a None response indicates that there was an error
    pub fn get(&self) -> Option<CassResult> {
        unsafe {
            let result = cass_future_get_result(self.0);
            if result.is_null() {
                None
            } else {
                Some(CassResult::build(result))
            }
        }
    }
}

impl Drop for SessionFuture {
    fn drop(&mut self) { unsafe { cass_future_free(self.0) } }
}

/// The future result of a session close statement.
/// It can represent a result if the operation completed successfully or an
/// error if the operation failed. It can be waited on, polled or a callback
/// can be attached.
#[derive(Debug)]
pub struct CloseFuture(*mut _Future);

impl Protected<*mut _Future> for Future {
    fn inner(&self) -> *mut _Future { self.0 }
    fn build(inner: *mut _Future) -> Self { Future(inner) }
}

impl Protected<*mut _Future> for PreparedFuture {
    fn inner(&self) -> *mut _Future { self.0 }
    fn build(inner: *mut _Future) -> Self { PreparedFuture(inner) }
}

impl Protected<*mut _Future> for ResultFuture {
    fn inner(&self) -> *mut _Future { self.inner }
    fn build(inner: *mut _Future) -> Self { ResultFuture { inner, state: FutureTarget { inner: FutureState::Created } } }
}

impl Protected<*mut _Future> for SessionFuture {
    fn inner(&self) -> *mut _Future { self.0 }
    fn build(inner: *mut _Future) -> Self { SessionFuture(inner) }
}

impl Protected<*mut _Future> for CloseFuture {
    fn inner(&self) -> *mut _Future { self.0 }
    fn build(inner: *mut _Future) -> Self { CloseFuture(inner) }
}


impl Drop for CloseFuture {
    fn drop(&mut self) { unsafe { cass_future_free(self.0) } }
}

impl CloseFuture {
    /// Wait for the future to be set with either a result or error.
    ///
    /// Important: Do not wait in a future callback. Waiting in a future
    /// callback will cause a deadlock.
    pub fn wait(&self) -> Result<()> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    /// Gets the error code from future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn error_code(&self) -> Result<()> { unsafe { cass_future_error_code(self.0).to_result(()).chain_err(|| "") } }

    /// Gets the error message from future. If the future is not ready this method will
    /// wait for the future to be set.
    pub fn get(&self) -> PreparedStatement { unsafe { PreparedStatement::build(cass_future_get_prepared(self.0)) } }
}
