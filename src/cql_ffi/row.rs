#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::column::CassColumn;
use cql_ffi::iterator::row_iterator::RowIterator;

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;
use std::ffi::CString;

use cql_ffi::value::CassValue;

use cql_ffi::types::cass_size_t;
use cql_bindgen::CassRow as _CassRow;
use cql_bindgen::cass_row_get_column;
use cql_bindgen::cass_row_get_column_by_name;
use cql_bindgen::cass_iterator_from_row;

pub struct CassRow(pub *const _CassRow);

impl Debug for CassRow {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        for column in self.iter() {
            try!(write!(f, "'{{{:?}}}\t", CassValue(column.0)));
        }
        Ok(())
    }
}

impl CassRow {
    pub fn get_column(&self, index: cass_size_t) -> CassColumn {unsafe{
        CassColumn(cass_row_get_column(self.0,index))
    }}

    pub fn get_column_by_name(&self, name: &str) -> CassColumn {unsafe{
        let name = CString::new(name).unwrap();
        CassColumn(cass_row_get_column_by_name(self.0,name.as_ptr()))
    }}

    pub fn iter(&self) -> RowIterator {unsafe{
        RowIterator(cass_iterator_from_row(self.0))
    }}

}
