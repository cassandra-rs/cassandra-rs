use cassandra::error::*;
use cassandra::prepared::PreparedStatement;
use cassandra::result::CassResult;
use cassandra::util::Protected;
use cassandra::consistency::Consistency;
use cassandra::write_type::WriteType;
use cassandra_sys::CassError_;
use cassandra_sys::CASS_OK;
use cassandra_sys::CassFuture as _Future;
use cassandra_sys::cass_future_error_code;
use cassandra_sys::cass_future_error_message;
use cassandra_sys::cass_future_free;
use cassandra_sys::cass_future_get_error_result;
use cassandra_sys::cass_future_get_prepared;
use cassandra_sys::cass_future_get_result;
use cassandra_sys::cass_future_ready;
use cassandra_sys::cass_future_set_callback;
use cassandra_sys::{cass_true, cass_false};

use std::mem;
use std::slice;
use std::str;
use std::sync::{Arc, Mutex};
use std::marker::PhantomData;
use futures;

/// A future representing the result of a Cassandra driver operation.
///
/// On success, returns a result of type `T`. On failure, returns a Cassandra error.
///
/// When constructing this take care to supply the correct type argument, since it will
/// be used to control how the result is extracted from the underlying Cassandra
/// driver future (see `Completable`).
#[must_use]
#[derive(Debug)]
pub struct CassFuture<T> {
    /// The underlying Cassandra driver future object.
    inner: *mut _Future,

    /// The current state of this future.
    state: Arc<FutureTarget>,

    /// Treat as if it contains a T.
    phantom: PhantomData<T>,
}

// The underlying C type has no thread-local state, and explicitly supports access
// from multiple threads: https://datastax.github.io/cpp-driver/topics/#thread-safety
//
// But it can be used to move a value of type `T`, so `T` needs to be `Send` if the
// future is.
//
// The same is not true for `Sync`, because a future doesn't give multiple threads
// concurrent access to the value (you can only poll a future once).
unsafe impl<T> Sync for CassFuture<T> {}
unsafe impl<T> Send for CassFuture<T> where T: Send {}

impl<T> CassFuture<T> {
    /// Wrap a Cassandra driver future to make it a proper Rust future.
    ///
    /// When invoking this take care to supply the correct type argument, since it will
    /// be used to control how the result is extracted from the underlying Cassandra
    /// driver future (see `Completable`).
    pub(crate) fn build(inner: *mut _Future) -> Self {
        CassFuture {
            inner,
            state: Arc::new(FutureTarget { inner: Mutex::new(FutureState::Created) }),
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for CassFuture<T> {
    /// Drop this CassFuture.
    ///
    /// This also drops its reference to the FutureTarget, but if
    /// we're waiting to be called back the FutureState::Awaiting holds another reference to
    /// the target, which keeps it alive until the callback fires.
    fn drop(&mut self) { unsafe { cass_future_free(self.inner) }; }
}

/// A type is Completable if it can be returned from a Cassandra driver future.
/// You should only use this if you reasonably expect that a particular future will
/// have such a result; for `CassFuture`s we ensure this by construction.
pub trait Completable where Self: Sized {
    /// Extract the result from the future, if present.
    unsafe fn get(inner: *mut _Future) -> Option<Self>;
}

/// Futures that complete with no value, or report an error.
impl Completable for () {
    unsafe fn get(_inner: *mut _Future) -> Option<Self> {
        Some(())
    }
}

/// The mainline case - a CassResult.
impl Completable for CassResult {
    unsafe fn get(inner: *mut _Future) -> Option<Self> {
        cass_future_get_result(inner).as_ref().map(|r| CassResult::build(r as *const _))
    }
}

/// Futures that complete with a prepared statement.
impl Completable for PreparedStatement {
    unsafe fn get(inner: *mut _Future) -> Option<Self> {
        cass_future_get_prepared(inner).as_ref().map(|r| PreparedStatement::build(r as *const _))
    }
}

/// A Cassandra future is a normal Rust future.
impl<T: Completable> futures::Future for CassFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> { unsafe {
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
                    if cass_future_ready(self.inner) == cass_true {
                        // Future is ready; wrap success in `Ok(Ready)` or report failure as `Err`.
                        get_completion(self.inner).map(futures::Async::Ready)
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
                    get_completion(self.inner).map(futures::Async::Ready)
                }
            }
        };

        if install_callback {
            // Install the callback. If callback cannot be sent, report immediate `Err`.
            let data =
                (self.state.as_ref() as *const FutureTarget) as *mut ::std::os::raw::c_void;
            cass_future_set_callback(self.inner, Some(notify_task), data)
                .to_result(())
        } else {
            Ok(())
        }.and_then(move |_| ret)
    }}
}

/// Extract success or failure from a completed future.
unsafe fn get_completion<T: Completable>(inner: *mut _Future) -> Result<T> {
    // Future is ready; wrap success in `Ok(Ready)` or report failure as `Err`.
    let rc = cass_future_error_code(inner);
    match rc {
        CASS_OK => {
            match Completable::get(inner) {
                None => Err(CassErrorCode::LIB_NULL_VALUE.to_error()),
                Some(v) => Ok(v)
            }
        },
        _ => Err(get_cass_future_error(rc, inner)),
    }
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
/// The FutureTarget is held by a CassFuture, and (in the FutureState::Awaiting state) by
/// a C++ Cassandra driver callback. The latter pointer is represented by one inside that state,
/// so that it is not freed early.
#[derive(Debug)]
struct FutureTarget {
    inner: Mutex<FutureState>,
}

/// The state of a Cassandra future.
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

/// Callback which wakes the task waiting on this future.
/// Called by the C++ driver when the future is ready,
/// with a pointer to the `CassFuture`.
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
