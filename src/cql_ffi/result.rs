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

impl Drop for CassResult {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl CassResult {
    unsafe fn free(&mut self) {cass_result_free(self.0)}
    pub unsafe fn row_count(&self) -> u64 {cass_result_row_count(self.0)}
    pub unsafe fn column_count(&self) -> u64 {cass_result_column_count(self.0)}
    pub unsafe fn column_name(&self, index: cass_size_t) -> CassString {CassString(cass_result_column_name(self.0, index))}
    pub unsafe fn column_type(&self, index: cass_size_t) -> CassValueType {CassValueType::build(cass_result_column_type(self.0, index))}
    pub unsafe fn first_row(&self) -> CassRow {CassRow(cass_result_first_row(self.0))}
    pub unsafe fn has_more_pages(&self) -> bool {if cass_result_has_more_pages(self.0) > 0 {true} else {false}}
    pub unsafe fn iter(&self) -> CassIterator {CassIterator(cass_iterator_from_result(self.0))}

}
