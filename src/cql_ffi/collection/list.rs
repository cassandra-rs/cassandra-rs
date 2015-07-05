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
//use cql_bindgen::cass_collection_append_decimal;

use cql_ffi::value::CassValue;
use cql_ffi::collection::collection::CassCollectionType;
use cql_ffi::error::CassError;
use cql_ffi::uuid::CassUuid;
use cql_ffi::inet::CassInet;

pub struct CassList(pub *mut _CassCollection);


impl Drop for CassList {
    fn drop(&mut self) {unsafe{
        cass_collection_free(self.0)
    }}
}

impl CassList {
    pub fn new(item_count: u64) -> CassList {unsafe{
        CassList(cass_collection_new(CassCollectionType::LIST as u32,item_count))
    }}

    pub fn append_int32(&mut self, value: i32) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_collection_append_int32(self.0,value)).wrap(self)
    }}

    pub fn append_int64(&mut self, value: i64) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_collection_append_int64(self.0,value)).wrap(self)
    }}

    pub fn append_float(&mut self, value: f32) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_collection_append_float(self.0,value)).wrap(self)
    }}

    pub fn append_double(&mut self, value: f64) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_collection_append_double(self.0,value)).wrap(self)
    }}

    pub fn append_bool(&mut self, value: bool) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_collection_append_bool(self.0,if value {1} else {0})).wrap(self)
    }}

    pub fn append_string(&mut self, value: &str) -> Result<&Self,CassError> {unsafe{
        let cstr = CString::new(value).unwrap();
        let result = cass_collection_append_string(self.0,cstr.as_ptr());
        CassError::build(result).wrap(self)
    }}

    pub fn append_bytes(&mut self, value: Vec<u8>) -> Result<&Self,CassError> {unsafe{
        let bytes = cass_collection_append_bytes(self.0,value[..].as_ptr(), value.len() as u64);
        CassError::build(bytes).wrap(self)
    }}

    pub fn append_uuid(&mut self, value: CassUuid) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_collection_append_uuid(self.0,value.0)).wrap(self)
    }}

    pub fn append_inet(&mut self, value: CassInet) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_collection_append_inet(self.0,value.0)).wrap(self)
    }}

//FIXME rust doesn't have good decimal support yet
//    pub fn append_decimal<'a>(&'a mut self, value: String) -> Result<&'a Self,CassError> {unsafe{
//        CassError::build(cass_collection_append_decimal(self.0,value)).wrap(self)
//    }}
    
    }

pub struct ListIterator(pub *mut _CassIterator);

impl Drop for ListIterator {
    fn drop(&mut self) {unsafe{
        cass_iterator_free(self.0)
    }}
}

impl Iterator for ListIterator {
    type Item = CassValue;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {unsafe{
        match cass_iterator_next(self.0) {
            0 => None,
            _ => Some(self.get_value())
        }    
    }}
}

impl ListIterator {
    pub fn get_value(&mut self)-> CassValue {unsafe{
        CassValue::new(cass_iterator_get_value(self.0))
    }}
}
