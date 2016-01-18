use std::ffi::NulError;
use std::ffi::CString;

use cassandra::statement::Statement;
use cassandra::policy;

use cassandra_sys::CassError;
use cassandra_sys::CassConsistency;
use cassandra_sys::cass_batch_set_consistency;
use cassandra_sys::cass_batch_add_statement;
use cassandra_sys::cass_batch_set_custom_payload;
use cassandra_sys::cass_batch_set_retry_policy;
use cassandra_sys::cass_batch_set_serial_consistency;
use cassandra_sys::cass_batch_set_timestamp;
use cassandra_sys::cass_batch_free;
use cassandra_sys::cass_batch_new;
use cassandra_sys::CASS_BATCH_TYPE_LOGGED;
use cassandra_sys::CASS_BATCH_TYPE_UNLOGGED;
use cassandra_sys::CASS_BATCH_TYPE_COUNTER;
use cassandra_sys::CassCustomPayload as _CassCustomPayload;
use cassandra::policy::retry::RetryPolicy;
use cassandra::consistency::Consistency;
use cassandra_sys::cass_custom_payload_free;
use cassandra_sys::cass_custom_payload_new;
use cassandra_sys::cass_custom_payload_set;
use cassandra::consistency;
pub use cassandra_sys::CassBatch as _Batch;
use cassandra::statement;

///A group of statements that are executed as a single batch.
///<b>Note:</b> Batches are not supported by the binary protocol version 1.
pub struct Batch(*mut _Batch);

pub mod protected {
    use cassandra::batch::Batch;
    use cassandra_sys::CassBatch as _Batch;
    use cassandra::batch::CustomPayload;
    use cassandra_sys::CassCustomPayload as _CassCustomPayload;
    pub fn inner(batch: &Batch) -> *mut _Batch {
        batch.0
    }

    pub fn inner_payload(payload: &CustomPayload) -> *mut _CassCustomPayload {
        payload.0
    }
}

///Custom payloads not fully supported yet
pub struct CustomPayload(*mut _CassCustomPayload);

impl CustomPayload {
    ///creates a new custom payload
    pub fn new() -> Self {
        unsafe { CustomPayload(cass_custom_payload_new()) }
    }

    ///Sets an item to the custom payload.
    pub fn set(&self, name: String, value: &[u8]) -> Result<(), NulError> {
        unsafe {
            Ok(cass_custom_payload_set(self.0,
                                       try!(CString::new(name)).as_ptr(),
                                       value.as_ptr(),
                                       value.len() as u64))
        }
    }
}

impl Drop for CustomPayload {
    fn drop(&mut self) {
        unsafe { cass_custom_payload_free(self.0) }
    }
}

///Type of Cassandra Batch operation to perform
pub enum BatchType {
    ///Logged batches have Atomicity guarantees
    LOGGED = CASS_BATCH_TYPE_LOGGED as isize,
    ///Unlogged batches do not provide any atomicity guarantees
    UNLOGGED = CASS_BATCH_TYPE_UNLOGGED as isize,
    ///Counter batches can only be used when writing counter types
    COUNTER = CASS_BATCH_TYPE_COUNTER as isize,
}

impl Drop for Batch {
    ///Frees a batch instance. Batches can be immediately freed after being
    ///executed.
    fn drop(&mut self) {
        unsafe { cass_batch_free(self.0) }
    }
}

impl Batch {
    ///Creates a new batch statement with batch type.
    pub fn new(batch_type: BatchType) -> Batch {
        unsafe { Batch(cass_batch_new(batch_type as u32)) }
    }

    ///Sets the batch's consistency level
    pub fn set_consistency(&mut self, consistency: CassConsistency) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_consistency(self.0, consistency) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    ///Sets the batch's serial consistency level.
    ///
    ///<b>Default:</b> Not set
    pub fn set_serial_consistency(&mut self, consistency: Consistency) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_serial_consistency(self.0, consistency::protected::inner(consistency)) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    /// Sets the batch's timestamp.
    pub fn set_timestamp(&mut self, timestamp: i64) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_timestamp(self.0, timestamp) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    ///Sets the batch's retry policy.
    pub fn set_retry_policy(&mut self, retry_policy: RetryPolicy) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_retry_policy(self.0, policy::retry::protected::inner(retry_policy)) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    ///Sets the batch's custom payload.
    pub fn set_custom_payload(&mut self, custom_payload: CustomPayload) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_custom_payload(self.0, custom_payload.0) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    ///Adds a statement to a batch.
    pub fn add_statement(&mut self, statement: &Statement) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_add_statement(self.0, statement::protected::inner(statement)) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }
}
