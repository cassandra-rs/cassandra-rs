#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;

use std::str::FromStr;

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
use cql_bindgen::CassStatement as _CassStatement;
use cql_bindgen::cass_statement_new;
use cql_bindgen::cass_statement_free;
use cql_bindgen::cass_statement_add_key_index;
use cql_bindgen::cass_statement_set_keyspace;
use cql_bindgen::cass_statement_set_consistency;
use cql_bindgen::cass_statement_set_serial_consistency;
use cql_bindgen::cass_statement_set_paging_size;
use cql_bindgen::cass_statement_set_paging_state;
use cql_bindgen::cass_statement_bind_null;
use cql_bindgen::cass_statement_bind_int32;
use cql_bindgen::cass_statement_bind_int64;
use cql_bindgen::cass_statement_bind_float;
use cql_bindgen::cass_statement_bind_double;
use cql_bindgen::cass_statement_bind_bool;
use cql_bindgen::cass_statement_bind_string;
use cql_bindgen::cass_statement_bind_bytes;
use cql_bindgen::cass_statement_bind_custom;
use cql_bindgen::cass_statement_bind_collection;
use cql_bindgen::cass_statement_bind_decimal;
use cql_bindgen::cass_statement_bind_inet;
use cql_bindgen::cass_statement_bind_uuid;

use cql_bindgen::cass_statement_bind_int32_by_name;
use cql_bindgen::cass_statement_bind_int64_by_name;
use cql_bindgen::cass_statement_bind_float_by_name;
use cql_bindgen::cass_statement_bind_double_by_name;
use cql_bindgen::cass_statement_bind_bool_by_name;
use cql_bindgen::cass_statement_bind_string_by_name;
use cql_bindgen::cass_statement_bind_bytes_by_name;
use cql_bindgen::cass_statement_bind_custom_by_name;
use cql_bindgen::cass_statement_bind_collection_by_name;
use cql_bindgen::cass_statement_bind_decimal_by_name;
use cql_bindgen::cass_statement_bind_inet_by_name;
use cql_bindgen::cass_statement_bind_uuid_by_name;


#[derive(Debug,Eq,PartialEq)]
#[allow(raw_pointer_derive)]
pub struct CassStatement(pub *mut _CassStatement);

