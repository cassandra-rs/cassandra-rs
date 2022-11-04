use crate::cassandra::data_type::ConstDataType;
use crate::cassandra::error::*;
use crate::cassandra::statement::Statement;
use crate::cassandra::util::Protected;

use crate::cassandra_sys::cass_prepared_bind;
use crate::cassandra_sys::cass_prepared_free;
use crate::cassandra_sys::cass_prepared_parameter_data_type;
use crate::cassandra_sys::cass_prepared_parameter_data_type_by_name_n;
use crate::cassandra_sys::cass_prepared_parameter_name;
use crate::cassandra_sys::CassPrepared as _PreparedStatement;
use std::os::raw::c_char;
use std::{mem, slice, str};

/// A statement that has been prepared against at least one Cassandra node.
/// Instances of this class should not be created directly, but through Session.prepare().
#[derive(Debug)]
pub struct PreparedStatement(*const _PreparedStatement);

unsafe impl Send for PreparedStatement {}
unsafe impl Sync for PreparedStatement {}

impl Drop for PreparedStatement {
    /// Frees a prepared statement
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { cass_prepared_free(self.0) }
        }
    }
}

impl Protected<*const _PreparedStatement> for PreparedStatement {
    fn inner(&self) -> *const _PreparedStatement {
        self.0
    }
    fn build(inner: *const _PreparedStatement) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        PreparedStatement(inner)
    }
}

impl PreparedStatement {
    /// Creates a bound statement from a pre-prepared statement.
    pub fn bind(&self) -> Statement {
        unsafe { Statement::build(cass_prepared_bind(self.0)) }
    }

    /// Gets the name of a parameter at the specified index.
    pub fn parameter_name(&self, index: usize) -> Result<&str> {
        let mut name = std::ptr::null();
        let mut name_length = 0;
        unsafe {
            cass_prepared_parameter_name(self.0, index, &mut name, &mut name_length)
                .to_result(())
                .and_then(|_| {
                    Ok(str::from_utf8(slice::from_raw_parts(
                        name as *const u8,
                        name_length,
                    ))?)
                })
        }
    }

    /// Gets the data type of a parameter at the specified index.
    ///
    /// Returns a reference to the data type of the parameter. Do not free
    /// this reference as it is bound to the lifetime of the prepared.
    pub fn parameter_data_type(&self, index: usize) -> ConstDataType {
        unsafe { ConstDataType::build(cass_prepared_parameter_data_type(self.0, index)) }
    }

    /// Gets the data type of a parameter for the specified name.
    ///
    /// Returns a reference to the data type of the parameter. Do not free
    /// this reference as it is bound to the lifetime of the prepared.
    pub fn parameter_data_type_by_name(&self, name: &str) -> ConstDataType {
        unsafe {
            let name_ptr = name.as_ptr() as *const c_char;
            ConstDataType::build(cass_prepared_parameter_data_type_by_name_n(
                self.0,
                name_ptr,
                name.len(),
            ))
        }
    }
}
