#![allow(dead_code)]
#![allow(raw_pointer_derive)]

use std::ffi::CString;

use cass_types::cass_size_t;
use libc::types::os::arch::c95::c_char;

#[repr(C)]
#[derive(Copy)]
struct Struct_CassString_ {
    pub data: *const c_char,
    pub length: cass_size_t,
}
impl ::std::default::Default for Struct_CassString_ {
    fn default() -> Struct_CassString_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassString = Struct_CassString_;

extern "C" {
    pub fn cass_string_init(null_terminated: *const c_char) -> CassString;
    pub fn cass_string_init2(data: *const c_char, length: cass_size_t) -> CassString;
}
