use std::ffi::CString;

use cql_bindgen::cass_user_type_set_uuid_by_name;
use cql_bindgen::cass_user_type_set_user_type_by_name;
use cql_bindgen::cass_user_type_set_uint32;
use cql_bindgen::cass_user_type_set_uint32_by_name;
use cql_bindgen::cass_user_type_set_int8;
use cql_bindgen::cass_user_type_set_int8_by_name;
use cql_bindgen::cass_user_type_set_int16;
use cql_bindgen::cass_user_type_set_int16_by_name;
#[allow(unused_imports)]
use cql_bindgen::cass_user_type_set_decimal;
#[allow(unused_imports)]
use cql_bindgen::cass_user_type_set_decimal_by_name;
use cql_bindgen::cass_user_type_set_bool_by_name;
use cql_bindgen::cass_user_type_set_bytes_by_name;
use cql_bindgen::cass_user_type_set_inet_by_name;
use cql_bindgen::cass_user_type_set_null_by_name;
use cql_bindgen::cass_user_type_set_tuple_by_name;
use cql_bindgen::cass_user_type_free;
use cql_bindgen::cass_user_type_data_type;
use cql_bindgen::cass_user_type_set_null;
use cql_bindgen::cass_user_type_set_int32;
use cql_bindgen::cass_user_type_set_int32_by_name;
use cql_bindgen::cass_user_type_set_int64;
use cql_bindgen::cass_user_type_set_int64_by_name;
use cql_bindgen::cass_user_type_set_float;
use cql_bindgen::cass_user_type_set_float_by_name;
use cql_bindgen::cass_user_type_set_double;
use cql_bindgen::cass_user_type_set_double_by_name;
use cql_bindgen::cass_user_type_set_bool;
use cql_bindgen::cass_user_type_set_string;
use cql_bindgen::cass_user_type_set_string_by_name;
use cql_bindgen::cass_user_type_set_bytes;
use cql_bindgen::cass_user_type_set_uuid;
use cql_bindgen::cass_user_type_set_inet;
use cql_bindgen::cass_user_type_set_collection;
use cql_bindgen::cass_user_type_set_collection_by_name;
use cql_bindgen::cass_user_type_set_tuple;
use cql_bindgen::cass_user_type_set_user_type;
use cql_bindgen::CassUserType as _CassUserType;
use cql_bindgen::cass_iterator_fields_from_user_type;

use cql_ffi::value::Value;
use cql_ffi::uuid::Uuid;
use cql_ffi::inet::Inet;
use cql_ffi::collection::Set;
use cql_ffi::tuple::Tuple;
use cql_ffi::error::CassError;
use cql_ffi::data_type::ConstDataType;
use cql_ffi::iterator::FieldIterator;

pub struct UserType(pub *mut _CassUserType);

impl Drop for UserType {
    fn drop(&mut self) { unsafe { cass_user_type_free(self.0) } }
}

// impl Drop for UserType {
//    fn drop(&mut self) {unsafe{
//        cass_user_type_free(self.0)
//    }}
// }

impl UserType {
    ///Gets the data type of a user defined type.
    pub fn data_type(&self) -> ConstDataType { unsafe { ConstDataType(cass_user_type_data_type(self.0)) } }

