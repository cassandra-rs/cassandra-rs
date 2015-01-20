#![allow(dead_code)]
#![allow(raw_pointer_derive)]
use std::fmt;
use std::slice;

use cass_types::cass_size_t;
use libc::types::os::arch::c95::c_char;

#[repr(C)]
#[derive(Copy)]
pub struct CassString {
    pub data: *const c_char,
    pub length: cass_size_t,
}

impl fmt::Show for CassString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {unsafe{
        if self.length > 1000000 {panic!("wtf: {}", self.length)};
        let data = self.data as *const u8;
        let slice = slice::from_raw_buf(&data,self.length as usize);
        let vec = slice.to_vec();
        match String::from_utf8(vec) {
            Ok(str_buf) => write!(f, "{}", str_buf),
            Err(err) => panic!(err)
        }
    }}
}

impl ::std::default::Default for CassString {
    fn default() -> CassString { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn cass_string_init(null_terminated: *const c_char) -> CassString;
    pub fn cass_string_init2(data: *const c_char, length: cass_size_t) -> CassString;
}
