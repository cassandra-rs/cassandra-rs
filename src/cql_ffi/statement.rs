#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_int;

use std::ffi::CString;

use cql_ffi::collection::cass_set::CassSet;
use cql_ffi::collection::cass_map::CassMap;
use cql_ffi::collection::cass_list::CassList;
use cql_ffi::error::CassError;
use cql_ffi::uuid::CassUuid;
use cql_ffi::inet::CassInet;
use cql_ffi::result::CassResult;
use cql_ffi::consistency::CassConsistency;

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
//use cql_bindgen::cass_statement_bind_decimal;
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
//use cql_bindgen::cass_statement_bind_decimal_by_name;
use cql_bindgen::cass_statement_bind_inet_by_name;
use cql_bindgen::cass_statement_bind_uuid_by_name;


#[derive(Debug,Eq,PartialEq,Clone)]
#[allow(raw_pointer_derive)]
pub struct CassStatement(pub *mut _CassStatement);

impl Drop for CassStatement {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

pub enum CassBindable {
    
}

impl CassStatement {
    unsafe fn free(&mut self) {cass_statement_free(self.0)}
    
    pub fn bind(&mut self, params:Vec<CassBindable>) {
        
    }
    
    pub fn new(query: &str, parameter_count: cass_size_t) -> Self {unsafe{
        let query = CString::new(query).unwrap();
        CassStatement(cass_statement_new(query.as_ptr(),parameter_count))
    }}
    
    pub fn add_key_index(&mut self, index: cass_size_t) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_statement_add_key_index(self.0,index)).wrap(self)
    }}

    pub fn set_keyspace(&mut self, keyspace: String) -> Result<&Self,CassError> {unsafe{
        let keyspace = CString::new(keyspace).unwrap();
        CassError::build(cass_statement_set_keyspace(self.0,(keyspace.as_ptr()))).wrap(self)
    }}

    pub fn set_consistency(&mut self, consistency: CassConsistency) -> Result<&Self,CassError> {unsafe{
        CassError::build(cass_statement_set_consistency(self.0,consistency.0)).wrap(self)
    }}

    pub fn set_serial_consistency(&mut self, serial_consistency: CassConsistency) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_set_serial_consistency(self.0,serial_consistency.0)).wrap(self)
    }}

    pub fn set_paging_size(&mut self, page_size: c_int) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_set_paging_size(self.0,page_size)).wrap(self)
    }}

    pub fn set_paging_state(&mut self, result: &CassResult) -> Result<&mut Self,CassError> {unsafe{
        try!(CassError::build(cass_statement_set_paging_state(self.0,result.0)).wrap(()));
        Ok(self)
    }}

    pub fn bind_null(&mut self, index: cass_size_t) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_null(self.0,index)).wrap(self)
    }}

    pub fn bind_int32(&mut self, index: cass_size_t, value: cass_int32_t) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_int32(self.0,index, value)).wrap(self)
    }}

    pub fn bind_int64(&mut self, index: cass_size_t, value: cass_int64_t) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_int64(self.0,index, value)).wrap(self)
    }}

    pub fn bind_float(&mut self, index: cass_size_t, value: cass_float_t) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_float(self.0,index, value)).wrap(self)
    }}

    pub fn bind_double(&mut self, index: cass_size_t, value: cass_double_t) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_double(self.0,index, value)).wrap(self)
    }}

    pub fn bind_bool(&mut self, index: cass_size_t, value: bool) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_bool(self.0,index, if value{1} else {0})).wrap(self)
    }}

    pub fn bind_string(&mut self, index: cass_size_t, value: &str) -> Result<&mut Self,CassError> {unsafe{
        let value = CString::new(value).unwrap();
        CassError::build(cass_statement_bind_string(self.0,index, value.as_ptr())).wrap(self)
    }}

    pub fn bind_bytes(&mut self, index: cass_size_t, value: Vec<u8>) -> Result<&mut Self,CassError> {unsafe{
        let bytes = cass_statement_bind_bytes(self.0,index, value.as_ptr(), value.len() as u64);
        CassError::build(bytes).wrap(self)
    }}

    pub fn bind_map(&mut self, index: cass_size_t, collection: CassMap)-> Result<&mut Self,CassError>{unsafe{
        CassError::build(cass_statement_bind_collection(self.0,index,collection.0)).wrap(self)
    }}

    pub fn bind_set(&mut self, index: cass_size_t, collection: CassSet)-> Result<&mut Self,CassError>{unsafe{
        CassError::build(cass_statement_bind_collection(self.0,index,collection.0)).wrap(self)
    }}

    pub fn bind_list(&mut self, index: cass_size_t, collection: CassList)-> Result<&mut Self,CassError>{unsafe{
        CassError::build(cass_statement_bind_collection(self.0,index,collection.0)).wrap(self)
    }}
    
    pub fn bind_uuid(&mut self, index: cass_size_t, value: CassUuid) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_uuid(self.0,index, value.0)).wrap(self)
    }}

    pub fn bind_inet(&mut self, index: cass_size_t, value: CassInet) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_inet(self.0,index, value.0)).wrap(self)
    }}

