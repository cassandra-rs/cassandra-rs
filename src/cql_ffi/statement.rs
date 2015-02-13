#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;

use cql_ffi::error::CassError;
use cql_ffi::collection::CassCollection;
use cql_ffi::decimal::CassDecimal;
use cql_ffi::uuid::CassUuid;
use cql_ffi::bytes::CassBytes;
use cql_ffi::string::CassString;
use cql_ffi::inet::CassInet;
use cql_ffi::result::CassResult;
use cql_ffi::consistency::CassConsistency;

use cql_ffi::types::cass_size_t;
use cql_ffi::types::cass_byte_t;
use cql_ffi::types::cass_double_t;
use cql_ffi::types::cass_bool_t;
use cql_ffi::types::cass_int32_t;
use cql_ffi::types::cass_float_t;
use cql_ffi::types::cass_int64_t;

#[derive(Debug,Eq,PartialEq,Copy)]
pub enum CassStatement { }

extern "C" {
    pub fn cass_statement_new(query: CassString, parameter_count: cass_size_t) -> *mut CassStatement;
    pub fn cass_statement_free(statement: *mut CassStatement);
    pub fn cass_statement_add_key_index(statement: *mut CassStatement, index: cass_size_t) -> CassError;
    pub fn cass_statement_set_keyspace(statement: *mut CassStatement, keyspace: *const c_char) -> CassError;
    pub fn cass_statement_set_consistency(statement: *mut CassStatement, consistency: CassConsistency) -> CassError;
    pub fn cass_statement_set_serial_consistency(statement: *mut CassStatement, serial_consistency: CassConsistency) -> CassError;
    pub fn cass_statement_set_paging_size(statement: *mut CassStatement, page_size: c_int) -> CassError;
    pub fn cass_statement_set_paging_state(statement: *mut CassStatement, result: *const CassResult) -> CassError;
    pub fn cass_statement_bind_null(statement: *mut CassStatement, index: cass_size_t) -> CassError;
    pub fn cass_statement_bind_int32(statement: *mut CassStatement, index: cass_size_t, value: cass_int32_t) -> CassError;
    pub fn cass_statement_bind_int64(statement: *mut CassStatement, index: cass_size_t, value: cass_int64_t) -> CassError;
    pub fn cass_statement_bind_float(statement: *mut CassStatement, index: cass_size_t, value: cass_float_t) -> CassError;
    pub fn cass_statement_bind_double(statement: *mut CassStatement, index: cass_size_t, value: cass_double_t) -> CassError;
    pub fn cass_statement_bind_bool(statement: *mut CassStatement, index: cass_size_t, value: cass_bool_t) -> CassError;
    pub fn cass_statement_bind_string(statement: *mut CassStatement, index: cass_size_t, value: CassString) -> CassError;
    pub fn cass_statement_bind_bytes(statement: *mut CassStatement, index: cass_size_t, value: CassBytes) -> CassError;
    pub fn cass_statement_bind_uuid(statement: *mut CassStatement, index: cass_size_t, value: CassUuid) -> CassError;
    pub fn cass_statement_bind_inet(statement: *mut CassStatement, index: cass_size_t, value: CassInet) -> CassError;
    pub fn cass_statement_bind_decimal(statement: *mut CassStatement, index: cass_size_t, value: CassDecimal) -> CassError;
    pub fn cass_statement_bind_custom(statement: *mut CassStatement, index: cass_size_t, size: cass_size_t, output: *mut *mut cass_byte_t) -> CassError;
    pub fn cass_statement_bind_collection(statement: *mut CassStatement, index: cass_size_t, collection: *const CassCollection) -> CassError;
    pub fn cass_statement_bind_int32_by_name(statement: *mut CassStatement, name: *const c_char, value: cass_int32_t) -> CassError;
    pub fn cass_statement_bind_int64_by_name(statement: *mut CassStatement, name: *const c_char, value: cass_int64_t) -> CassError;
    pub fn cass_statement_bind_float_by_name(statement: *mut CassStatement, name: *const c_char, value: cass_float_t) -> CassError;
    pub fn cass_statement_bind_double_by_name(statement: *mut CassStatement, name: *const c_char, value: cass_double_t) -> CassError;
    pub fn cass_statement_bind_bool_by_name(statement: *mut CassStatement, name: *const c_char, value: cass_bool_t) -> CassError;
    pub fn cass_statement_bind_string_by_name(statement: *mut CassStatement, name: *const c_char, value: CassString) -> CassError;
    pub fn cass_statement_bind_bytes_by_name(statement: *mut CassStatement, name: *const c_char, value: CassBytes) -> CassError;
    pub fn cass_statement_bind_uuid_by_name(statement: *mut CassStatement, name: *const c_char, value: CassUuid) -> CassError;
    pub fn cass_statement_bind_inet_by_name(statement: *mut CassStatement, name: *const c_char, value: CassInet) -> CassError;
    pub fn cass_statement_bind_decimal_by_name(statement: *mut CassStatement, name: *const c_char, value: CassDecimal) -> CassError;
    pub fn cass_statement_bind_custom_by_name(statement: *mut CassStatement, name: *const c_char, size: cass_size_t, output: *mut *mut cass_byte_t) -> CassError;
    pub fn cass_statement_bind_collection_by_name(statement: *mut CassStatement, name: *const c_char, collection: *const CassCollection)-> CassError;
}
