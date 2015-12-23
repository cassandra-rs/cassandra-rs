use cql_bindgen::cass_function_meta_argument;
use cql_bindgen::cass_function_meta_argument_count;
use cql_bindgen::cass_function_meta_argument_type_by_name;
use cql_bindgen::cass_function_meta_body;
use cql_bindgen::cass_function_meta_called_on_null_input;
use cql_bindgen::cass_function_meta_field_by_name;
use cql_bindgen::cass_function_meta_full_name;
use cql_bindgen::cass_function_meta_language;
use cql_bindgen::cass_function_meta_name;
use cql_bindgen::cass_function_meta_return_type;
use cql_bindgen::cass_iterator_fields_from_function_meta;
use cql_bindgen::CassFunctionMeta as _CassFunctionMeta;
use cql_bindgen::CASS_OK;

use cql_ffi::iterator::FieldIterator;

use std::{mem, slice, str};
use std::ffi::CString;
use cql_ffi::error::CassError;

use cql_ffi::data_type::ConstDataType;
use cql_ffi::value::Value;

pub struct FunctionMeta(pub *const _CassFunctionMeta);

impl FunctionMeta {
    pub fn fields_iter(&self) -> FieldIterator {
        unsafe { FieldIterator(cass_iterator_fields_from_function_meta(self.0)) }
    }

    ///Gets the name of the function.
    pub fn name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_function_meta_name(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize)).unwrap().to_string()
        }
    }

    /// Gets the full name of the function. The full name includes the
    ///function's name and the function's signature:
    ///"name(type1 type2.. typeN)".
    pub fn full_name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_function_meta_full_name(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize)).unwrap().to_string()
        }
    }

    ///Gets the body of the function.
    pub fn body(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_function_meta_body(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize)).unwrap().to_string()
        }
    }

    ///Gets the language of the function.
    pub fn language(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_function_meta_language(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length as usize)).unwrap().to_string()
        }
    }

    ///Gets whether a function is called on "null".
    pub fn called_on_null_input(&self) -> bool {
        unsafe { if cass_function_meta_called_on_null_input(self.0) > 0 { true } else { false } }
    }

    ///Gets the number of arguments this function takes.
    pub fn argument_count(&self) -> u64 { unsafe { cass_function_meta_argument_count(self.0) } }

    /// Gets the function's argument name and type for the provided index.
    pub fn argument(&self, index: u64) -> Result<(), CassError> {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            let mut data_type = mem::zeroed();

            match cass_function_meta_argument(self.0, index, &mut name, &mut name_length, &mut data_type) {
                CASS_OK => Ok(()),
                err => Err(CassError::build(err)),
            }
        }
    }

    /// Gets the function's argument and type for the provided name.
    pub fn argument_type_by_name(&self, name: &str) -> ConstDataType {
        unsafe { ConstDataType(cass_function_meta_argument_type_by_name(self.0, CString::new(name).unwrap().as_ptr())) }
    }

    ///Gets the return type of the function.
    pub fn return_type(&self) -> ConstDataType { unsafe { ConstDataType(cass_function_meta_return_type(self.0)) } }

    ///Gets a metadata field for the provided name. Metadata fields allow direct
    ///access to the column data found in the underlying "functions" metadata table.
    pub fn field_by_name(&self, name: &str) -> Value {
        unsafe { Value(cass_function_meta_field_by_name(self.0, CString::new(name).unwrap().as_ptr())) }
    }
}
