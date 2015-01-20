#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_uint;

use cass_types::cass_bool_t;
use cass_types::cass_double_t;
use cass_types::cass_float_t;
use cass_types::cass_size_t;
use cass_types::cass_int32_t;
use cass_types::cass_int64_t;

use cass_error::CassError;
use cass_bytes::CassBytes;
use cass_inet::CassInet;
use cass_uuid::CassUuid;
use cass_string::CassString;
use cass_decimal::CassDecimal;


pub enum CassValue { }

pub enum CassValueType {
    CASS_VALUE_TYPE_UNKNOWN = 65535is,
    CASS_VALUE_TYPE_CUSTOM = 0,
    CASS_VALUE_TYPE_ASCII = 1,
    CASS_VALUE_TYPE_BIGINT = 2,
    CASS_VALUE_TYPE_BLOB = 3,
    CASS_VALUE_TYPE_BOOLEAN = 4,
    CASS_VALUE_TYPE_COUNTER = 5,
    CASS_VALUE_TYPE_DECIMAL = 6,
    CASS_VALUE_TYPE_DOUBLE = 7,
    CASS_VALUE_TYPE_FLOAT = 8,
    CASS_VALUE_TYPE_INT = 9,
    CASS_VALUE_TYPE_TEXT = 10,
    CASS_VALUE_TYPE_TIMESTAMP = 11,
    CASS_VALUE_TYPE_UUID = 12,
    CASS_VALUE_TYPE_VARCHAR = 13,
    CASS_VALUE_TYPE_VARINT = 14,
    CASS_VALUE_TYPE_TIMEUUID = 15,
    CASS_VALUE_TYPE_INET = 16,
    CASS_VALUE_TYPE_LIST = 32,
    CASS_VALUE_TYPE_MAP = 33,
    CASS_VALUE_TYPE_SET = 34,
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
