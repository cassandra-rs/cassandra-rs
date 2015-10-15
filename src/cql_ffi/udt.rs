use std::ffi::CString;
use std::mem;
use std::slice;
use std::str;

use cql_ffi::value::Value;
use cql_ffi::uuid::Uuid;
use cql_ffi::inet::Inet;
use cql_ffi::collection::set::Set;
use cql_ffi::tuple::Tuple;

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

use cql_bindgen::CassDataType as _DataType;
use cql_bindgen::CassUserType as _UserType;

use cql_ffi::value::ValueType;
use cql_ffi::error::CassandraError;

pub struct DataType(pub *mut _DataType);
pub struct ConstDataType(pub *const _DataType);

pub struct UserType(pub *mut _UserType);

impl DataType {
    pub fn new(value_type: ValueType) -> Self {
        unsafe {
            DataType(cass_data_type_new(value_type as u32))
        }
    }

    pub fn new_from_existing(data_type: DataType) -> Self {
        unsafe {
            DataType(cass_data_type_new_from_existing(data_type.0))
        }
    }

    pub fn new_tuple(item_count: u64) -> DataType {
        unsafe {
            DataType(cass_data_type_new_tuple(item_count))
        }
    }

    pub fn new_udt(field_count: u64) -> DataType {
        unsafe {
            DataType(cass_data_type_new_udt(field_count))
        }
    }

    pub fn get_type(data_type: DataType) -> ValueType {
        unsafe {
            ValueType::build(cass_data_type_type(data_type.0))
        }
    }

    pub fn type_name<S>(data_type: DataType, type_name: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let type_name = CString::new(type_name.into()).unwrap();
            CassandraError::build(
                cass_data_type_type_name(
                    data_type.0,
                    &mut type_name.as_ptr(),
                    &mut(type_name.as_bytes().len() as u64)
                )
            ).wrap(())
        }
    }

    pub fn set_type_name<S>(data_type: DataType, type_name: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let type_name = CString::new(type_name.into()).unwrap();
            CassandraError::build(
                cass_data_type_set_type_name(
                    data_type.0,
                    type_name.as_ptr())
                )
            .wrap(())
        }
    }

    pub fn set_type_name_n<S>(data_type: DataType, type_name: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let type_name = CString::new(type_name.into()).unwrap();
            CassandraError::build(
                cass_data_type_set_type_name_n(
                    data_type.0,
                    type_name.as_ptr(),
                    type_name.as_bytes().len() as u64
                )
            ).wrap(())
        }
    }

    pub fn keyspace<S>(data_type: DataType, keyspace: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let keyspace = CString::new(keyspace.into()).unwrap();
            CassandraError::build(
                cass_data_type_keyspace(
                    data_type.0,
                    &mut(keyspace.as_ptr()),
                    &mut(keyspace.as_bytes().len() as u64)
                )
            ).wrap(())
        }
    }

    pub fn set_keyspace<S>(data_type: DataType, keyspace: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let keyspace = CString::new(keyspace.into()).unwrap();
            CassandraError::build(
                cass_data_type_set_keyspace(
                    data_type.0,
                    keyspace.as_ptr()
                )
            ).wrap(())
        }
    }

    pub fn set_keyspace_n<S>(data_type: DataType, keyspace: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let keyspace = CString::new(keyspace.into()).unwrap();
            CassandraError::build(
                cass_data_type_set_keyspace_n(
                    data_type.0,
                    keyspace.as_ptr(),
                    keyspace.as_bytes().len() as u64
                )
            ).wrap(())
        }
    }

    pub fn class_name<S>(data_type: DataType, class_name: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let class_name = CString::new(class_name.into()).unwrap();
            CassandraError::build(
                cass_data_type_class_name(
                    data_type.0,
                    &mut class_name.as_ptr(),
                    &mut(class_name.as_bytes().len() as u64)
                )
            ).wrap(())
        }
    }


    pub fn set_class_name<S>(data_type: DataType, class_name: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let class_name = CString::new(class_name.into()).unwrap();
            CassandraError::build(
                cass_data_type_set_class_name(
                    data_type.0,
                    class_name.as_ptr()
                )
            ).wrap(())
        }
    }

    pub fn set_class_name_n<S>(data_type: DataType, class_name: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let class_name = CString::new(class_name.into()).unwrap();
            CassandraError::build(
                cass_data_type_set_class_name_n(
                    data_type.0,
                    class_name.as_ptr(),
                    class_name.as_bytes().len() as u64
                )
            ).wrap(())
        }
    }

    pub fn sub_data_type(data_type: DataType, index: u64) -> ConstDataType {
        unsafe {
            ConstDataType(cass_data_type_sub_data_type(data_type.0, index))
        }
    }

    pub fn sub_data_type_by_name<S>(data_type: DataType, name: S) -> ConstDataType
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            ConstDataType(cass_data_type_sub_data_type_by_name(data_type.0, name.as_ptr()))
        }
    }

    pub fn sub_data_type_by_name_n<S>(data_type: DataType, name: S) -> ConstDataType
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            ConstDataType(cass_data_type_sub_data_type_by_name_n(data_type.0,
                                                                     name.as_ptr(),
                                                                     name.as_bytes().len() as u64))
        }
    }


    pub fn sub_type_name<S>(data_type: DataType, index: u64, name: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassandraError::build(
                cass_data_type_sub_type_name(
                    data_type.0,
                    index,
                    &mut name.as_ptr(),
                    &mut(name.as_bytes().len() as u64)
                )
            ).wrap(())
        }
    }

    pub fn add_sub_type(data_type: DataType,
                        sub_data_type: DataType)
                        -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(
                cass_data_type_add_sub_type(data_type.0, sub_data_type.0)
            ).wrap(())
        }
    }

    pub fn add_sub_type_by_name<S>(data_type: DataType,
                                   name: S,
                                   sub_data_type: DataType)
                                   -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassandraError::build(
                cass_data_type_add_sub_type_by_name(
                    data_type.0,
                    name.as_ptr(),
                    sub_data_type.0
                )
            ).wrap(())
        }
    }

    pub fn add_sub_value_type<S>(data_type: DataType,
                                 sub_value_type: ValueType)
                                 -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            CassandraError::build(
                cass_data_type_add_sub_value_type(
                    data_type.0,
                    sub_value_type as u32
                )
            ).wrap(())
        }
    }
}

