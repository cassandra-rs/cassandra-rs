#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

pub use cql_bindgen::CassBatch as _CassBatch;

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

pub struct CassBatch(pub *mut _CassBatch);
pub enum CassBatchType {
    LOGGED = CASS_BATCH_TYPE_LOGGED as isize,
    UNLOGGED = CASS_BATCH_TYPE_UNLOGGED as isize,
    COUNTER = CASS_BATCH_TYPE_COUNTER as isize
}

impl CassBatch {
    pub unsafe fn new(_type: CassBatchType) -> CassBatch {CassBatch(cass_batch_new(_type as u32))}
    pub unsafe fn free(&mut self) {cass_batch_free(self.0)}
    pub unsafe fn set_consistency(&mut self, consistency: CassConsistency) -> CassError {cass_batch_set_consistency(self.0,consistency)}
    pub unsafe fn add_statement(&mut self, statement: CassStatement) -> CassError {cass_batch_add_statement(self.0,statement.0)}
}
