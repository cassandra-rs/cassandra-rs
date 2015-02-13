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

#[derive(Copy)]
pub enum CassValue { }

#[repr(C)]
#[derive(Debug,Copy,PartialEq)]
pub enum CassValueType {
    UNKNOWN = 65535is,
    CUSTOM = 0,
    ASCII = 1,
    BIGINT = 2,
    BLOB = 3,
    BOOLEAN = 4,
    COUNTER = 5,
    DECIMAL = 6,
    DOUBLE = 7,
    FLOAT = 8,
    INT = 9,
    TEXT = 10,
    TIMESTAMP = 11,
    UUID = 12,
    VARCHAR = 13,
    VARINT = 14,
    TIMEUUID = 15,
    INET = 16,
    LIST = 32,
    MAP = 33,
    SET = 34,
}

extern "C" {
    pub fn cass_value_get_int32(value: *const CassValue, output: *mut cass_int32_t) -> CassError;
    pub fn cass_value_get_int64(value: *const CassValue, output: *mut cass_int64_t) -> CassError;
    pub fn cass_value_get_float(value: *const CassValue, output: *mut cass_float_t) -> CassError;
    pub fn cass_value_get_double(value: *const CassValue, output: *mut cass_double_t) -> CassError;
    pub fn cass_value_get_bool(value: *const CassValue, output: *mut cass_bool_t) -> CassError;
    pub fn cass_value_get_uuid(value: *const CassValue, output: *mut CassUuid) -> CassError;
    pub fn cass_value_get_inet(value: *const CassValue, output: *mut CassInet) -> CassError;
    pub fn cass_value_get_string(value: *const CassValue, output: *mut CassString) -> CassError;
    pub fn cass_value_get_bytes(value: *const CassValue, output: *mut CassBytes) -> CassError;
    pub fn cass_value_get_decimal(value: *const CassValue, output: *mut CassDecimal) -> CassError;
    pub fn cass_value_type(value: *const CassValue) -> CassValueType;
    pub fn cass_value_is_null(value: *const CassValue) -> cass_bool_t;
    pub fn cass_value_is_collection(value: *const CassValue) -> cass_bool_t;
    pub fn cass_value_item_count(collection: *const CassValue) -> cass_size_t;
    pub fn cass_value_primary_sub_type(collection: *const CassValue) -> CassValueType;
    pub fn cass_value_secondary_sub_type(collection: *const CassValue) -> CassValueType;
}
