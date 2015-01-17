#![allow(dead_code)]
#![allow(raw_pointer_derive)]
use std::fmt;
use std::raw;
use std::slice;
use std::str;
use std::ffi::CString;
use std::num::Int;

use cass_types::cass_size_t;
use libc::types::os::arch::c95::c_char;

use cass_error::CassError;
use cass_value::CassValue;
use cass_value::cass_value_get_string;

#[repr(C)]
#[derive(Copy)]
struct Struct_CassString_ {
    pub data: *const c_char,
    pub length: cass_size_t,
}

impl fmt::Show for Struct_CassString_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {unsafe{
        if self.length > 1000000 {panic!("wtf: {}", self.length)};
        let data = self.data as *const u8;
        let slice = slice::from_raw_buf(&data,self.length as usize);
        let vec = slice.to_vec();
        match String::from_utf8(vec) {
            Ok(str_buf) => write!(f, "{}", str_buf),
            Err(err) => panic!()
        }
    }}
}

impl ::std::default::Default for Struct_CassString_ {
    fn default() -> Struct_CassString_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassString = Struct_CassString_;

extern "C" {
    pub fn cass_string_init(null_terminated: *const c_char) -> CassString;
    pub fn cass_string_init2(data: *const c_char, length: cass_size_t) -> CassString;
}
