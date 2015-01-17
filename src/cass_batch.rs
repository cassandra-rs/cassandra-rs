#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_uint;

use cass_consistency::CassConsistency;
use cass_statement::CassStatement;
use cass_error::CassError;

enum Struct_CassBatch_ { }
pub type CassBatch = Struct_CassBatch_;

type Enum_CassBatchType_ = c_uint;
pub const CASS_BATCH_TYPE_LOGGED: c_uint = 0;
pub const CASS_BATCH_TYPE_UNLOGGED: c_uint = 1;
pub const CASS_BATCH_TYPE_COUNTER: c_uint = 2;
pub type CassBatchType = Enum_CassBatchType_;

#[link(name = "cassandra")]
extern "C" {
    pub fn cass_batch_new(_type: CassBatchType) -> *mut CassBatch;
    pub fn cass_batch_free(batch: *mut CassBatch);
    pub fn cass_batch_set_consistency(batch: *mut CassBatch, consistency: CassConsistency) -> CassError;
    pub fn cass_batch_add_statement(batch: *mut CassBatch, statement: *mut CassStatement) -> CassError;
}
