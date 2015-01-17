#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cass_statement::CassStatement;

enum Struct_CassPrepared_ { }
pub type CassPrepared = Struct_CassPrepared_;

extern "C" {
    pub fn cass_prepared_free(prepared: *const CassPrepared);
    pub fn cass_prepared_bind(prepared: *const CassPrepared) -> *mut CassStatement;
}
