use cassandra_sys::CassIterator as _CassIterator;
use cassandra_sys::cass_iterator_free;
use cassandra_sys::cass_iterator_next;
use cassandra_sys::cass_iterator_get_column;
use cassandra_sys::CassRow as _Row;
use cassandra_sys::cass_row_get_column;
use cassandra_sys::cass_row_get_column_by_name;
use cassandra_sys::cass_iterator_from_row;
use cassandra_sys::CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS;
use cassandra::util::Protected;
use cassandra::value::Value;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::ffi::CString;
use std::iter::IntoIterator;
use std::iter;
use cassandra_sys::cass_true;
use cassandra_sys::cass_false;

use cassandra::error::CassError;
use cassandra::column::Column;

///A collection of column values.
pub struct Row(*const _Row);

impl Protected<*const _Row> for Row {
    fn inner(&self) -> *const _Row {
        self.0
    }
    fn build(inner: *const _Row) -> Self {
        Row(inner)
    }
}

impl Debug for Row {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for column in self {
            try!(write!(f, "{:?}\t", Value::build(column.inner())));
        }
        Ok(())
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for column in self {
            try!(write!(f, "{}\t", Value::build(column.inner())));
        }
        Ok(())
    }
}

///Auto inferencing conversion from c* to rust
pub trait AsRustType<T> {
    ///convert while reading cassandra columns
    fn get_col(&self, index: u64) -> Result<T, CassError>;
}

impl AsRustType<bool> for Row {
    fn get_col(&self, index: u64) -> Result<bool, CassError> {
        let col = try!(self.get_column(index));
        col.get_bool()
    }
}

impl AsRustType<f64> for Row {
    fn get_col(&self, index: u64) -> Result<f64, CassError> {
        let col = try!(self.get_column(index));
        col.get_double()
    }
}

impl AsRustType<f32> for Row {
    fn get_col(&self, index: u64) -> Result<f32, CassError> {
        let col = try!(self.get_column(index));
        col.get_float()
    }
}

impl AsRustType<i64> for Row {
    fn get_col(&self, index: u64) -> Result<i64, CassError> {
        let col = try!(self.get_column(index));
        col.get_i64()
    }
}

impl AsRustType<i32> for Row {
    fn get_col(&self, index: u64) -> Result<i32, CassError> {
        let col = try!(self.get_column(index));
        col.get_i32()
    }
}

impl Row {
    ///Get a particular column by index
    pub fn get_column(&self, index: u64) -> Result<Column, CassError> {
        unsafe {
            let col = cass_row_get_column(self.0, index);
            if col.is_null() {
                Err(CassError::build(CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS))
            } else {
                Ok(Column::build(col))
            }
        }
    }

    ///Get a particular column by name
    pub fn get_column_by_name<S>(&self, name: S) -> Column
        where S: Into<String> {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            println!("name: {:?}", name);
            println!("self: {:?}", self);
            // unimplemented!();
            Column::build(cass_row_get_column_by_name(self.0, name.as_ptr()))
        }
    }
}

///An iterator over the columns in a row
pub struct RowIterator(pub *mut _CassIterator);


impl Drop for RowIterator {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl iter::Iterator for RowIterator {
    type Item = Column;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(Column::build(cass_iterator_get_column(self.0))),
            }
        }
    }
}

impl<'a> Iterator for &'a RowIterator {
    type Item = Column;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(Column::build(cass_iterator_get_column(self.0))),
            }
        }
    }
}

impl Display for RowIterator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for item in self {
            try!(write!(f, "{}\t", Value::build(item.inner())));
        }
        Ok(())
    }
}

impl IntoIterator for Row {
    type Item = Column;
    type IntoIter = RowIterator;

    ///Creates a new iterator for the specified row. This can be
    ///used to iterate over columns in a row.
    fn into_iter(self) -> Self::IntoIter {
        unsafe { RowIterator(cass_iterator_from_row(self.0)) }
    }
}

impl<'a> IntoIterator for &'a Row {
    type Item = Column;
    type IntoIter = RowIterator;
    fn into_iter(self) -> Self::IntoIter {
        unsafe { RowIterator(cass_iterator_from_row(self.0)) }
    }
}