impl Drop for CassStatement {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl CassStatement {
    pub unsafe fn new(query: &str, parameter_count: cass_size_t) -> CassStatement {
        let query:CassString = FromStr::from_str(query).unwrap();
        CassStatement(cass_statement_new(query.0,parameter_count))
    }
    unsafe fn free(&self) {cass_statement_free(self.0)}
    pub unsafe fn add_key_index<'a>(&'a self, index: cass_size_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_add_key_index(self.0,index)).wrap(self)}
    pub unsafe fn set_keyspace<'a>(&'a self, keyspace: *const c_char) -> Result<&'a Self,CassError> {CassError::build(cass_statement_set_keyspace(self.0,keyspace)).wrap(self)}
    pub unsafe fn set_consistency<'a>(&'a self, consistency: CassConsistency) -> Result<&'a Self,CassError> {CassError::build(cass_statement_set_consistency(self.0,consistency.0)).wrap(self)}
    pub unsafe fn set_serial_consistency<'a>(&'a self, serial_consistency: CassConsistency) -> Result<&'a Self,CassError> {CassError::build(cass_statement_set_serial_consistency(self.0,serial_consistency.0)).wrap(self)}
    pub unsafe fn set_paging_size<'a>(&'a self, page_size: c_int) -> Result<&'a Self,CassError> {CassError::build(cass_statement_set_paging_size(self.0,page_size)).wrap(self)}
    pub unsafe fn set_paging_state<'a>(&'a self, result: &CassResult) -> Result<&'a Self,CassError> {CassError::build(cass_statement_set_paging_state(self.0,result.0)).wrap(self)}
    pub unsafe fn bind_null<'a>(&'a self, index: cass_size_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_null(self.0,index)).wrap(self)}
    pub unsafe fn bind_int32<'a>(&'a self, index: cass_size_t, value: cass_int32_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_int32(self.0,index, value)).wrap(self)}
    pub unsafe fn bind_int64<'a>(&'a self, index: cass_size_t, value: cass_int64_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_int64(self.0,index, value)).wrap(self)}
    pub unsafe fn bind_float<'a>(&'a self, index: cass_size_t, value: cass_float_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_float(self.0,index, value)).wrap(self)}
    pub unsafe fn bind_double<'a>(&'a self, index: cass_size_t, value: cass_double_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_double(self.0,index, value)).wrap(self)}
    pub unsafe fn bind_bool<'a>(&'a self, index: cass_size_t, value: cass_bool_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_bool(self.0,index, value)).wrap(self)}
    pub unsafe fn bind_string<'a>(&'a self, index: cass_size_t, value: CassString) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_string(self.0,index, value.0)).wrap(self)}
    pub unsafe fn bind_bytes<'a>(&'a self, index: cass_size_t, value: CassBytes) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_bytes(self.0,index, value.0)).wrap(self)}
    pub unsafe fn bind_uuid<'a>(&'a self, index: cass_size_t, value: CassUuid) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_uuid(self.0,index, value.0)).wrap(self)}
    pub unsafe fn bind_inet<'a>(&'a self, index: cass_size_t, value: CassInet) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_inet(self.0,index, value.0)).wrap(self)}
    pub unsafe fn bind_decimal<'a>(&'a self, index: cass_size_t, value: CassDecimal) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_decimal(self.0,index, value.0)).wrap(self)}
    pub unsafe fn bind_custom<'a>(&'a self, index: cass_size_t, size: cass_size_t, output: *mut *mut cass_byte_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_custom(self.0,index, size, output)).wrap(self)}
    pub unsafe fn bind_collection<'a>(&'a self, index: cass_size_t, collection: &CassCollection) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_collection(self.0,index, collection.0)).wrap(self)}
    pub unsafe fn bind_int32_by_name<'a>(&'a self, name: *const c_char, value: cass_int32_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_int32_by_name(self.0,name, value)).wrap(self)}
    pub unsafe fn bind_int64_by_name<'a>(&'a self, name: *const c_char, value: cass_int64_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_int64_by_name(self.0,name, value)).wrap(self)}
    pub unsafe fn bind_float_by_name<'a>(&'a self, name: *const c_char, value: cass_float_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_float_by_name(self.0,name, value)).wrap(self)}
    pub unsafe fn bind_double_by_name<'a>(&'a self, name: *const c_char, value: cass_double_t) -> Result<&'a Self,CassError> {CassError::build(cass_statement_bind_double_by_name(self.0,name, value)).wrap(self)}
    pub unsafe fn bind_bool_by_name<'a>(&'a self, name: *const c_char, value: cass_bool_t) -> Result<&'a Self,CassError>{CassError::build(cass_statement_bind_bool_by_name(self.0,name,value)).wrap(self)}
    pub unsafe fn bind_string_by_name<'a>(&'a self, name: *const c_char, value: CassString)-> Result<&'a Self,CassError>{CassError::build(cass_statement_bind_string_by_name(self.0,name,value.0)).wrap(self)}
    pub unsafe fn bind_bytes_by_name<'a>(&'a self, name: *const c_char, value: CassBytes)-> Result<&'a Self,CassError>{CassError::build(cass_statement_bind_bytes_by_name(self.0,name,value.0)).wrap(self)}
    pub unsafe fn bind_uuid_by_name<'a>(&'a self, name: *const c_char, value: CassUuid) -> Result<&'a Self,CassError>{CassError::build(cass_statement_bind_uuid_by_name(self.0,name,value.0)).wrap(self)}
    pub unsafe fn bind_inet_by_name<'a>(&'a self, name: *const c_char, value: CassInet)-> Result<&'a Self,CassError>{CassError::build(cass_statement_bind_inet_by_name(self.0,name,value.0)).wrap(self)}
    pub unsafe fn bind_decimal_by_name<'a>(&'a self, name: *const c_char, value: CassDecimal)-> Result<&'a Self,CassError>{CassError::build(cass_statement_bind_decimal_by_name(self.0,name, value.0)).wrap(self)}
    pub unsafe fn bind_custom_by_name<'a>(&'a self, name: *const c_char, size: cass_size_t, output: *mut *mut cass_byte_t)-> Result<&'a Self,CassError>{CassError::build(cass_statement_bind_custom_by_name(self.0,name, size, output)).wrap(self)}
    pub unsafe fn bind_collection_by_name<'a>(&'a self, name: *const c_char, collection: CassCollection)-> Result<&'a Self,CassError>{CassError::build(cass_statement_bind_collection_by_name(self.0,name,collection.0)).wrap(self)}
}
