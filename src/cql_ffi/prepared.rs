#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::statement::CassStatement;

pub enum CassPrepared { }

extern "C" {
    pub fn cass_prepared_free(prepared: *const CassPrepared);
    pub fn cass_prepared_bind(prepared: *const CassPrepared) -> *mut CassStatement;
}