//    pub fn bind_decimal<'a>(&'a self, index: cass_size_t, value: String) -> Result<&'a Self,CassError> {unsafe{
//        CassError::build(cass_statement_bind_decimal(self.0,index, value)).wrap(&self)
//    }}

    pub fn bind_custom(&mut self, index: cass_size_t, size: cass_size_t, output: *mut *mut cass_byte_t) -> Result<&mut Self,CassError> {unsafe{
        CassError::build(cass_statement_bind_custom(self.0,index, size, output)).wrap(self)
    }}

    pub fn bind_int32_by_name(&mut self, name: &str, value: cass_int32_t) -> Result<&mut Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_int32_by_name(self.0,name.as_ptr(), value)).wrap(self)
    }}

    pub fn bind_int64_by_name(&mut self, name: &str, value: cass_int64_t) -> Result<&mut Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_int64_by_name(self.0,name.as_ptr(), value)).wrap(self)
    }}

    pub fn bind_float_by_name(&mut self, name: &str, value: cass_float_t) -> Result<&mut Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_float_by_name(self.0,name.as_ptr(), value)).wrap(self)
    }}

    pub fn bind_double_by_name(&mut self, name: &str, value: cass_double_t) -> Result<&mut Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_double_by_name(self.0,name.as_ptr(), value)).wrap(self)
    }}

    pub fn bind_bool_by_name(&mut self, name: &str, value: bool) -> Result<&mut Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_bool_by_name(self.0,name.as_ptr(),if value {1} else {0})).wrap(self)
    }}

    pub fn bind_string_by_name(&mut self, name: &str, value: &str)-> Result<&mut Self,CassError> {unsafe{
        let name = CString::new(name).unwrap();
        let value = CString::new(value).unwrap();
        let result = cass_statement_bind_string_by_name(self.0,name.as_ptr(),value.as_ptr());
        CassError::build(result).wrap(self)
    }}

    pub fn bind_bytes_by_name(&mut self, name: &str, mut value: Vec<u8>)-> Result<&mut Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        let val = value.as_mut_ptr();
        let result = cass_statement_bind_bytes_by_name(self.0,name.as_ptr(),val, value.len() as u64);
        CassError::build(result).wrap(self)
    }}

    pub fn bind_uuid_by_name(&mut self, name: &str, value: CassUuid) -> Result<&mut Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_uuid_by_name(self.0,name.as_ptr(),value.0)).wrap(self)
    }}

    pub fn bind_inet_by_name(&mut self, name: &str, value: CassInet)-> Result<&mut Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_inet_by_name(self.0,name.as_ptr(),value.0)).wrap(self)
    }}

//    pub fn bind_decimal_by_name<'a>(&'a self, name: &str, value: String)-> Result<&'a Self,CassError>{unsafe{
//        let name = CString::new(name).unwrap();
//        CassError::build(cass_statement_bind_decimal_by_name(self.0,name.as_ptr(), value)).wrap(&self)
//    }}

    pub fn bind_custom_by_name(&mut self, name: &str, size: cass_size_t, output: *mut *mut cass_byte_t)-> Result<&mut Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_custom_by_name(self.0,name.as_ptr(), size, output)).wrap(self)
    }}

    pub fn bind_set_by_name(&mut self, name: &str, collection: CassSet)-> Result<&mut Self,CassError>{unsafe{
        let name = CString::new(name).unwrap();
        CassError::build(cass_statement_bind_collection_by_name(self.0,name.as_ptr(),collection.0)).wrap(self)
    }}
}
