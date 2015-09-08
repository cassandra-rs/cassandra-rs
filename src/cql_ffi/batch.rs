use cql_ffi::statement::CassStatement;

use cql_bindgen::CassError;
use cql_bindgen::CassConsistency;
use cql_bindgen::cass_batch_set_consistency;
use cql_bindgen::cass_batch_add_statement;
use cql_bindgen::cass_batch_free;
use cql_bindgen::cass_batch_new;
use cql_bindgen::CASS_BATCH_TYPE_LOGGED;
use cql_bindgen::CASS_BATCH_TYPE_UNLOGGED;
use cql_bindgen::CASS_BATCH_TYPE_COUNTER;
pub use cql_bindgen::CassBatch as _CassBatch;

pub struct CassBatch(pub *mut _CassBatch);

pub enum CassBatchType {
    LOGGED = CASS_BATCH_TYPE_LOGGED as isize,
    UNLOGGED = CASS_BATCH_TYPE_UNLOGGED as isize,
    COUNTER = CASS_BATCH_TYPE_COUNTER as isize,
}

impl Drop for CassBatch {
    fn drop(&mut self) {
        unsafe {
            cass_batch_free(self.0)
        }
    }
}

impl CassBatch {
    pub fn new(_type: CassBatchType) -> CassBatch {
        unsafe {
            CassBatch(cass_batch_new(_type as u32))
        }
    }

    pub fn set_consistency(&mut self, consistency: CassConsistency) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_consistency(self.0, consistency) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    pub fn add_statement(&mut self, statement: CassStatement) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_add_statement(self.0, statement.0) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }
}
