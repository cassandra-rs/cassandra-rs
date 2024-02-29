use bigdecimal::BigDecimal;

use crate::cassandra::error::*;

use crate::cassandra::inet::Inet;
use crate::cassandra::iterator::{MapIterator, SetIterator, UserTypeIterator};

use crate::cassandra::util::{Protected, ProtectedInner};
use crate::cassandra::uuid::Uuid;
use crate::cassandra::value::Value;
use crate::cassandra_sys::cass_false;
use crate::cassandra_sys::cass_iterator_free;
use crate::cassandra_sys::cass_iterator_from_row;
use crate::cassandra_sys::cass_iterator_get_column;
use crate::cassandra_sys::cass_iterator_next;
use crate::cassandra_sys::cass_row_get_column;
use crate::cassandra_sys::cass_row_get_column_by_name_n;
use crate::cassandra_sys::cass_true;
use crate::cassandra_sys::CassIterator as _CassIterator;
use crate::cassandra_sys::CassRow as _Row;
use crate::LendingIterator;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::os::raw::c_char;

/// A collection of column values. Read-only, so thread-safe.
//
// Borrowed immutably.
pub struct Row<'a>(*const _Row, PhantomData<&'a _Row>);

unsafe impl Sync for Row<'_> {}
unsafe impl Send for Row<'_> {}

impl ProtectedInner<*const _Row> for Row<'_> {
    fn inner(&self) -> *const _Row {
        self.0
    }
}

impl Protected<*const _Row> for Row<'_> {
    fn build(inner: *const _Row) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        Row(inner, PhantomData)
    }
}

impl Debug for Row<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut iter = self.iter();
        while let Some(column) = iter.next() {
            write!(f, "{:?}\t", Value::build(column.inner()))?;
        }
        Ok(())
    }
}

impl Display for Row<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut iter = self.iter();
        while let Some(column) = iter.next() {
            write!(f, "{}\t", Value::build(column.inner()))?;
        }
        Ok(())
    }
}

/// Auto inferencing conversion from Cassandra to Rust.
pub trait AsRustType<T> {
    /// Convert Cassandra column by index.
    fn get(&self, index: usize) -> Result<T>;

    /// Convert Cassandra column by name.
    fn get_by_name<S>(&self, name: S) -> Result<T>
    where
        S: Into<String>;
}

impl AsRustType<bool> for Row<'_> {
    fn get(&self, index: usize) -> Result<bool> {
        let col = self.get_column(index)?;
        col.get_bool()
    }

    fn get_by_name<S>(&self, name: S) -> Result<bool>
    where
        S: Into<String>,
    {
        self.get_column_by_name(name)?.get_bool()
    }
}

impl AsRustType<String> for Row<'_> {
    fn get(&self, index: usize) -> Result<String> {
        let col = self.get_column(index)?;
        col.get_string()
    }

    fn get_by_name<S>(&self, name: S) -> Result<String>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_string()
    }
}

impl AsRustType<f64> for Row<'_> {
    fn get(&self, index: usize) -> Result<f64> {
        let col = self.get_column(index)?;
        col.get_f64()
    }

    fn get_by_name<S>(&self, name: S) -> Result<f64>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_f64()
    }
}

impl AsRustType<f32> for Row<'_> {
    fn get(&self, index: usize) -> Result<f32> {
        let col = self.get_column(index)?;
        col.get_f32()
    }

    fn get_by_name<S>(&self, name: S) -> Result<f32>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_f32()
    }
}

impl AsRustType<i64> for Row<'_> {
    fn get(&self, index: usize) -> Result<i64> {
        let col = self.get_column(index)?;
        col.get_i64()
    }

    fn get_by_name<S>(&self, name: S) -> Result<i64>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_i64()
    }
}

impl AsRustType<i32> for Row<'_> {
    fn get(&self, index: usize) -> Result<i32> {
        let col = self.get_column(index)?;
        col.get_i32()
    }

    fn get_by_name<S>(&self, name: S) -> Result<i32>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_i32()
    }
}

impl AsRustType<i16> for Row<'_> {
    fn get(&self, index: usize) -> Result<i16> {
        let col = self.get_column(index)?;
        col.get_i16()
    }

    fn get_by_name<S>(&self, name: S) -> Result<i16>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_i16()
    }
}

impl AsRustType<i8> for Row<'_> {
    fn get(&self, index: usize) -> Result<i8> {
        let col = self.get_column(index)?;
        col.get_i8()
    }

    fn get_by_name<S>(&self, name: S) -> Result<i8>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_i8()
    }
}

impl AsRustType<u32> for Row<'_> {
    fn get(&self, index: usize) -> Result<u32> {
        let col = self.get_column(index)?;
        col.get_u32()
    }

    fn get_by_name<S>(&self, name: S) -> Result<u32>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_u32()
    }
}

impl AsRustType<Inet> for Row<'_> {
    fn get(&self, index: usize) -> Result<Inet> {
        let col = self.get_column(index)?;
        col.get_inet()
    }

    fn get_by_name<S>(&self, name: S) -> Result<Inet>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_inet()
    }
}

