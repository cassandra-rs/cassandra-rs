use std::ffi::CString;

use cql_ffi::error::CassError;
use cql_bindgen::CassSsl as _Ssl;
use cql_bindgen::cass_ssl_new;
use cql_bindgen::cass_ssl_free;
use cql_bindgen::cass_ssl_add_trusted_cert;
//use cql_bindgen::cass_ssl_add_trusted_cert_n;
use cql_bindgen::cass_ssl_set_verify_flags;
use cql_bindgen::cass_ssl_set_cert;
use cql_bindgen::cass_ssl_set_private_key;

pub struct Ssl(pub *mut _Ssl);

impl Drop for Ssl {
    fn drop(&mut self) {
        unsafe {
            cass_ssl_free(self.0)
        }
    }
}

impl Ssl {
    pub fn new() -> Ssl {
        unsafe {
            Ssl(cass_ssl_new())
        }
    }

    pub fn add_trusted_cert(&mut self, cert: &str) -> Result<&Self, CassError> {
        unsafe {
            let cert = CString::new(cert).unwrap();
            CassError::build(cass_ssl_add_trusted_cert(self.0, cert.as_ptr())).wrap(self)
        }
    }

    pub fn set_verify_flags(&mut self, flags: i32) {
        unsafe {
            cass_ssl_set_verify_flags(self.0, flags)
        }
    }

    pub fn set_cert(&mut self, cert: &str) -> Result<&Self, CassError> {
        unsafe {
            let cert = CString::new(cert).unwrap();
            CassError::build(cass_ssl_set_cert(self.0,cert.as_ptr())).wrap(self)
        }
    }

    pub fn set_private_key(&mut self, key: &str, password: *const i8) -> Result<&Self, CassError> {
        unsafe {
            let key = CString::new(key).unwrap();
            CassError::build(cass_ssl_set_private_key(self.0,key.as_ptr(), password)).wrap(self)
        }
    }
}
