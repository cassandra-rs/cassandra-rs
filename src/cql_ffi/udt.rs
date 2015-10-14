use std::ffi::CString;
use std::mem;
use std::slice;
use std::str;

use cql_ffi::value::CassValue;
use cql_ffi::uuid::Uuid;
use cql_ffi::inet::CassInet;
use cql_ffi::collection::set::CassSet;
use cql_ffi::tuple::CassTuple;

use cql_bindgen::cass_data_type_new;
use cql_bindgen::CassIterator as _CassIterator;


use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
//use cql_bindgen::cass_iterator_get_value;

use cql_bindgen::cass_data_type_new_from_existing;

use cql_bindgen::cass_data_type_new_tuple;
use cql_bindgen::cass_data_type_new_udt;
//use cql_bindgen::cass_data_type_free;
use cql_bindgen::cass_data_type_type;
use cql_bindgen::cass_data_type_type_name;
use cql_bindgen::cass_data_type_set_type_name;

use cql_bindgen::cass_data_type_set_type_name_n;
use cql_bindgen::cass_data_type_keyspace;
use cql_bindgen::cass_data_type_set_keyspace;
use cql_bindgen::cass_data_type_set_keyspace_n;
use cql_bindgen::cass_data_type_class_name;
use cql_bindgen::cass_data_type_set_class_name;

use cql_bindgen::cass_data_type_set_class_name_n;
use cql_bindgen::cass_data_type_sub_data_type;
use cql_bindgen::cass_data_type_sub_data_type_by_name;
use cql_bindgen::cass_data_type_sub_data_type_by_name_n;
use cql_bindgen::cass_data_type_sub_type_name;
use cql_bindgen::cass_data_type_add_sub_type;
use cql_bindgen::cass_data_type_add_sub_type_by_name;
//use cql_bindgen::cass_data_type_add_sub_type_by_name_n;
use cql_bindgen::cass_data_type_add_sub_value_type;
//use cql_bindgen::cass_data_type_add_sub_value_type_by_name;
//use cql_bindgen::cass_data_type_add_sub_value_type_by_name_n;
use cql_bindgen::cass_user_type_new_from_data_type;
//use cql_bindgen::cass_user_type_free;
use cql_bindgen::cass_user_type_data_type;
use cql_bindgen::cass_user_type_set_null;
//use cql_bindgen::cass_user_type_set_null_by_name;
//use cql_bindgen::cass_user_type_set_null_by_name_n;
use cql_bindgen::cass_user_type_set_int32;
use cql_bindgen::cass_user_type_set_int32_by_name;
//use cql_bindgen::cass_user_type_set_int32_by_name_n;
use cql_bindgen::cass_user_type_set_int64;
use cql_bindgen::cass_user_type_set_int64_by_name;
//use cql_bindgen::cass_user_type_set_int64_by_name_n;
use cql_bindgen::cass_user_type_set_float;
use cql_bindgen::cass_user_type_set_float_by_name;
//use cql_bindgen::cass_user_type_set_float_by_name_n;
use cql_bindgen::cass_user_type_set_double;
use cql_bindgen::cass_user_type_set_double_by_name;
//use cql_bindgen::cass_user_type_set_double_by_name_n;
use cql_bindgen::cass_user_type_set_bool;
//use cql_bindgen::cass_user_type_set_bool_by_name;
//use cql_bindgen::cass_user_type_set_bool_by_name_n;
use cql_bindgen::cass_user_type_set_string;
//use cql_bindgen::cass_user_type_set_string_n;
use cql_bindgen::cass_user_type_set_string_by_name;
//use cql_bindgen::cass_user_type_set_string_by_name_n;
use cql_bindgen::cass_user_type_set_bytes;
//use cql_bindgen::cass_user_type_set_bytes_by_name;
//use cql_bindgen::cass_user_type_set_bytes_by_name_n;
use cql_bindgen::cass_user_type_set_uuid;
//use cql_bindgen::cass_user_type_set_uuid_by_name;
//use cql_bindgen::cass_user_type_set_uuid_by_name_n;
use cql_bindgen::cass_user_type_set_inet;
//use cql_bindgen::cass_user_type_set_inet_by_name;
//use cql_bindgen::cass_user_type_set_inet_by_name_n;
//use cql_bindgen::cass_user_type_set_decimal;
//use cql_bindgen::cass_user_type_set_decimal_by_name;
//use cql_bindgen::cass_user_type_set_decimal_by_name_n;
use cql_bindgen::cass_user_type_set_collection;
use cql_bindgen::cass_user_type_set_collection_by_name;
//use cql_bindgen::cass_user_type_set_collection_by_name_n;
use cql_bindgen::cass_user_type_set_tuple;
//use cql_bindgen::cass_user_type_set_tuple_by_name;
//use cql_bindgen::cass_user_type_set_tuple_by_name_n;
use cql_bindgen::cass_user_type_set_user_type;
//use cql_bindgen::cass_user_type_set_user_type_by_name;
//use cql_bindgen::cass_user_type_set_user_type_by_name_n;

