#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::statement::CassStatement;
use cql_bindgen::CassPrepared as _CassPrepared;
use cql_bindgen::cass_prepared_free;
use cql_bindgen::cass_prepared_bind;

pub struct CassPrepared(pub *const _CassPrepared);

impl CassPrepared {
    pub unsafe fn free(&mut self) {cass_prepared_free(self.0)}
    pub unsafe fn bind(&self) -> CassStatement {CassStatement(cass_prepared_bind(self.0))}
}
