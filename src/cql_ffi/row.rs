#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;

use cql_ffi::value::CassValue;
use cql_ffi::iterator::CassIterator;

use cql_ffi::types::cass_size_t;
use cql_bindgen::CassRow as _CassRow;
use cql_bindgen::cass_row_get_column;
use cql_bindgen::cass_row_get_column_by_name;
use cql_bindgen::cass_iterator_from_row;

pub struct CassRow(pub *const _CassRow);

impl CassRow {
    pub unsafe fn get_column(&self, index: cass_size_t) -> CassValue {CassValue(cass_row_get_column(self.0,index))}
    pub unsafe fn get_column_by_name(&self, name: &str) -> CassValue {CassValue(cass_row_get_column_by_name(self.0,name.as_ptr() as *const i8))}
    pub unsafe fn from_row(&self) -> CassIterator {CassIterator(cass_iterator_from_row(self.0))}

}
