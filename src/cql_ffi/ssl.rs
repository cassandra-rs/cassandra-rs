#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;

use cql_ffi::error::CassError;
use cql_ffi::string::CassString;
use cql_bindgen::CassSsl as _CassSsl;
use cql_bindgen::cass_ssl_new;
use cql_bindgen::cass_ssl_free;
use cql_bindgen::cass_ssl_add_trusted_cert;
use cql_bindgen::cass_ssl_set_verify_flags;
use cql_bindgen::cass_ssl_set_cert;
use cql_bindgen::cass_ssl_set_private_key;

pub struct CassSsl(pub *mut _CassSsl);

impl CassSsl {
    pub unsafe fn new() -> CassSsl {CassSsl(cass_ssl_new())}
    pub unsafe fn free(ssl: &mut CassSsl) {cass_ssl_free(ssl.0)}
    pub unsafe fn add_trusted_cert(ssl: &mut CassSsl, cert: CassString) -> Result<(),CassError> {CassError::build(cass_ssl_add_trusted_cert(ssl.0, cert.0))}
    pub unsafe fn set_verify_flags(ssl: &mut CassSsl, flags: c_int) {cass_ssl_set_verify_flags(ssl.0,flags)}
    pub unsafe fn set_cert(ssl: &mut CassSsl, cert: CassString) -> Result<(),CassError> {CassError::build(cass_ssl_set_cert(ssl.0,cert.0))}
    pub unsafe fn set_private_key(ssl: &mut CassSsl, key: CassString, password: *const c_char) -> Result<(),CassError> {CassError::build(cass_ssl_set_private_key(ssl.0,key.0, password))}
}
