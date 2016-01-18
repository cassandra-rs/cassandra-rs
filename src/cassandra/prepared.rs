use cassandra::statement::Statement;

use cassandra_sys::CassPrepared as _PreparedStatement;
// use cassandra_sys::cass_prepared_free;
use cassandra_sys::cass_prepared_bind;
use cassandra_sys::cass_prepared_parameter_name;
use cassandra_sys::cass_prepared_parameter_data_type;
use cassandra_sys::cass_prepared_parameter_data_type_by_name;
use cassandra::data_type::ConstDataType;
use cassandra::statement;
use std::{mem, slice, str};
use std::ffi::CString;

/// A statement that has been prepared against at least one Cassandra node.
/// Instances of this class should not be created directly, but through Session.prepare().
pub struct PreparedStatement(*const _PreparedStatement);

unsafe impl Sync for PreparedStatement {}
unsafe impl Send for PreparedStatement {}

pub mod protected {
    use cassandra::prepared::PreparedStatement;
    use cassandra_sys::CassPrepared as _PreparedStatement;
    pub fn build(prepared: *const _PreparedStatement) -> PreparedStatement {
        PreparedStatement(prepared)
    }
}

// impl Drop for PreparedStatement {
//    ///Frees a prepared instance.
//    fn drop(&mut self) {
//        unsafe { cass_prepared_free(self.0) }
//    }
// }

impl PreparedStatement {
    ///Creates a bound statement from a pre-prepared statement.
    pub fn bind(&self) -> Statement {
        unsafe { statement::protected::build(cass_prepared_bind(self.0)) }
    }

    ///Gets the name of a parameter at the specified index.
    pub fn parameter_name(&self, index: u64) -> Result<&str, str::Utf8Error> {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_prepared_parameter_name(self.0, index, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize))
        }
    }

    ///Gets the data type of a parameter at the specified index.
    ///
    ///Returns a reference to the data type of the parameter. Do not free
    ///this reference as it is bound to the lifetime of the prepared.
    pub fn parameter_data_type(&self, index: u64) -> ConstDataType {
        unsafe { ConstDataType(cass_prepared_parameter_data_type(self.0, index)) }
    }

    ///Gets the data type of a parameter for the specified name.
    ///
    ///Returns a reference to the data type of the parameter. Do not free
    ///this reference as it is bound to the lifetime of the prepared.
    pub fn parameter_data_type_by_name(&self, name: &str) -> ConstDataType {
        unsafe {
            ConstDataType(cass_prepared_parameter_data_type_by_name(self.0,
                                                                    CString::new(name)
                                                                        .unwrap()
                                                                        .as_ptr()))
        }
    }
}
