#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::value::CassValueType;
use cql_ffi::row::CassRow;
use cql_ffi::string::CassString;
use cql_ffi::types::cass_size_t;
use cql_ffi::iterator::CassIterator;
use cql_bindgen::CassResult as _CassResult;

use cql_bindgen::cass_result_free;
use cql_bindgen::cass_result_row_count;
use cql_bindgen::cass_result_column_count;
use cql_bindgen::cass_result_column_name;
use cql_bindgen::cass_result_column_type;
use cql_bindgen::cass_result_first_row;
use cql_bindgen::cass_result_has_more_pages;
use cql_bindgen::cass_iterator_from_result;

pub struct CassResult(pub *const _CassResult);

impl CassResult {
    pub unsafe fn free(&mut self) {cass_result_free(self.0)}
    pub unsafe fn row_count(result: CassResult) -> u64 {cass_result_row_count(result.0)}
    pub unsafe fn column_count(result: CassResult) -> u64 {cass_result_column_count(result.0)}
    pub unsafe fn column_name(result: CassResult, index: cass_size_t) -> CassString {CassString(cass_result_column_name(result.0, index))}
    pub unsafe fn column_type(result: CassResult, index: cass_size_t) -> CassValueType {CassValueType(cass_result_column_type(result.0, index))}
    pub unsafe fn first_row(result: CassResult) -> CassRow {CassRow(cass_result_first_row(result.0))}
    pub unsafe fn has_more_pages(result: CassResult) -> bool {if cass_result_has_more_pages(result.0) > 0 {true} else {false}}
    pub unsafe fn iter(&self) -> CassIterator {CassIterator(cass_iterator_from_result(self.0))}

}
