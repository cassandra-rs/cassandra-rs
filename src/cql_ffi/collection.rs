#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

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

use cql_ffi::error::CassError;
use cql_ffi::uuid::CassUuid;
use cql_ffi::bytes::CassBytes;
use cql_ffi::string::CassString;
use cql_ffi::decimal::CassDecimal;
use cql_ffi::inet::CassInet;
use cql_ffi::types::cass_size_t;

pub struct CassCollection(pub *mut _CassCollection);

#[repr(C)]
#[derive(Debug,Copy)]
pub enum CassCollectionType {
    LIST = 32is,
    MAP = 33,
    SET = 34
}

impl Drop for CassCollection {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl CassCollection {
    pub unsafe fn new(_type: CassCollectionType, item_count: cass_size_t) -> CassCollection {CassCollection(cass_collection_new(_type as u32,item_count))}
    unsafe fn free(&mut self) {cass_collection_free(self.0)}
    pub unsafe fn append_int32<'a>(&'a mut self, value: i32) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_int32(self.0,value)).wrap(self)}
    pub unsafe fn append_int64<'a>(&'a mut self, value: i64) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_int64(self.0,value)).wrap(self)}
    pub unsafe fn append_float<'a>(&'a mut self, value: f32) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_float(self.0,value)).wrap(self)}
    pub unsafe fn append_double<'a>(&'a mut self, value: f64) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_double(self.0,value)).wrap(self)}
    pub unsafe fn append_bool<'a>(&'a mut self, value: bool) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_bool(self.0,if value {1} else {0})).wrap(self)}
    pub unsafe fn append_string<'a>(&'a mut self, value: CassString) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_string(self.0,value.0)).wrap(self)}
    pub unsafe fn append_bytes<'a>(&'a mut self, value: CassBytes) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_bytes(self.0,value.0)).wrap(self)}
    pub unsafe fn append_uuid<'a>(&'a mut self, value: CassUuid) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_uuid(self.0,value.0)).wrap(self)}
    pub unsafe fn append_inet<'a>(&'a mut self, value: CassInet) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_inet(self.0,value.0)).wrap(self)}
    pub unsafe fn append_decimal<'a>(&'a mut self, value: CassDecimal) -> Result<&'a Self,CassError> {CassError::build(cass_collection_append_decimal(self.0,value.0)).wrap(self)}
}
