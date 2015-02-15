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

impl CassCollection {
    pub unsafe fn new(_type: CassCollectionType, item_count: cass_size_t) -> CassCollection {CassCollection(cass_collection_new(_type as u32,item_count))}
    pub unsafe fn free(collection: &mut CassCollection) {cass_collection_free(collection.0)}
    pub unsafe fn append_int32(collection: &mut CassCollection, value: i32) -> Result<(),CassError> {CassError::build(cass_collection_append_int32(collection.0,value))}
    pub unsafe fn append_int64(collection: &mut CassCollection, value: i64) -> Result<(),CassError> {CassError::build(cass_collection_append_int64(collection.0,value))}
    pub unsafe fn append_float(collection: &mut CassCollection, value: f32) -> Result<(),CassError> {CassError::build(cass_collection_append_float(collection.0,value))}
    pub unsafe fn append_double(collection: &mut CassCollection, value: f64) -> Result<(),CassError> {CassError::build(cass_collection_append_double(collection.0,value))}
    pub unsafe fn append_bool(collection: &mut CassCollection, value: bool) -> Result<(),CassError> {CassError::build(cass_collection_append_bool(collection.0,if value {1} else {0}))}
    pub unsafe fn append_string(collection: &mut CassCollection, value: CassString) -> Result<(),CassError> {CassError::build(cass_collection_append_string(collection.0,value.0))}
    pub unsafe fn append_bytes(collection: &mut CassCollection, value: CassBytes) -> Result<(),CassError> {CassError::build(cass_collection_append_bytes(collection.0,value.0))}
    pub unsafe fn append_uuid(collection: &mut CassCollection, value: CassUuid) -> Result<(),CassError> {CassError::build(cass_collection_append_uuid(collection.0,value.0))}
    pub unsafe fn append_inet(collection: &mut CassCollection, value: CassInet) -> Result<(),CassError> {CassError::build(cass_collection_append_inet(collection.0,value.0))}
    pub unsafe fn append_decimal(collection: &mut CassCollection, value: CassDecimal) -> Result<(),CassError> {CassError::build(cass_collection_append_decimal(collection.0,value.0))}
}
