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
use std::sync::{Arc, Mutex};
use futures;

#[must_use]
/// The future result of an operation.
/// It can represent a result if the operation completed successfully or an
/// error if the operation failed. It can be waited on, polled or a callback
/// can be attached.
#[derive(Debug)]
pub struct ResultFuture {
    inner: *mut _Future,
    state: Arc<FutureTarget>,
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
/// The FutureTarget is held by a ResultFuture, and (in the FutureState::Awaiting state) by
/// a C++ Cassandra driver callback. The latter pointer is represented by one inside that state,
/// so that it is not freed early.
#[derive(Debug)]
struct FutureTarget {
    inner: Mutex<FutureState>,
}

/// The state of the corresponding ResultFuture.
///
/// This is an FSM.
#[derive(Debug)]
enum FutureState {
    /// Initial state: the future has been created but no callback has yet been installed.
    Created,

    /// The future has been created and a callback has been installed, invoking this task.
    /// `keep_alive` is an Arc to the enclosing target (i.e., a cycle) which stands for the
    /// pointer held by the C++ Cassandra driver future as the callback target. This prevents
    /// it from being freed early.
    Awaiting { task: futures::task::Task, keep_alive: Arc<FutureTarget> },

    /// The future has called back to indicate completion.
    Ready,
}

impl Drop for ResultFuture {
    /// Drop this ResultFuture.
    ///
    /// This also drops its reference to the FutureTarget, but if
    /// we're waiting to be called back the FutureState::Awaiting holds another reference to
    /// the target, which keeps it alive until the callback fires.
    fn drop(&mut self) { unsafe { cass_future_free(self.inner) }; }
}

impl ResultFuture {
    /// Sets a callback that is called when a future is set
    unsafe fn set_callback(&self, callback: _CassFutureCallback, data: *mut raw::c_void) -> Result<&Self> {
        cass_future_set_callback(self.inner, callback, data).to_result(self).chain_err(|| "while setting callback")
    }

    /// Gets the set status of the future.
    fn ready(&self) -> bool { unsafe { cass_future_ready(self.inner) == cass_true } }

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
    let future_target: &FutureTarget = &*(data as *const FutureTarget);
    // The future is now ready, so transition to the appropriate state.
    let mut lock = future_target.inner.lock().expect("notify_task");
    let state = mem::replace(&mut *lock, FutureState::Ready);
    if let FutureState::Awaiting { ref task, .. } = state {
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
        let mut install_callback = false;
        let ret = {
            // Perform the following computations under the lock, and then release it.
            //
            // We must take care to avoid deadlock. The lock hierarchy is: take the Rust lock
            // (self.state.inner) first, then take the C++ lock (internal to the C++
            // implementation of futures).
            //
            // Poll is always called by the Rust event loop, never from within C++ code or
            // from notify_task. `self.ready()` and `self.get_completion()` take an internal
            // mutex within C++ code, but they never call back to Rust and so cannot violate
            // the lock hierarchy. However `self.set_callback()` calls into Rust code
            // (`notify_task`) if the future is already complete, so we must avoid holding the
            // Rust lock while calling it. We achieve this by using a boolean flag to request
            // the callback be set outside the lock region.
            let mut lock = self.state.as_ref().inner.lock().expect("poll");
            match *lock {
                ref mut state @ FutureState::Created => {
                    // No task yet - schedule a callback. But as an optimization, if it's ready
                    // already, complete now without scheduling a callback.
                    //
                    if self.ready() {
                        // Future is ready; wrap success in `Ok(Ready)` or report failure as `Err`.
                        self.get_completion().map(futures::Async::Ready)
                    } else {
                        // Future is not ready; park this task and arrange to be called back when
                        // it is.
                        *state = FutureState::Awaiting {
                            task: futures::task::current(),
                            keep_alive: self.state.clone(),
                        };
                        install_callback = true;
                        Ok(futures::Async::NotReady)
                    }
                },
                FutureState::Awaiting { ref mut task, .. } => {
                    // Callback already scheduled; don't set it again (C doesn't support it anyway),
                    // but be sure to swizzle the new task into place. No need to check for
                    // readiness here; we have to wait for the callback anyway so we might as well
                    // do all the work in one place.
                    *task = futures::task::current();
                    Ok(futures::Async::NotReady)
                },
                FutureState::Ready => {
                    // Future has been marked ready by callback. Safe to return now.
                    self.get_completion().map(futures::Async::Ready)
                }
            }
        };

        if install_callback {
            // Install the callback. If callback cannot be sent, report immediate `Err`.
            unsafe {
                let data =
                    (self.state.as_ref() as *const FutureTarget) as *mut ::std::os::raw::c_void;
                self.set_callback(Some(notify_task), data).map(|_| ())
            }
        } else {
            Ok(())
        }.and_then(move |_| ret)
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

impl Protected<*mut _Future> for PreparedFuture {
    fn inner(&self) -> *mut _Future { self.0 }
    fn build(inner: *mut _Future) -> Self { PreparedFuture(inner) }
}

impl Protected<*mut _Future> for ResultFuture {
    fn inner(&self) -> *mut _Future { self.inner }
    fn build(inner: *mut _Future) -> Self { ResultFuture { inner, state: Arc::new(FutureTarget { inner: Mutex::new(FutureState::Created) }) } }
}

impl Protected<*mut _Future> for SessionFuture {
    fn inner(&self) -> *mut _Future { self.0 }
    fn build(inner: *mut _Future) -> Self { SessionFuture(inner) }
}