use cql_bindgen::cass_iterator_get_user_type_field_name;
use cql_bindgen::cass_iterator_get_user_type_field_value;

use cql_bindgen::CassDataType as _CassDataType;
use cql_bindgen::CassUserType as _CassUserType;

use cql_ffi::value::ValueType;
use cql_ffi::error::CassError;

pub struct CassDataType(pub *mut _CassDataType);
pub struct CassConstDataType(pub *const _CassDataType);

pub struct CassUserType(pub *mut _CassUserType);

impl CassDataType {
    pub fn new(value_type: ValueType) -> Self {
        unsafe {
            CassDataType(cass_data_type_new(value_type as u32))
        }
    }

    pub fn new_from_existing(data_type: CassDataType) -> Self {
        unsafe {
            CassDataType(cass_data_type_new_from_existing(data_type.0))
        }
    }

    pub fn new_tuple(item_count: u64) -> CassDataType {
        unsafe {
            CassDataType(cass_data_type_new_tuple(item_count))
        }
    }

    pub fn new_udt(field_count: u64) -> CassDataType {
        unsafe {
            CassDataType(cass_data_type_new_udt(field_count))
        }
    }

    pub fn get_type(data_type: CassDataType) -> ValueType {
        unsafe {
            ValueType::build(cass_data_type_type(data_type.0))
        }
    }

