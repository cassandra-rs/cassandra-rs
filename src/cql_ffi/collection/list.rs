use std::ffi::CString;

use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_value;
use cql_bindgen::CassIterator as _CassIterator;
use cql_bindgen::CassCollection as _CassCollection;
use cql_bindgen::cass_collection_new;
use cql_bindgen::cass_collection_free;
use cql_bindgen::cass_collection_append_int32;
use cql_bindgen::cass_collection_append_int64;
use cql_bindgen::cass_collection_append_float;
use cql_bindgen::cass_collection_append_double;
use cql_bindgen::cass_collection_append_bool;
use cql_bindgen::cass_collection_append_bytes;
use cql_bindgen::cass_collection_append_uuid;
use cql_bindgen::cass_collection_append_string;
use cql_bindgen::cass_collection_append_inet;
use cql_bindgen::cass_collection_append_decimal;

use cql_bindgen::cass_collection_append_collection;
use cql_bindgen::cass_collection_append_int16;
use cql_bindgen::cass_collection_append_int8;
use cql_bindgen::cass_collection_append_tuple;
use cql_bindgen::cass_collection_append_uint32;
use cql_bindgen::cass_collection_append_user_type;
use cql_bindgen::cass_collection_data_type;
use cql_bindgen::cass_collection_new_from_data_type;


use cql_ffi::value::Value;
use cql_ffi::collection::collection::CassCollectionType;
use cql_ffi::error::CassandraError;
use cql_ffi::uuid::Uuid;
use cql_ffi::inet::Inet;

pub struct List(pub *mut _CassCollection);


impl Drop for List {
    fn drop(&mut self) {
        unsafe { cass_collection_free(self.0) }
    }
}

impl List {
    pub fn new(item_count: u64) -> List {
        unsafe { List(cass_collection_new(CassCollectionType::LIST as u32, item_count)) }
    }

    pub fn append_int32(&mut self, value: i32) -> Result<&Self, CassandraError> {
        unsafe { CassandraError::build(cass_collection_append_int32(self.0, value)).wrap(self) }
    }

    pub fn append_int64(&mut self, value: i64) -> Result<&Self, CassandraError> {
        unsafe { CassandraError::build(cass_collection_append_int64(self.0, value)).wrap(self) }
    }

    pub fn append_float(&mut self, value: f32) -> Result<&Self, CassandraError> {
        unsafe { CassandraError::build(cass_collection_append_float(self.0, value)).wrap(self) }
    }

    pub fn append_double(&mut self, value: f64) -> Result<&Self, CassandraError> {
        unsafe { CassandraError::build(cass_collection_append_double(self.0, value)).wrap(self) }
    }

    pub fn append_bool(&mut self, value: bool) -> Result<&Self, CassandraError> {
        unsafe { CassandraError::build(cass_collection_append_bool(self.0, if value { 1 } else { 0 })).wrap(self) }
    }

    pub fn append_string(&mut self, value: &str) -> Result<&Self, CassandraError> {
        unsafe {
            let cstr = CString::new(value).unwrap();
            let result = cass_collection_append_string(self.0, cstr.as_ptr());
            CassandraError::build(result).wrap(self)
        }
    }

    pub fn append_bytes(&mut self, value: Vec<u8>) -> Result<&Self, CassandraError> {
        unsafe {
            let bytes = cass_collection_append_bytes(self.0, value[..].as_ptr(), value.len() as u64);
            CassandraError::build(bytes).wrap(self)
        }
    }

    pub fn append_uuid(&mut self, value: Uuid) -> Result<&Self, CassandraError> {
        unsafe { CassandraError::build(cass_collection_append_uuid(self.0, value.0)).wrap(self) }
    }

    pub fn append_inet(&mut self, value: Inet) -> Result<&Self, CassandraError> {
        unsafe { CassandraError::build(cass_collection_append_inet(self.0, value.0)).wrap(self) }
    }

// FIXME rust doesn't have good decimal support yet
// pub fn append_decimal<'a>(&'a mut self, value: String) -> Result<&'a
// Self,CassandraError> {unsafe{
// CassandraError::build(cass_collection_append_decimal(self.0,value)).
// wrap(self)
//    }}

    }

pub struct ListIterator(pub *mut _CassIterator);

impl Drop for ListIterator {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl Iterator for ListIterator {
    type Item = Value;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(self.get_value()),
            }
        }
    }
}

impl ListIterator {
    pub fn get_value(&mut self) -> Value {
        unsafe { Value::new(cass_iterator_get_value(self.0)) }
    }
}
