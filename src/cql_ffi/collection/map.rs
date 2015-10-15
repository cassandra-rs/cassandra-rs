use std::ffi::CString;
use std::iter::Iterator;

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
use cql_bindgen::CassIterator as _CassIterator;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_map_value;
use cql_bindgen::cass_iterator_get_map_key;

//use cql_bindgen::cass_collection_append_decimal;

use cql_ffi::value::Value;
use cql_ffi::error::CassandraError;
use cql_ffi::uuid::Uuid;
use cql_ffi::inet::Inet;
use cql_ffi::collection::collection::CassCollectionType;

pub struct Map(pub *mut _CassCollection);


impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            cass_collection_free(self.0)
        }
    }
}

impl Map {
    pub fn new(item_count: u64) -> Map {
        unsafe {
            Map(cass_collection_new(CassCollectionType::MAP as u32, item_count))
        }
    }

    pub fn append_int32(&mut self, value: i32) -> Result<&Self, CassandraError> {
        unsafe {
            CassandraError::build(cass_collection_append_int32(self.0,value)).wrap(self)
        }
    }

    pub fn append_int64(&mut self, value: i64) -> Result<&Self, CassandraError> {
        unsafe {
            CassandraError::build(cass_collection_append_int64(self.0,value)).wrap(self)
        }
    }

    pub fn append_float(&mut self, value: f32) -> Result<&Self, CassandraError> {
        unsafe {
            CassandraError::build(cass_collection_append_float(self.0,value)).wrap(self)
        }
    }

    pub fn append_double(&mut self, value: f64) -> Result<&Self, CassandraError> {
        unsafe {
            CassandraError::build(cass_collection_append_double(self.0,value)).wrap(self)
        }
    }

    pub fn append_bool(&mut self, value: bool) -> Result<&Self, CassandraError> {
        unsafe {
            CassandraError::build(cass_collection_append_bool(self.0,if value {1} else {0})).wrap(self)
        }
    }

    pub fn append_string(&mut self, value: &str) -> Result<&Self, CassandraError> {
        unsafe {
            let str = CString::new(value).unwrap();
            CassandraError::build(cass_collection_append_string(self.0,str.as_ptr())).wrap(self)
        }
    }

    pub fn append_bytes(&mut self, value: Vec<u8>) -> Result<&Self, CassandraError> {
        unsafe {
            let bytes = cass_collection_append_bytes(self.0, value.as_ptr(), value.len() as u64);
            CassandraError::build(bytes).wrap(self)
        }
    }

    pub fn append_uuid(&mut self, value: Uuid) -> Result<&Self, CassandraError> {
        unsafe {
            CassandraError::build(cass_collection_append_uuid(self.0,value.0)).wrap(self)
        }
    }

    pub fn append_inet(&mut self, value: Inet) -> Result<&Self, CassandraError> {
        unsafe {
            CassandraError::build(cass_collection_append_inet(self.0,value.0)).wrap(self)
        }
    }

//    pub fn append_decimal<'a>(&'a mut self, value: String) -> Result<&'a Self,CassandraError> {unsafe{
//        CassandraError::build(cass_collection_append_decimal(self.0,value)).wrap(self)
//    }}


}

pub struct MapIterator(pub *mut _CassIterator);

impl MapIterator {
    pub fn get_key(&mut self) -> Value {
        unsafe {
            Value::new(cass_iterator_get_map_key(self.0))
        }
    }
    pub fn get_value(&mut self) -> Value {
        unsafe {
            Value::new(cass_iterator_get_map_value(self.0))
        }
    }

    pub fn get_pair(&mut self) -> Result<(Value, Value), CassandraError> {
        Ok((self.get_key(), self.get_value()))
    }

}


impl Drop for MapIterator {
    fn drop(&mut self) {
        unsafe {
            cass_iterator_free(self.0)
        }
    }
}

impl Iterator for MapIterator {
    type Item = (Value,Value);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(self.get_pair().unwrap()),
            }
        }
    }
}
