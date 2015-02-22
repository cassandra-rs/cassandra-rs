#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_int;

use std::ffi::CString;

use cql_ffi::collection::CassSet;
use cql_ffi::collection::CassMap;
use cql_ffi::collection::CassList;
use cql_ffi::error::CassError;
use cql_ffi::decimal::CassDecimal;
use cql_ffi::uuid::CassUuid;
use cql_ffi::bytes::CassBytes;
use cql_ffi::string::CassString;
use cql_ffi::inet::CassInet;
use cql_ffi::result::CassResult;
use cql_ffi::consistency::CassConsistency;
use cql_ffi::string::AsCassStr;

use cql_ffi::types::cass_size_t;
use cql_ffi::types::cass_byte_t;
use cql_ffi::types::cass_double_t;
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
    unsafe fn free(&mut self) {cass_statement_free(self.0)}
    
    pub fn new(query: &str, parameter_count: cass_size_t) -> Self {unsafe{
            CassStatement(cass_statement_new(query.as_cass_str().0,parameter_count))
    }}
    
    pub fn add_key_index(&self, index: cass_size_t) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_statement_add_key_index(self.0,index)).wrap(&self)
    }}

    pub fn set_keyspace(&self, keyspace: String) -> Result<&Self,CassError> {unsafe{
        let keyspace = CString::new(keyspace).unwrap();
        CassError::build(cass_statement_set_keyspace(self.0,(keyspace.as_ptr()))).wrap(&self)
    }}

    pub fn set_consistency(&self, consistency: CassConsistency) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_statement_set_consistency(self.0,consistency.0)).wrap(&self)
    }}

    pub fn set_serial_consistency(&self, serial_consistency: CassConsistency) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_statement_set_serial_consistency(self.0,serial_consistency.0)).wrap(&self)
    }}

    pub fn set_paging_size<'a>(&'a self, page_size: c_int) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_set_paging_size(self.0,page_size)).wrap(self)
    }}

    pub fn set_paging_state<'a>(&'a self, result: &'a CassResult) -> Result<&'a Self,CassError> {unsafe{
        try!(CassError::build(cass_statement_set_paging_state(self.0,result.0)).wrap(()));
        Ok(self)
    }}

    pub fn bind_null<'a>(&'a self, index: cass_size_t) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_null(self.0,index)).wrap(&self)
    }}

    pub fn bind_int32<'a>(&'a self, index: cass_size_t, value: cass_int32_t) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_int32(self.0,index, value)).wrap(&self)
    }}

    pub fn bind_int64<'a>(&'a self, index: cass_size_t, value: cass_int64_t) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_int64(self.0,index, value)).wrap(&self)
    }}

    pub fn bind_float<'a>(&'a self, index: cass_size_t, value: cass_float_t) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_float(self.0,index, value)).wrap(&self)
    }}

    pub fn bind_double<'a>(&'a self, index: cass_size_t, value: cass_double_t) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_double(self.0,index, value)).wrap(&self)
    }}

    pub fn bind_bool<'a>(&'a self, index: cass_size_t, value: bool) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_bool(self.0,index, if value{1} else {0})).wrap(&self)
    }}

    pub fn bind_string<'a>(&'a self, index: cass_size_t, value: &str) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_string(self.0,index, value.as_cass_str().0)).wrap(&self)
    }}

    pub fn bind_bytes<'a>(&'a self, index: cass_size_t, value: CassBytes) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_bytes(self.0,index, value.0)).wrap(&self)
    }}

    pub fn bind_map(&self, index: cass_size_t, collection: CassMap)-> Result<&Self,CassError>{unsafe{
        CassError::build(cass_statement_bind_collection(self.0,index,collection.0)).wrap(&self)
    }}

    pub fn bind_set(&self, index: cass_size_t, collection: CassSet)-> Result<&Self,CassError>{unsafe{
        CassError::build(cass_statement_bind_collection(self.0,index,collection.0)).wrap(&self)
    }}

    pub fn bind_list(&self, index: cass_size_t, collection: CassList)-> Result<&Self,CassError>{unsafe{
        CassError::build(cass_statement_bind_collection(self.0,index,collection.0)).wrap(&self)
    }}
    
    pub fn bind_uuid<'a>(&'a self, index: cass_size_t, value: CassUuid) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_uuid(self.0,index, value.0)).wrap(&self)
    }}

    pub fn bind_inet<'a>(&'a self, index: cass_size_t, value: CassInet) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_inet(self.0,index, value.0)).wrap(&self)
    }}

    pub fn bind_decimal<'a>(&'a self, index: cass_size_t, value: CassDecimal) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_decimal(self.0,index, value.0)).wrap(&self)
    }}

    pub fn bind_custom<'a>(&'a self, index: cass_size_t, size: cass_size_t, output: *mut *mut cass_byte_t) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_custom(self.0,index, size, output)).wrap(&self)
    }}

    pub fn bind_int32_by_name<'a>(&'a self, name: &str, value: cass_int32_t) -> Result<&'a Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_int32_by_name(self.0,name.as_ptr(), value)).wrap(&self)
    }}

    pub fn bind_int64_by_name<'a>(&'a self, name: &str, value: cass_int64_t) -> Result<&'a Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_int64_by_name(self.0,name.as_ptr(), value)).wrap(&self)
    }}

    pub fn bind_float_by_name<'a>(&'a self, name: &str, value: cass_float_t) -> Result<&'a Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_float_by_name(self.0,name.as_ptr(), value)).wrap(&self)
    }}

    pub fn bind_double_by_name<'a>(&'a self, name: &str, value: cass_double_t) -> Result<&'a Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_double_by_name(self.0,name.as_ptr(), value)).wrap(&self)
    }}

    pub fn bind_bool_by_name<'a>(&'a self, name: &str, value: bool) -> Result<&'a Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_bool_by_name(self.0,name.as_ptr(),if value {1} else {0})).wrap(&self)
    }}

    pub fn bind_string_by_name<'a>(&'a self, name: &'a str, value: CassString)-> Result<&'a Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_string_by_name(self.0,name.as_ptr(),value.0)).wrap(&self)
    }}

    pub fn bind_bytes_by_name<'a>(&'a self, name: &str, value: CassBytes)-> Result<&'a Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_bytes_by_name(self.0,name.as_ptr(),value.0)).wrap(&self)
    }}

    pub fn bind_uuid_by_name<'a>(&'a self, name: &str, value: CassUuid) -> Result<&'a Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_uuid_by_name(self.0,name.as_ptr(),value.0)).wrap(&self)
    }}

    pub fn bind_inet_by_name<'a>(&'a self, name: &str, value: CassInet)-> Result<&'a Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_inet_by_name(self.0,name.as_ptr(),value.0)).wrap(&self)
    }}

    pub fn bind_decimal_by_name<'a>(&'a self, name: &str, value: CassDecimal)-> Result<&'a Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_decimal_by_name(self.0,name.as_ptr(), value.0)).wrap(&self)
    }}

    pub fn bind_custom_by_name<'a>(&'a self, name: &str, size: cass_size_t, output: *mut *mut cass_byte_t)-> Result<&'a Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_custom_by_name(self.0,name.as_ptr(), size, output)).wrap(&self)
    }}

    pub fn bind_set_by_name<'a>(&'a self, name: &str, collection: CassSet)-> Result<&'a Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_collection_by_name(self.0,name.as_ptr(),collection.0)).wrap(&self)
    }}
}
