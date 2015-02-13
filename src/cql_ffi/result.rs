#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::value::CassValueType;
use cql_ffi::row::CassRow;
use cql_ffi::string::CassString;
use cql_ffi::types::cass_size_t;
use cql_ffi::types::cass_bool_t;

pub enum CassResult { }

extern "C" {
    pub fn cass_result_free(result: *const CassResult);
    pub fn cass_result_row_count(result: *const CassResult) -> cass_size_t;
    pub fn cass_result_column_count(result: *const CassResult) -> cass_size_t;
    pub fn cass_result_column_name(result: *const CassResult, index: cass_size_t) -> CassString;
    pub fn cass_result_column_type(result: *const CassResult, index: cass_size_t) -> CassValueType;
    pub fn cass_result_first_row(result: *const CassResult) -> *const CassRow;
    pub fn cass_result_has_more_pages(result: *const CassResult) -> cass_bool_t;
}
