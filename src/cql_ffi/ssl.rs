#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;
use std::ffi::CString;

use cql_ffi::error::CassError;
use cql_bindgen::CassSsl as _CassSsl;
use cql_bindgen::cass_ssl_new;
use cql_bindgen::cass_ssl_free;
use cql_bindgen::cass_ssl_add_trusted_cert;
use cql_bindgen::cass_ssl_set_verify_flags;
use cql_bindgen::cass_ssl_set_cert;
use cql_bindgen::cass_ssl_set_private_key;

pub struct CassSsl(pub *mut _CassSsl);

impl Drop for CassSsl {
    fn drop(&mut self) {unsafe{
        cass_ssl_free(self.0)
    }}
}

impl CassSsl {
    pub fn new() -> CassSsl {unsafe{
            CassSsl(cass_ssl_new())
    }}

    pub fn add_trusted_cert(&mut self, cert: &str) -> Result<&Self,CassError> {unsafe{
        let cert = CString::new(cert).unwrap();        
        CassError::build(cass_ssl_add_trusted_cert(self.0, cert.as_ptr())).wrap(self)
    }}

    pub fn set_verify_flags(&mut self, flags: c_int) {unsafe{
            cass_ssl_set_verify_flags(self.0,flags)
    }}        

    pub fn set_cert(&mut self, cert: &str) -> Result<&Self,CassError> {unsafe{
        let cert = CString::new(cert).unwrap();        
        CassError::build(cass_ssl_set_cert(self.0,cert.as_ptr())).wrap(self)
    }}

    pub fn set_private_key(&mut self, key: &str, password: *const c_char) -> Result<&Self,CassError> {unsafe{
        let key = CString::new(key).unwrap();        
        CassError::build(cass_ssl_set_private_key(self.0,key.as_ptr(), password)).wrap(self)
    }}
}