    ///Sets a null in a user defined type at the specified index.
    pub fn set_null(&mut self, index: u64) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_null(self.0, index)).wrap(()) }
    }

    /// Sets a null in a user defined type at the specified name.
    pub fn set_null_by_name<S>(&mut self, name: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_null_by_name(self.0, name.as_ptr())).wrap(())
        }
    }

    ///Sets a "tinyint" in a user defined type at the specified index.
    pub fn set_int8(&mut self, index: u64, value: i8) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_int8(self.0, index, value)).wrap(()) }
    }

    ///Sets a "tinyint" in a user defined type at the specified name.
    pub fn set_int8_by_name<S>(&mut self, name: S, value: i8) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    let rc = cass_user_type_set_int8_by_name(self.0, name.as_ptr(), value);
                    CassError::build(rc).wrap(())
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }

    ///Sets an "smallint" in a user defined type at the specified index.
    pub fn set_int16(&mut self, index: u64, value: i16) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_int16(self.0, index, value)).wrap(()) }
    }

    ///Sets an "smallint" in a user defined type at the specified name.
    pub fn set_int16_by_name<S>(&mut self, name: S, value: i16) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    let rc = cass_user_type_set_int16_by_name(self.0, name.as_ptr(), value);
                    CassError::build(rc).wrap(())
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }

    ///Sets an "int" in a user defined type at the specified index.
    pub fn set_int32(&mut self, index: u64, value: i32) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_int32(self.0, index, value)).wrap(()) }
    }

    ///Sets an "int" in a user defined type at the specified name.
    pub fn set_int32_by_name<S>(&mut self, name: S, value: i32) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    let rc = cass_user_type_set_int32_by_name(self.0, name.as_ptr(), value);
                    CassError::build(rc).wrap(())
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }

    ///Sets a "date" in a user defined type at the specified index.
    pub fn set_uint32(&mut self, index: u64, value: u32) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_uint32(self.0, index, value)).wrap(()) }
    }

    ///Sets a "date" in a user defined type at the specified name.
    pub fn set_uint32_by_name<S>(&mut self, name: S, value: u32) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    let rc = cass_user_type_set_uint32_by_name(self.0, name.as_ptr(), value);
                    CassError::build(rc).wrap(())
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }


    ///Sets an "bigint", "counter", "timestamp" or "time" in a
    ///user defined type at the specified index.
    pub fn set_int64(&mut self, index: u64, value: i64) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_int64(self.0, index, value)).wrap(()) }
    }

    ///Sets an "bigint", "counter", "timestamp" or "time" in a
    ///user defined type at the specified name.
    pub fn set_int64_by_name<S>(&mut self, name: S, value: i64) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_int64_by_name(self.0, name.as_ptr(), value)).wrap(())
        }
    }

    ///Sets a "float" in a user defined type at the specified index.
    pub fn set_float(&mut self, index: u64, value: f32) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_float(self.0, index, value)).wrap(()) }
    }

    /// Sets a "float" in a user defined type at the specified name.
    pub fn set_float_by_name<S>(&mut self, name: S, value: f32) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_float_by_name(self.0, name.as_ptr(), value)).wrap(())
        }
    }

    ///Sets an "double" in a user defined type at the specified index.
    pub fn set_double(&mut self, index: u64, value: f64) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_double(self.0, index, value)).wrap(()) }
    }

    ///Sets an "double" in a user defined type at the specified name.

    pub fn set_double_by_name<S>(&mut self, name: S, value: f64) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_double_by_name(self.0, name.as_ptr(), value)).wrap(())
        }
    }

    ///Sets a "boolean" in a user defined type at the specified index.
    pub fn set_bool(&mut self, index: u64, value: bool) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_bool(self.0, index, if value { 1u32 } else { 0u32 })).wrap(()) }
    }

    ///Sets a "boolean" in a user defined type at the specified name.
    pub fn set_bool_by_name<S>(&mut self, name: S, value: bool) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_bool_by_name(self.0, name.as_ptr(), if value { 1u32 } else { 0u32 }))
                .wrap(())
        }
    }

    ///Sets an "ascii", "text" or "varchar" in a user defined type at the
    ///specified index.
    pub fn set_stringl<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let value = CString::new(value.into()).unwrap();
            CassError::build(cass_user_type_set_string(self.0, index, value.as_ptr())).wrap(())
        }
    }

    ///Sets an "ascii", "text" or "varchar" in a user defined type at the
    ///specified name.
    pub fn set_string_by_name<S>(&mut self, name: S, value: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            let value = CString::new(value.into()).unwrap();
            CassError::build(cass_user_type_set_string_by_name(self.0, name.as_ptr(), value.as_ptr())).wrap(())
        }
    }

    // FIXME. right way to pass the vec?
    ///Sets a "blob" "varint" or "custom" in a user defined type at the specified index.
    pub fn set_bytes(&mut self, index: u64, value: Vec<u8>) -> Result<(), CassError> {
        unsafe {
            CassError::build(cass_user_type_set_bytes(self.0, index, value.as_ptr(), value.len() as u64)).wrap(())
        }
    }

    ///Sets a "blob", "varint" or "custom" in a user defined type at the specified name.
    pub fn set_bytes_by_name<S>(&mut self, name: S, value: Vec<u8>) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_bytes_by_name(self.0,
                                                              name.as_ptr(),
                                                              value.as_ptr(),
                                                              value.len() as u64))
                .wrap(())
        }
    }

    ///Sets a "uuid" or "timeuuid" in a user defined type at the specified index.
    pub fn set_uuid<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<Uuid>
    {
        unsafe { CassError::build(cass_user_type_set_uuid(self.0, index, value.into().0)).wrap(()) }
    }

    ///Sets a "uuid" or "timeuuid" in a user defined type at the specified name.
    pub fn set_uuid_by_name<S, U>(&mut self, name: S, value: U) -> Result<(), CassError>
        where S: Into<String>,
              U: Into<Uuid>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_uuid_by_name(self.0, name.as_ptr(), value.into().0)).wrap(())
        }
    }

    ///Sets a "inet" in a user defined type at the specified index.
    pub fn set_inet<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<Inet>
    {
        unsafe { CassError::build(cass_user_type_set_inet(self.0, index, value.into().0)).wrap(()) }
    }

    ///Sets a "inet" in a user defined type at the specified name.
    pub fn set_inet_by_name<S, U>(&mut self, name: S, value: U) -> Result<(), CassError>
        where S: Into<String>,
              U: Into<Inet>
    {
        let name = CString::new(name.into()).unwrap();
        unsafe { CassError::build(cass_user_type_set_inet_by_name(self.0, name.as_ptr(), value.into().0)).wrap(()) }
    }

    ///Sets a "list", "map" or "set" in a user defined type at the specified index.
    pub fn set_collection<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<Set>
    {
        unsafe { CassError::build(cass_user_type_set_collection(self.0, index, value.into().0)).wrap(()) }
    }

    ///Sets a "list", "map" or "set" in a user defined type at the
    ///specified name.
    pub fn set_collection_by_name<S>(&mut self, name: S, value: Set) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_collection_by_name(self.0, name.as_ptr(), value.0)).wrap(())
        }
    }

    ///Sets a "tuple" in a user defined type at the specified index.
    pub fn set_tuple(&mut self, index: u64, value: Tuple) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_tuple(self.0, index, value.0)).wrap(()) }
    }

    ///Sets a "tuple" in a user defined type at the specified name.
    pub fn set_tuple_by_name<S>(&mut self, name: S, value: Tuple) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_tuple_by_name(self.0, name.as_ptr(), value.0)).wrap(())
        }
    }

    ///Sets a user defined type in a user defined type at the specified index.
    pub fn set_user_type(&mut self, index: u64, value: UserType) -> Result<(), CassError> {
        unsafe { CassError::build(cass_user_type_set_user_type(self.0, index, value.0)).wrap(()) }
    }

    ///Sets a user defined type in a user defined type at the specified name.
    pub fn set_user_type_by_name<S>(&mut self, name: S, value: UserType) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(cass_user_type_set_user_type_by_name(self.0, name.as_ptr(), value.0)).wrap(())
        }
    }
}