    pub fn type_name<S>(data_type: CassDataType, type_name: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let type_name = CString::new(type_name.into()).unwrap();
            CassError::build(
                cass_data_type_type_name(
                    data_type.0,
                    &mut type_name.as_ptr(),
                    &mut(type_name.as_bytes().len() as u64)
                )
            ).wrap(())
        }
    }

    pub fn set_type_name<S>(data_type: CassDataType, type_name: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let type_name = CString::new(type_name.into()).unwrap();
            CassError::build(
                cass_data_type_set_type_name(
                    data_type.0,
                    type_name.as_ptr())
                )
            .wrap(())
        }
    }

    pub fn set_type_name_n<S>(data_type: CassDataType, type_name: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let type_name = CString::new(type_name.into()).unwrap();
            CassError::build(
                cass_data_type_set_type_name_n(
                    data_type.0,
                    type_name.as_ptr(),
                    type_name.as_bytes().len() as u64
                )
            ).wrap(())
        }
    }

    pub fn keyspace<S>(data_type: CassDataType, keyspace: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let keyspace = CString::new(keyspace.into()).unwrap();
            CassError::build(
                cass_data_type_keyspace(
                    data_type.0,
                    &mut(keyspace.as_ptr()),
                    &mut(keyspace.as_bytes().len() as u64)
                )
            ).wrap(())
        }
    }

    pub fn set_keyspace<S>(data_type: CassDataType, keyspace: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let keyspace = CString::new(keyspace.into()).unwrap();
            CassError::build(
                cass_data_type_set_keyspace(
                    data_type.0,
                    keyspace.as_ptr()
                )
            ).wrap(())
        }
    }

    pub fn set_keyspace_n<S>(data_type: CassDataType, keyspace: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let keyspace = CString::new(keyspace.into()).unwrap();
            CassError::build(
                cass_data_type_set_keyspace_n(
                    data_type.0,
                    keyspace.as_ptr(),
                    keyspace.as_bytes().len() as u64
                )
            ).wrap(())
        }
    }

    pub fn class_name<S>(data_type: CassDataType, class_name: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let class_name = CString::new(class_name.into()).unwrap();
            CassError::build(
                cass_data_type_class_name(
                    data_type.0,
                    &mut class_name.as_ptr(),
                    &mut(class_name.as_bytes().len() as u64)
                )
            ).wrap(())
        }
    }


    pub fn set_class_name<S>(data_type: CassDataType, class_name: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let class_name = CString::new(class_name.into()).unwrap();
            CassError::build(
                cass_data_type_set_class_name(
                    data_type.0,
                    class_name.as_ptr()
                )
            ).wrap(())
        }
    }

    pub fn set_class_name_n<S>(data_type: CassDataType, class_name: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let class_name = CString::new(class_name.into()).unwrap();
            CassError::build(
                cass_data_type_set_class_name_n(
                    data_type.0,
                    class_name.as_ptr(),
                    class_name.as_bytes().len() as u64
                )
            ).wrap(())
        }
    }

    pub fn sub_data_type(data_type: CassDataType, index: u64) -> CassConstDataType {
        unsafe {
            CassConstDataType(cass_data_type_sub_data_type(data_type.0, index))
        }
    }

    pub fn sub_data_type_by_name<S>(data_type: CassDataType, name: S) -> CassConstDataType
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassConstDataType(cass_data_type_sub_data_type_by_name(data_type.0, name.as_ptr()))
        }
    }

    pub fn sub_data_type_by_name_n<S>(data_type: CassDataType, name: S) -> CassConstDataType
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassConstDataType(cass_data_type_sub_data_type_by_name_n(data_type.0,
                                                                     name.as_ptr(),
                                                                     name.as_bytes().len() as u64))
        }
    }


    pub fn sub_type_name<S>(data_type: CassDataType, index: u64, name: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(
                cass_data_type_sub_type_name(
                    data_type.0,
                    index,
                    &mut name.as_ptr(),
                    &mut(name.as_bytes().len() as u64)
                )
            ).wrap(())
        }
    }

    pub fn add_sub_type(data_type: CassDataType,
                        sub_data_type: CassDataType)
                        -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_data_type_add_sub_type(data_type.0, sub_data_type.0)
            ).wrap(())
        }
    }

    pub fn add_sub_type_by_name<S>(data_type: CassDataType,
                                   name: S,
                                   sub_data_type: CassDataType)
                                   -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(
                cass_data_type_add_sub_type_by_name(
                    data_type.0,
                    name.as_ptr(),
                    sub_data_type.0
                )
            ).wrap(())
        }
    }

    pub fn add_sub_value_type<S>(data_type: CassDataType,
                                 sub_value_type: ValueType)
                                 -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            CassError::build(
                cass_data_type_add_sub_value_type(
                    data_type.0,
                    sub_value_type as u32
                )
            ).wrap(())
        }
    }
}

//impl Drop for CassDataType {
//    fn drop(&mut self) {unsafe{
//        cass_data_type_free(&mut self.0)
//    }}
//}

//impl Drop for CassUserType {
//    fn drop(&mut self) {unsafe{
//        cass_user_type_free(self.0)
//    }}
//}

impl CassUserType {
    pub fn new(data_type: CassConstDataType) -> Self {
        unsafe {
            CassUserType(cass_user_type_new_from_data_type(data_type.0))
        }
    }

    pub fn data_type(&self) -> CassConstDataType {
        unsafe {
            CassConstDataType(cass_user_type_data_type(self.0))
        }
    }

