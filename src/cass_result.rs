#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cass_value::CassValueType;
use cass_row::CassRow;
use cass_string::CassString;

use cass_types::cass_size_t;
use cass_types::cass_bool_t;

enum Struct_CassResult_ { }
pub type CassResult = Struct_CassResult_;

extern "C" {
    pub fn cass_result_free(result: *const CassResult);
    pub fn cass_result_row_count(result: *const CassResult) -> cass_size_t;
    pub fn cass_result_column_count(result: *const CassResult) -> cass_size_t;
    pub fn cass_result_column_name(result: *const CassResult, index: cass_size_t) -> CassString;
    pub fn cass_result_column_type(result: *const CassResult, index: cass_size_t) -> CassValueType;
    pub fn cass_result_first_row(result: *const CassResult) -> *const CassRow;
    pub fn cass_result_has_more_pages(result: *const CassResult) -> cass_bool_t;
}
