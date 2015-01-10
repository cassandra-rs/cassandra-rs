#![allow(non_camel_case_types)]
#![allow(dead_code)]

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


enum Struct_CassValue_ { }
pub type CassValue = Struct_CassValue_;

type Enum_CassValueType_ = c_uint;
pub const CASS_VALUE_TYPE_UNKNOWN: c_uint = 65535;
pub const CASS_VALUE_TYPE_CUSTOM: c_uint = 0;
pub const CASS_VALUE_TYPE_ASCII: c_uint = 1;
pub const CASS_VALUE_TYPE_BIGINT: c_uint = 2;
pub const CASS_VALUE_TYPE_BLOB: c_uint = 3;
pub const CASS_VALUE_TYPE_BOOLEAN: c_uint = 4;
pub const CASS_VALUE_TYPE_COUNTER: c_uint = 5;
pub const CASS_VALUE_TYPE_DECIMAL: c_uint = 6;
pub const CASS_VALUE_TYPE_DOUBLE: c_uint = 7;
pub const CASS_VALUE_TYPE_FLOAT: c_uint = 8;
pub const CASS_VALUE_TYPE_INT: c_uint = 9;
pub const CASS_VALUE_TYPE_TEXT: c_uint = 10;
pub const CASS_VALUE_TYPE_TIMESTAMP: c_uint = 11;
pub const CASS_VALUE_TYPE_UUID: c_uint = 12;
pub const CASS_VALUE_TYPE_VARCHAR: c_uint = 13;
pub const CASS_VALUE_TYPE_VARINT: c_uint = 14;
pub const CASS_VALUE_TYPE_TIMEUUID: c_uint = 15;
pub const CASS_VALUE_TYPE_INET: c_uint = 16;
pub const CASS_VALUE_TYPE_LIST: c_uint = 32;
pub const CASS_VALUE_TYPE_MAP: c_uint = 33;
pub const CASS_VALUE_TYPE_SET: c_uint = 34;
pub type CassValueType = Enum_CassValueType_;

extern "C" {
    pub fn cass_value_get_int32(value: *const CassValue, output: *mut cass_int32_t) -> CassError;
    pub fn cass_value_get_int64(value: *const CassValue, output: *mut cass_int64_t) -> CassError;
    pub fn cass_value_get_float(value: *const CassValue, output: *mut cass_float_t) -> CassError;
    pub fn cass_value_get_double(value: *const CassValue, output: *mut cass_double_t) -> CassError;
    pub fn cass_value_get_bool(value: *const CassValue, output: *mut cass_bool_t) -> CassError;
    pub fn cass_value_get_uuid(value: *const CassValue, output: CassUuid) -> CassError;
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
