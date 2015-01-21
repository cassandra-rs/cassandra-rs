#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;

use cass_value::CassValue;

use cass_types::cass_size_t;

pub enum CassRow { }

extern "C" {
    pub fn cass_row_get_column(row: *const CassRow, index: cass_size_t) -> *const CassValue;
    pub fn cass_row_get_column_by_name(row: *const CassRow, name: *const c_char) -> *const CassValue;
}
