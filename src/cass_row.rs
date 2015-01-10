#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::types::os::arch::c95::c_char;

use cass_value::CassValue;

use cass_types::cass_size_t;

enum Struct_CassRow_ { }
pub type CassRow = Struct_CassRow_;

extern "C" {
    pub fn cass_row_get_column(row: *const CassRow, index: cass_size_t) -> *const CassValue;
    pub fn cass_row_get_column_by_name(row: *const CassRow, name: *const c_char) -> *const CassValue;
}
