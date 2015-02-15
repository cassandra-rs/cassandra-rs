#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::types::cass_bool_t;
use cql_ffi::types::cass_double_t;
use cql_ffi::types::cass_float_t;
use cql_ffi::types::cass_size_t;
use cql_ffi::types::cass_int32_t;
use cql_ffi::types::cass_int64_t;
use cql_ffi::error::CassError;
use cql_ffi::bytes::CassBytes;
use cql_ffi::inet::CassInet;
use cql_ffi::uuid::CassUuid;
use cql_ffi::string::CassString;
use cql_ffi::decimal::CassDecimal;
use cql_bindgen::CassValue as _CassValue;
use cql_bindgen::CassValueType as _CassValueType;
use cql_bindgen::cass_value_secondary_sub_type;
use cql_bindgen::cass_value_primary_sub_type;
use cql_bindgen::cass_value_item_count;
use cql_bindgen::cass_value_is_collection;
use cql_bindgen::cass_value_is_null;
use cql_bindgen::cass_value_type;
use cql_bindgen::cass_value_get_decimal;
use cql_bindgen::cass_value_get_inet;
use cql_bindgen::cass_value_get_string;
use cql_bindgen::cass_value_get_bytes;
use cql_bindgen::cass_value_get_uuid;
use cql_bindgen::cass_value_get_bool;
use cql_bindgen::cass_value_get_double;
use cql_bindgen::cass_value_get_float;
use cql_bindgen::cass_value_get_int64;
use cql_bindgen::cass_value_get_int32;


pub struct CassValue(pub *const _CassValue);
pub struct CassValueType(pub _CassValueType);

//~ #[repr(C)]
//~ #[derive(Debug,Copy,PartialEq)]
//~ pub enum CassValueType {
    //~ UNKNOWN = 65535is,
    //~ CUSTOM = 0,
    //~ ASCII = 1,
    //~ BIGINT = 2,
    //~ BLOB = 3,
    //~ BOOLEAN = 4,
    //~ COUNTER = 5,
    //~ DECIMAL = 6,
    //~ DOUBLE = 7,
    //~ FLOAT = 8,
    //~ INT = 9,
    //~ TEXT = 10,
    //~ TIMESTAMP = 11,
    //~ UUID = 12,
    //~ VARCHAR = 13,
    //~ VARINT = 14,
    //~ TIMEUUID = 15,
    //~ INET = 16,
    //~ LIST = 32,
    //~ MAP = 33,
    //~ SET = 34,
//~ }


impl CassValue {
    pub unsafe fn get_int32(&self, output: *mut cass_int32_t) -> Result<(),CassError> {CassError::build(cass_value_get_int32(self.0,output))}
    pub unsafe fn get_int64(&self, output: *mut cass_int64_t) -> Result<(),CassError> {CassError::build(cass_value_get_int64(self.0,output))}
    pub unsafe fn get_float(&self, output: *mut cass_float_t) -> Result<(),CassError> {CassError::build(cass_value_get_float(self.0,output))}
    pub unsafe fn get_double(&self, output: *mut cass_double_t) -> Result<(),CassError> {CassError::build(cass_value_get_double(self.0,output))}
    pub unsafe fn get_bool(&self, output: *mut cass_bool_t) -> Result<(),CassError> {CassError::build(cass_value_get_bool(self.0,output))}
    pub unsafe fn get_uuid(&self, output: &mut CassUuid) -> Result<(),CassError> {CassError::build(cass_value_get_uuid(self.0,&mut output.0))}
    pub unsafe fn get_inet(&self, mut output: CassInet) -> Result<(),CassError> {CassError::build(cass_value_get_inet(self.0,&mut output.0))}
    pub unsafe fn get_string(&self, mut output: CassString) -> Result<(),CassError> {CassError::build(cass_value_get_string(self.0,&mut output.0))}
    pub unsafe fn get_bytes(&self, mut output: CassBytes) -> Result<(),CassError> {CassError::build(cass_value_get_bytes(self.0,&mut output.0))}
    pub unsafe fn get_decimal(&self, mut output: CassDecimal) -> Result<(),CassError> {CassError::build(cass_value_get_decimal(self.0,&mut output.0))}
    pub unsafe fn get_type(&self) -> CassValueType {CassValueType(cass_value_type(self.0))}
    pub unsafe fn is_null(&self) -> bool {if cass_value_is_null(self.0) > 0 {true} else {false}}
    pub unsafe fn is_collection(&self) -> bool {if cass_value_is_collection(self.0) > 0 {true} else {false}}
    pub unsafe fn item_count(&self) -> cass_size_t {cass_value_item_count(self.0)}
    pub unsafe fn primary_sub_type(&self) -> CassValueType {CassValueType(cass_value_primary_sub_type(self.0))}
    pub unsafe fn secondary_sub_type(&self) -> CassValueType {CassValueType(cass_value_secondary_sub_type(self.0))}
}