    pub fn set_null(&mut self, index: u64) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_user_type_set_null(
                    self.0,
                    index
                )
            ).wrap(())
        }
    }

    pub fn set_int32(&mut self, index: u64, value: i32) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_user_type_set_int32(
                    self.0,
                    index,
                    value
                )
            ).wrap(())
        }
    }

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

    pub fn set_int64(&mut self, index: u64, value: i64) -> Result<(), CassError> {
        unsafe {
            CassError::build(cass_user_type_set_int64(self.0, index, value)).wrap(())
        }
    }

    pub fn set_int64_by_name<S>(&mut self, name: S, value: i64) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(
                cass_user_type_set_int64_by_name(
                    self.0,
                    name.as_ptr(),
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_float(&mut self, index: u64, value: f32) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_user_type_set_float(
                    self.0,
                    index,
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_float_by_name<S>(&mut self, name: S, value: f32) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(
                cass_user_type_set_float_by_name(
                    self.0,
                    name.as_ptr(),
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_double(&mut self, index: u64, value: f64) -> Result<(), CassError> {
        unsafe {
            CassError::build(cass_user_type_set_double(self.0, index, value)).wrap(())
        }
    }

    pub fn set_double_by_name<S>(&mut self, name: S, value: f64) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(
                cass_user_type_set_double_by_name(
                    self.0,
                    name.as_ptr(),
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_bool(&mut self, index: u64, value: bool) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_user_type_set_bool(
                    self.0,
                    index,
                    if value {1u32} else {0u32}
                )
            ).wrap(())
        }
    }

    pub fn set_stringl<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let value = CString::new(value.into()).unwrap();
            CassError::build(cass_user_type_set_string(self.0, index, value.as_ptr())).wrap(())
        }
    }

    pub fn set_string_by_name<S>(&mut self, name: S, value: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            let value = CString::new(value.into()).unwrap();
            CassError::build(
                cass_user_type_set_string_by_name(
                    self.0,
                    name.as_ptr(),
                    value.as_ptr()
                )
            ).wrap(())
        }
    }

    //FIXME. right way to pass the vec?
    pub fn set_bytes(&mut self, index: u64, value: Vec<u8>) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_user_type_set_bytes(
                    self.0,
                    index,
                    value.as_ptr(),
                    value.len() as u64
                )
            ).wrap(())
        }
    }

    pub fn set_uuid<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<Uuid>
    {
        unsafe {
            CassError::build(cass_user_type_set_uuid(self.0, index, value.into().0)).wrap(())
        }
    }

    pub fn set_inet<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<CassInet>
    {
        unsafe {
            CassError::build(cass_user_type_set_inet(self.0, index, value.into().0)).wrap(())
        }
    }

    pub fn set_collection<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<CassSet>
    {
        unsafe {
            CassError::build(cass_user_type_set_collection(self.0, index, value.into().0)).wrap(())
        }
    }

    pub fn set_collection_by_name<S>(&mut self, name: S, value: CassSet) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassError::build(
                cass_user_type_set_collection_by_name(
                    self.0,
                    name.as_ptr(),
                    value.0
                )
            ).wrap(())
        }
    }

    pub fn set_tuple(&mut self, index: u64, value: CassTuple) -> Result<(), CassError> {
        unsafe {
            CassError::build(cass_user_type_set_tuple(self.0, index, value.0)).wrap(())
        }
    }

    pub fn set_user_type(&mut self, index: u64, value: CassUserType) -> Result<(), CassError> {
        unsafe {
            CassError::build(cass_user_type_set_user_type(self.0, index, value.0)).wrap(())
        }
    }
}

pub struct UserTypeIterator(pub *mut _CassIterator);

impl Drop for UserTypeIterator {
    fn drop(&mut self) {
        unsafe {
            cass_iterator_free(self.0)
        }
    }
}

impl Iterator for UserTypeIterator {
    type Item = (String,CassValue);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                //cass_iterator_get_user_type_field_name(fields, &field_name, &field_name_length);
                _ => {//
                    let field_name = mem::zeroed();
                    let field_name_length = mem::zeroed();
                    cass_iterator_get_user_type_field_name(self.0, field_name, field_name_length);
                    let slice = slice::from_raw_parts(field_name as *const u8,
                                                      field_name_length as usize);
                    let key = str::from_utf8(slice).unwrap().to_owned();

                    let field_value = cass_iterator_get_user_type_field_value(self.0);
                    Some((key, CassValue::new(field_value)))
                }
            }
        }
    }
}

impl UserTypeIterator {
//    pub fn get_field_name(&mut self)-> CassValue {unsafe{
//
//        CassValue::new(cass_iterator_get_user_type_field_name(self.0))
//    }}
}
