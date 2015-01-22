#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cass_consistency::CassConsistency;
use cass_statement::CassStatement;
use cass_error::CassError;

pub enum CassBatch { }

#[repr(C)]
pub enum CassBatchType {
    LOGGED = 0is,
    UNLOGGED = 1,
    COUNTER = 2
}

#[link(name = "cassandra")]
extern "C" {
    pub fn cass_batch_new(_type: CassBatchType) -> *mut CassBatch;
    pub fn cass_batch_free(batch: *mut CassBatch);
    pub fn cass_batch_set_consistency(batch: *mut CassBatch, consistency: CassConsistency) -> CassError;
    pub fn cass_batch_add_statement(batch: *mut CassBatch, statement: *mut CassStatement) -> CassError;
}
