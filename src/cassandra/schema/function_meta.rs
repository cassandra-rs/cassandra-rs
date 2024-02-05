use crate::cassandra::data_type::ConstDataType;
use crate::cassandra::error::*;
use crate::cassandra::iterator::FieldIterator;
use crate::cassandra::util::{Protected, ProtectedInner};
use crate::cassandra::value::Value;

use crate::cassandra_sys::cass_function_meta_argument;
use crate::cassandra_sys::cass_function_meta_argument_count;
use crate::cassandra_sys::cass_function_meta_argument_type_by_name_n;
use crate::cassandra_sys::cass_function_meta_body;
use crate::cassandra_sys::cass_function_meta_called_on_null_input;
use crate::cassandra_sys::cass_function_meta_field_by_name_n;
use crate::cassandra_sys::cass_function_meta_full_name;
use crate::cassandra_sys::cass_function_meta_language;
use crate::cassandra_sys::cass_function_meta_name;
use crate::cassandra_sys::cass_function_meta_return_type;
use crate::cassandra_sys::cass_iterator_fields_from_function_meta;
use crate::cassandra_sys::cass_true;
use crate::cassandra_sys::CassFunctionMeta as _CassFunctionMeta;

use std::marker::PhantomData;
use std::os::raw::c_char;
use std::{slice, str};

/// The metadata for a function
//
// Borrowed immutably.
#[derive(Debug)]
pub struct FunctionMeta<'a>(*const _CassFunctionMeta, PhantomData<&'a _CassFunctionMeta>);

impl ProtectedInner<*const _CassFunctionMeta> for FunctionMeta<'_> {
    fn inner(&self) -> *const _CassFunctionMeta {
        self.0
    }
}

impl Protected<*const _CassFunctionMeta> for FunctionMeta<'_> {
    fn build(inner: *const _CassFunctionMeta) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        FunctionMeta(inner, PhantomData)
    }
}

impl<'a> FunctionMeta<'a> {
    /// Iterator over the fields in this function.
    pub fn fields_iter(&self) -> FieldIterator<'a> {
        unsafe { FieldIterator::build(cass_iterator_fields_from_function_meta(self.0)) }
    }

    /// Gets the name of the function.
    pub fn get_name(&self) -> String {
        let mut name = std::ptr::null();
        let mut name_length = 0;
        unsafe {
            cass_function_meta_name(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length))
                .expect("must be utf8")
                .to_owned()
        }
    }

    /// Gets the full name of the function. The full name includes the
    /// function's name and the function's signature:
    /// "name(type1 type2.. typeN)".
    pub fn full_name(&self) -> String {
        let mut name = std::ptr::null();
        let mut name_length = 0;
        unsafe {
            cass_function_meta_full_name(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length))
                .expect("must be utf8")
                .to_owned()
        }
    }

    /// Gets the body of the function.
    pub fn body(&self) -> String {
        let mut name = std::ptr::null();
        let mut name_length = 0;
        unsafe {
            cass_function_meta_body(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length))
                .expect("must be utf8")
                .to_owned()
        }
    }

    /// Gets the language of the function.
    pub fn language(&self) -> String {
        let mut name = std::ptr::null();
        let mut name_length = 0;
        unsafe {
            cass_function_meta_language(self.0, &mut name, &mut name_length);
            str::from_utf8(slice::from_raw_parts(name as *const u8, name_length))
                .expect("must be utf8")
                .to_owned()
        }
    }

    /// Gets whether a function is called on "null".
    pub fn called_on_null_input(&self) -> bool {
        unsafe { cass_function_meta_called_on_null_input(self.0) == cass_true }
    }

    /// Gets the number of arguments this function takes.
    pub fn argument_count(&self) -> usize {
        unsafe { cass_function_meta_argument_count(self.0) }
    }

    /// Gets the function's argument name and type for the provided index.
    pub fn argument(&self, index: usize) -> Result<(String, ConstDataType<'a>)> {
        let mut name = std::ptr::null();
        let mut name_length = 0;
        let mut data_type = std::ptr::null();
        unsafe {
            cass_function_meta_argument(self.0, index, &mut name, &mut name_length, &mut data_type)
                .to_result(())?;
            let name = str::from_utf8(slice::from_raw_parts(name as *const u8, name_length))
                .expect("must be utf8")
                .to_owned();
            let data_type = ConstDataType::build(data_type);
            Ok((name, data_type))
        }
    }

    /// Gets the function's argument and type for the provided name.
    pub fn argument_type_by_name(&self, name: &str) -> ConstDataType<'a> {
        unsafe {
            let name_ptr = name.as_ptr() as *const c_char;
            // TODO: can return NULL
            ConstDataType::build(cass_function_meta_argument_type_by_name_n(
                self.0,
                name_ptr,
                name.len(),
            ))
        }
    }

    /// Gets the return type of the function.
    pub fn return_type(&self) -> ConstDataType<'a> {
        unsafe { ConstDataType::build(cass_function_meta_return_type(self.0)) }
    }

    /// Gets a metadata field for the provided name. Metadata fields allow direct
    /// access to the column data found in the underlying "functions" metadata table.
    pub fn field_by_name(&self, name: &str) -> Value<'a> {
        unsafe {
            let name_ptr = name.as_ptr() as *const c_char;
            // TODO: can return NULL
            Value::build(cass_function_meta_field_by_name_n(
                self.0,
                name_ptr,
                name.len(),
            ))
        }
    }
}
