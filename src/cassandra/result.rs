#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cassandra::data_type::ConstDataType;
use cassandra::row::Row;
use cassandra::util::Protected;
use cassandra::value::ValueType;
use cassandra::error::*;

use cassandra_sys::CassIterator as _CassIterator;
use cassandra_sys::CassResult as _CassResult;
use cassandra_sys::cass_false;
use cassandra_sys::cass_iterator_free;
use cassandra_sys::cass_iterator_from_result;
use cassandra_sys::cass_iterator_get_row;
use cassandra_sys::cass_iterator_next;
use cassandra_sys::cass_result_column_count;
use cassandra_sys::cass_result_column_data_type;
use cassandra_sys::cass_result_column_name;
use cassandra_sys::cass_result_column_type;
use cassandra_sys::cass_result_first_row;
use cassandra_sys::cass_result_free;
use cassandra_sys::cass_result_has_more_pages;
use cassandra_sys::cass_result_paging_state_token;
use cassandra_sys::cass_result_row_count;
use cassandra_sys::cass_true;

use std::ffi::CString;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem;
use std::slice;
use std::str;

/// The result of a query.
/// A result object is read-only and is thread-safe to read or iterate over
/// concurrently, since we do not bind any setters (e.g., `set_metadata`).
pub struct CassResult(*const _CassResult);
unsafe impl Sync for CassResult {}
unsafe impl Send for CassResult {}

impl Protected<*const _CassResult> for CassResult {
    fn inner(&self) -> *const _CassResult { self.0 }
    fn build(inner: *const _CassResult) -> Self { CassResult(inner) }
}

impl Debug for CassResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Result row count: {:?}\n", self.row_count())?;
        for row in self.iter() {
            write!(f, "{:?}\n", row)?;
        }
        Ok(())
    }
}

impl Display for CassResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Result row count: {}\n", self.row_count())?;
        for row in self.iter() {
            write!(f, "{}\n", row)?;
        }
        Ok(())
    }
}

// FIXME Understand why this drop results in double freeing when binding by name
// impl Drop for CassResult {
//    fn drop(&mut self) {
//        unsafe { cass_result_free(self.0) }
//    }
// }

impl CassResult {
    /// Gets the number of rows for the specified result.
    pub fn row_count(&self) -> u64 { unsafe { cass_result_row_count(self.0) as u64 } }

    /// Gets the number of columns per row for the specified result.
    pub fn column_count(&self) -> u64 { unsafe { cass_result_column_count(self.0) as u64 } }

    /// Gets the column name at index for the specified result.
    pub fn column_name(&self, index: usize) -> Result<&str> {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_result_column_name(self.0, index, &mut name, &mut name_length).to_result(())
                .and_then(|_| {
                    let slice = slice::from_raw_parts(name as *const u8, name_length as usize);
                    Ok(str::from_utf8(slice)?)
                }
            )
        }
    }

    /// Gets the column type at index for the specified result.
    pub fn column_type(&self, index: usize) -> ValueType {
        unsafe { ValueType::build(cass_result_column_type(self.0, index)) }
    }

    /// Gets the column datatype at index for the specified result.
    pub fn column_data_type(&self, index: usize) -> ConstDataType {
        unsafe { ConstDataType(cass_result_column_data_type(self.0, index)) }
    }

    /// Gets the first row of the result.
    pub fn first_row(&self) -> Option<Row> {
        unsafe {
            match self.row_count() {
                0 => None,
                _ => Some(Row::build(cass_result_first_row(self.0))),
            }
        }
    }

    /// Returns true if there are more pages.
    pub fn has_more_pages(&self) -> bool { unsafe { cass_result_has_more_pages(self.0) == cass_true } }

    /// Sets the statement's paging state. This can be used to get the next page of
    /// data in a multi-page query.
    ///
    /// <b>Warning:</b> The paging state should not be exposed to or come from
    /// untrusted environments. The paging state could be spoofed and potentially
    // used to gain access to other data.
    pub fn set_paging_state_token(&mut self, paging_state: &str) -> Result<&mut Self> {
        unsafe {
            let state = CString::new(paging_state)?;
            cass_result_paging_state_token(self.0, &mut state.as_ptr(), &mut (state.to_bytes().len()))
                .to_result(self)
        }
    }


    /// Creates a new iterator for the specified result. This can be
    /// used to iterate over rows in the result.
    pub fn iter(&self) -> ResultIterator { unsafe { ResultIterator(cass_iterator_from_result(self.0)) } }
}

/// An iterator over the results of a query
#[derive(Debug)]
pub struct ResultIterator(pub *mut _CassIterator);

// The underlying C type has no thread-local state, but does not support access
// from multiple threads: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for ResultIterator {}

impl Drop for ResultIterator {
    fn drop(&mut self) { unsafe { cass_iterator_free(self.0) } }
}

impl Iterator for ResultIterator {
    type Item = Row;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                cass_false => None,
                cass_true => Some(self.get_row()),
            }
        }
    }
}

impl ResultIterator {
    /// Gets the next row in the result set
    pub fn get_row(&mut self) -> Row { unsafe { Row::build(cass_iterator_get_row(self.0)) } }
}

impl IntoIterator for CassResult {
    type Item = Row;
    type IntoIter = ResultIterator;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

// impl<'a> IntoIterator for &'a CassandraResult {
//    type Item = Row;
//    type IntoIter = ResultIterator;
//
//    fn into_iter(self) -> Self::IntoIter {unsafe{
//        ResultIterator(cass_iterator_from_result(self.0))
//    }}
// }