impl<'a> AsRustType<SetIterator<'a>> for Row<'a> {
    // The iterator is newly-created here, but it borrows the data it iterates
    // over from the row (i.e., from its underlying result). Thus its lifetime
    // parameter is the same as the row's.
    fn get(&self, index: usize) -> Result<SetIterator<'a>> {
        let col = self.get_column(index)?;
        col.get_set()
    }

    fn get_by_name<S>(&self, name: S) -> Result<SetIterator<'a>>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_set()
    }
}

impl<'a> AsRustType<MapIterator<'a>> for Row<'a> {
    // The iterator is newly-created here, but it borrows the data it iterates
    // over from the row (i.e., from its underlying result). Thus its lifetime
    // parameter is the same as the row's.
    fn get(&self, index: usize) -> Result<MapIterator<'a>> {
        let col = self.get_column(index)?;
        col.get_map()
    }

    fn get_by_name<S>(&self, name: S) -> Result<MapIterator<'a>>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_map()
    }
}

impl<'a> AsRustType<UserTypeIterator<'a>> for Row<'a> {
    // The iterator is newly-created here, but it borrows the data it iterates
    // over from the row (i.e., from its underlying result). Thus its lifetime
    // parameter is the same as the row's.
    fn get(&self, index: usize) -> Result<UserTypeIterator<'a>> {
        let col = self.get_column(index)?;
        col.get_user_type()
    }

    fn get_by_name<S>(&self, name: S) -> Result<UserTypeIterator<'a>>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_user_type()
    }
}

impl AsRustType<Uuid> for Row<'_> {
    fn get(&self, index: usize) -> Result<Uuid> {
        let col = self.get_column(index)?;
        col.get_uuid()
    }

    fn get_by_name<S>(&self, name: S) -> Result<Uuid>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_uuid()
    }
}

impl AsRustType<uuid::Uuid> for Row<'_> {
    fn get(&self, index: usize) -> Result<uuid::Uuid> {
        let col = self.get_column(index)?;
        col.get_uuid().map(|x| x.into())
    }

    fn get_by_name<S>(&self, name: S) -> Result<uuid::Uuid>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_uuid().map(|x| x.into())
    }
}

impl AsRustType<Vec<u8>> for Row<'_> {
    fn get(&self, index: usize) -> Result<Vec<u8>> {
        let col = self.get_column(index)?;
        col.get_bytes().map(|b| b.to_vec())
    }

    fn get_by_name<S>(&self, name: S) -> Result<Vec<u8>>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_bytes().map(|b| b.to_vec())
    }
}

impl AsRustType<BigDecimal> for Row<'_> {
    fn get(&self, index: usize) -> Result<BigDecimal> {
        let col = self.get_column(index)?;
        col.get_decimal().map(|x| x.into())
    }

    fn get_by_name<S>(&self, name: S) -> Result<BigDecimal>
    where
        S: Into<String>,
    {
        let col = self.get_column_by_name(name)?;
        col.get_decimal().map(|x| x.into())
    }
}

impl<'a> Row<'a> {
    /// Get a particular column by index
    pub fn get_column(&self, index: usize) -> Result<Value<'a>> {
        unsafe {
            let col = cass_row_get_column(self.0, index);
            if col.is_null() {
                Err(CassErrorCode::LIB_INDEX_OUT_OF_BOUNDS.to_error())
            } else {
                Ok(Value::build(col))
            }
        }
    }

    /// Get a particular column by name
    pub fn get_column_by_name<S>(&self, name: S) -> Result<Value<'a>>
    where
        S: Into<String>,
    {
        unsafe {
            let name_str = name.into();
            let name_ptr = name_str.as_ptr() as *const c_char;
            let col = cass_row_get_column_by_name_n(self.0, name_ptr, name_str.len());
            if col.is_null() {
                Err(CassErrorCode::LIB_INDEX_OUT_OF_BOUNDS.to_error())
            } else {
                Ok(Value::build(col))
            }
        }
    }

    /// Creates a new iterator for the specified row. This can be
    /// used to iterate over columns in a row.
    pub fn iter(&'a self) -> RowIterator<'a> {
        unsafe { RowIterator(cass_iterator_from_row(self.0), PhantomData) }
    }
}

/// An iterator over the columns in a row
///
/// A Cassandra iterator is a `LendingIterator` because it borrows from some
/// underlying value, but owns a single item. Each time `next()` is invoked it
/// decodes the current item into that item, thus invalidating its previous
/// value.
#[derive(Debug)]
pub struct RowIterator<'a>(*mut _CassIterator, PhantomData<&'a _Row>);

// The underlying C type has no thread-local state, and forbids only concurrent
// mutation/free: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for RowIterator<'_> {}
unsafe impl Sync for RowIterator<'_> {}

impl Drop for RowIterator<'_> {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl LendingIterator for RowIterator<'_> {
    type Item<'a> = Value<'a> where Self: 'a;

    fn next(&mut self) -> Option<<Self as LendingIterator>::Item<'_>> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(Value::build(cass_iterator_get_column(self.0))),
            }
        }
    }
}
