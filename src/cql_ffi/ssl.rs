#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;

use cql_ffi::error::CassError;
use cql_ffi::string::CassString;

pub enum CassSsl { }

pub enum CassSslVerifyFlags {
    NONE = 0is,
    PEER_CERT = 1,
    PEER_IDENTITY = 2
}

extern "C" {
    pub fn cass_ssl_new() -> *mut CassSsl;
    pub fn cass_ssl_free(ssl: *mut CassSsl);
    pub fn cass_ssl_add_trusted_cert(ssl: *mut CassSsl, cert: CassString) -> CassError;
    pub fn cass_ssl_set_verify_flags(ssl: *mut CassSsl, flags: c_int);
    pub fn cass_ssl_set_cert(ssl: *mut CassSsl, cert: CassString) -> CassError;
    pub fn cass_ssl_set_private_key(ssl: *mut CassSsl, key: CassString, password: *const c_char) -> CassError;
}