//impl Drop for DataType {
//    fn drop(&mut self) {unsafe{
//        cass_data_type_free(&mut self.0)
//    }}
//}

//impl Drop for UserType {
//    fn drop(&mut self) {unsafe{
//        cass_user_type_free(self.0)
//    }}
//}

impl UserType {
    pub fn new(data_type: ConstDataType) -> Self {
        unsafe {
            UserType(cass_user_type_new_from_data_type(data_type.0))
        }
    }

    pub fn data_type(&self) -> ConstDataType {
        unsafe {
            ConstDataType(cass_user_type_data_type(self.0))
        }
    }

    pub fn set_null(&mut self, index: u64) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(
                cass_user_type_set_null(
                    self.0,
                    index
                )
            ).wrap(())
        }
    }

    pub fn set_int32(&mut self, index: u64, value: i32) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(
                cass_user_type_set_int32(
                    self.0,
                    index,
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_int32_by_name<S>(&mut self, name: S, value: i32) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    let rc = cass_user_type_set_int32_by_name(self.0, name.as_ptr(), value);
                    CassandraError::build(rc).wrap(())
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }

    pub fn set_int64(&mut self, index: u64, value: i64) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(cass_user_type_set_int64(self.0, index, value)).wrap(())
        }
    }

    pub fn set_int64_by_name<S>(&mut self, name: S, value: i64) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassandraError::build(
                cass_user_type_set_int64_by_name(
                    self.0,
                    name.as_ptr(),
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_float(&mut self, index: u64, value: f32) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(
                cass_user_type_set_float(
                    self.0,
                    index,
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_float_by_name<S>(&mut self, name: S, value: f32) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassandraError::build(
                cass_user_type_set_float_by_name(
                    self.0,
                    name.as_ptr(),
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_double(&mut self, index: u64, value: f64) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(cass_user_type_set_double(self.0, index, value)).wrap(())
        }
    }

    pub fn set_double_by_name<S>(&mut self, name: S, value: f64) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassandraError::build(
                cass_user_type_set_double_by_name(
                    self.0,
                    name.as_ptr(),
                    value
                )
            ).wrap(())
        }
    }

    pub fn set_bool(&mut self, index: u64, value: bool) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(
                cass_user_type_set_bool(
                    self.0,
                    index,
                    if value {1u32} else {0u32}
                )
            ).wrap(())
        }
    }

    pub fn set_stringl<S>(&mut self, index: u64, value: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let value = CString::new(value.into()).unwrap();
            CassandraError::build(cass_user_type_set_string(self.0, index, value.as_ptr())).wrap(())
        }
    }

    pub fn set_string_by_name<S>(&mut self, name: S, value: S) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            let value = CString::new(value.into()).unwrap();
            CassandraError::build(
                cass_user_type_set_string_by_name(
                    self.0,
                    name.as_ptr(),
                    value.as_ptr()
                )
            ).wrap(())
        }
    }

    //FIXME. right way to pass the vec?
    pub fn set_bytes(&mut self, index: u64, value: Vec<u8>) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(
                cass_user_type_set_bytes(
                    self.0,
                    index,
                    value.as_ptr(),
                    value.len() as u64
                )
            ).wrap(())
        }
    }

    pub fn set_uuid<S>(&mut self, index: u64, value: S) -> Result<(), CassandraError>
        where S: Into<Uuid>
    {
        unsafe {
            CassandraError::build(cass_user_type_set_uuid(self.0, index, value.into().0)).wrap(())
        }
    }

    pub fn set_inet<S>(&mut self, index: u64, value: S) -> Result<(), CassandraError>
        where S: Into<Inet>
    {
        unsafe {
            CassandraError::build(cass_user_type_set_inet(self.0, index, value.into().0)).wrap(())
        }
    }

    pub fn set_collection<S>(&mut self, index: u64, value: S) -> Result<(), CassandraError>
        where S: Into<Set>
    {
        unsafe {
            CassandraError::build(cass_user_type_set_collection(self.0, index, value.into().0)).wrap(())
        }
    }

    pub fn set_collection_by_name<S>(&mut self, name: S, value: Set) -> Result<(), CassandraError>
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            CassandraError::build(
                cass_user_type_set_collection_by_name(
                    self.0,
                    name.as_ptr(),
                    value.0
                )
            ).wrap(())
        }
    }

    pub fn set_tuple(&mut self, index: u64, value: Tuple) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(cass_user_type_set_tuple(self.0, index, value.0)).wrap(())
        }
    }

    pub fn set_user_type(&mut self, index: u64, value: UserType) -> Result<(), CassandraError> {
        unsafe {
            CassandraError::build(cass_user_type_set_user_type(self.0, index, value.0)).wrap(())
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
    type Item = (String,Value);
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
                    Some((key, Value::new(field_value)))
                }
            }
        }
    }
}

impl UserTypeIterator {
//    pub fn get_field_name(&mut self)-> Value {unsafe{
//
//        Value::new(cass_iterator_get_user_type_field_name(self.0))
//    }}
}
