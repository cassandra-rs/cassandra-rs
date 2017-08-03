use cassandra::column::Column;
use cassandra::error::*;

use cassandra::iterator::{MapIterator, SetIterator};
use cassandra::util::Protected;
use cassandra::value::Value;
use cassandra_sys::CassIterator as _CassIterator;
use cassandra_sys::CassRow as _Row;
use cassandra_sys::cass_false;
use cassandra_sys::cass_iterator_free;
use cassandra_sys::cass_iterator_from_row;
use cassandra_sys::cass_iterator_get_column;
use cassandra_sys::cass_iterator_next;
use cassandra_sys::cass_row_get_column;
use cassandra_sys::cass_row_get_column_by_name;
use cassandra_sys::cass_true;
use std::ffi::CString;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::iter;
use std::iter::IntoIterator;

/// A collection of column values.
pub struct Row(*const _Row);

impl Protected<*const _Row> for Row {
    fn inner(&self) -> *const _Row { self.0 }
    fn build(inner: *const _Row) -> Self { Row(inner) }
}

impl Debug for Row {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for column in self {
            write!(f, "{:?}\t", Value::build(column.inner()))?;
        }
        Ok(())
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for column in self {
            write!(f, "{}\t", Value::build(column.inner()))?;
        }
        Ok(())
    }
}

/// Auto inferencing conversion from c* to rust
pub trait AsRustType<T> {
    /// convert while reading cassandra columns
    fn get_col(&self, index: usize) -> Result<T>;

    /// convert while reading cassandra columns by name
    fn get_col_by_name<S>(&self, name: S) -> Result<T>
        where S: Into<String>;
}

impl AsRustType<bool> for Row {
    fn get_col(&self, index: usize) -> Result<bool> {
        let col = self.get_column(index)?;
        col.get_bool()
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<bool>
        where S: Into<String> {
        self.get_column_by_name(name)?.get_bool()
    }
}

impl AsRustType<String> for Row {
    fn get_col(&self, index: usize) -> Result<String> {
        let col = self.get_column(index)?;
        col.get_string()
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<String>
        where S: Into<String> {
        let col = self.get_column_by_name(name)?;
        col.get_string()
    }
}

impl AsRustType<f64> for Row {
    fn get_col(&self, index: usize) -> Result<f64> {
        let col = self.get_column(index)?;
        col.get_double()
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<f64>
        where S: Into<String> {
        let col = self.get_column_by_name(name)?;
        col.get_double()
    }
}

impl AsRustType<f32> for Row {
    fn get_col(&self, index: usize) -> Result<f32> {
        let col = self.get_column(index)?;
        col.get_float()
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<f32>
        where S: Into<String> {
        let col = self.get_column_by_name(name)?;
        col.get_float()
    }
}

impl AsRustType<i64> for Row {
    fn get_col(&self, index: usize) -> Result<i64> {
        let col = self.get_column(index)?;
        col.get_i64()
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<i64>
        where S: Into<String> {
        let col = self.get_column_by_name(name)?;
        col.get_i64()
    }
}

impl AsRustType<i32> for Row {
    fn get_col(&self, index: usize) -> Result<i32> {
        let col = self.get_column(index)?;
        col.get_i32()
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<i32>
        where S: Into<String> {
        let col = self.get_column_by_name(name)?;
        col.get_i32()
    }
}

impl AsRustType<SetIterator> for Row {
    fn get_col(&self, index: usize) -> Result<SetIterator> {
        let col = self.get_column(index)?;
        col.set_iter()
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<SetIterator>
        where S: Into<String> {
        let col = self.get_column_by_name(name)?;
        col.set_iter()
    }
}

impl AsRustType<MapIterator> for Row {
    fn get_col(&self, index: usize) -> Result<MapIterator> {
        let col = self.get_column(index)?;
        col.map_iter()
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<MapIterator>
        where S: Into<String> {
        let col = self.get_column_by_name(name)?;
        col.map_iter()
    }
}

impl AsRustType<Vec<u8>> for Row {
    fn get_col(&self, index: usize) -> Result<Vec<u8>> {
        let col = self.get_column(index)?;
        col.get_blob().map(|b| b.to_vec())
    }

    fn get_col_by_name<S>(&self, name: S) -> Result<Vec<u8>>
        where S: Into<String> {
        let col = self.get_column_by_name(name)?;
        col.get_blob().map(|b| b.to_vec())
    }
}

impl Row {
    /// Get a particular column by index
    pub fn get_column(&self, index: usize) -> Result<Column> {
        unsafe {
            let col = cass_row_get_column(self.0, index);
            if col.is_null() {
                Err(CassErrorCode::LIB_INDEX_OUT_OF_BOUNDS.to_error())
            } else {
                Ok(Column::build(col))
            }
        }
    }

    /// Get a particular column by name
    pub fn get_column_by_name<S>(&self, name: S) -> Result<Column>
        where S: Into<String> {
        unsafe {
            let col = cass_row_get_column_by_name(self.0,
                                                  CString::new(name.into())?.as_ptr());
            if col.is_null() {
                Err(CassErrorCode::LIB_INDEX_OUT_OF_BOUNDS.to_error())
            } else {
                Ok(Column::build(col))
            }
        }
    }
}

/// An iterator over the columns in a row
#[derive(Debug)]
pub struct RowIterator(pub *mut _CassIterator);


impl Drop for RowIterator {
    fn drop(&mut self) { unsafe { cass_iterator_free(self.0) } }
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
            write!(f, "{}\t", Value::build(item.inner()))?;
        }
        Ok(())
    }
}

impl IntoIterator for Row {
    type Item = Column;
    type IntoIter = RowIterator;

    /// Creates a new iterator for the specified row. This can be
    /// used to iterate over columns in a row.
    fn into_iter(self) -> Self::IntoIter { unsafe { RowIterator(cass_iterator_from_row(self.0)) } }
}

impl<'a> IntoIterator for &'a Row {
    type Item = Column;
    type IntoIter = RowIterator;
    fn into_iter(self) -> Self::IntoIter { unsafe { RowIterator(cass_iterator_from_row(self.0)) } }
}
