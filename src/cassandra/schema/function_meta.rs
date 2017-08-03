use cassandra::data_type::ConstDataType;
use cassandra::iterator::FieldIterator;
use cassandra::util::Protected;
use cassandra::value::Value;
use cassandra::error::*;

use cassandra_sys::CASS_OK;
use cassandra_sys::CassFunctionMeta as _CassFunctionMeta;
use cassandra_sys::cass_function_meta_argument;
use cassandra_sys::cass_function_meta_argument_count;
use cassandra_sys::cass_function_meta_argument_type_by_name;
use cassandra_sys::cass_function_meta_body;
use cassandra_sys::cass_function_meta_called_on_null_input;
use cassandra_sys::cass_function_meta_field_by_name;
use cassandra_sys::cass_function_meta_full_name;
use cassandra_sys::cass_function_meta_language;
use cassandra_sys::cass_function_meta_name;
use cassandra_sys::cass_function_meta_return_type;
use cassandra_sys::cass_iterator_fields_from_function_meta;
use cassandra_sys::cass_true;

use std::{mem, slice, str};
use std::ffi::CString;

/// The metadata for a function
#[derive(Debug)]
pub struct FunctionMeta(*const _CassFunctionMeta);

impl Protected<*const _CassFunctionMeta> for FunctionMeta {
    fn inner(&self) -> *const _CassFunctionMeta { self.0 }
    fn build(inner: *const _CassFunctionMeta) -> Self { FunctionMeta(inner) }
}

impl FunctionMeta {
    /// Iterator over the fields in this function
    pub fn fields_iter(&self) -> FieldIterator {
        unsafe { FieldIterator::build(cass_iterator_fields_from_function_meta(self.0)) }
    }

    /// Gets the name of the function.
    #[allow(cast_possible_truncation)]
    pub fn get_name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_function_meta_name(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize))
                .expect("must be utf8")
                .to_owned()
        }
    }

    /// Gets the full name of the function. The full name includes the
    /// function's name and the function's signature:
    /// "name(type1 type2.. typeN)".
    #[allow(cast_possible_truncation)]
    pub fn full_name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_function_meta_full_name(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize))
                .expect("must be utf8")
                .to_owned()
        }
    }

    /// Gets the body of the function.
    #[allow(cast_possible_truncation)]
    pub fn body(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_function_meta_body(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize))
                .expect("must be utf8")
                .to_owned()
        }
    }

    /// Gets the language of the function.
    #[allow(cast_possible_truncation)]
    pub fn language(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_function_meta_language(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize))
                .expect("must be utf8")
                .to_owned()
        }
    }

    /// Gets whether a function is called on "null".
    pub fn called_on_null_input(&self) -> bool {
        unsafe { cass_function_meta_called_on_null_input(self.0) == cass_true }
    }

    /// Gets the number of arguments this function takes.
    pub fn argument_count(&self) -> usize { unsafe { cass_function_meta_argument_count(self.0) } }

    /// Gets the function's argument name and type for the provided index.
    pub fn argument(&self, index: usize) -> Result<()> {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            let mut data_type = mem::zeroed();

            cass_function_meta_argument(self.0, index, &mut name, &mut name_length, &mut data_type).to_result(())
        }
    }

    /// Gets the function's argument and type for the provided name.
    pub fn argument_type_by_name(&self, name: &str) -> ConstDataType {
        unsafe {
            ConstDataType(cass_function_meta_argument_type_by_name(self.0,
                                                                   CString::new(name)
                                                                       .expect("must be utf8")
                                                                       .as_ptr()))
        }
    }

    /// Gets the return type of the function.
    pub fn return_type(&self) -> ConstDataType { unsafe { ConstDataType(cass_function_meta_return_type(self.0)) } }

    /// Gets a metadata field for the provided name. Metadata fields allow direct
    /// access to the column data found in the underlying "functions" metadata table.
    pub fn field_by_name(&self, name: &str) -> Value {
        unsafe {
            Value::build(cass_function_meta_field_by_name(self.0, CString::new(name).expect("must be utf8").as_ptr()))
        }
    }
}
