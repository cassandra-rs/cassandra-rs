#![allow(dead_code)]
#![allow(raw_pointer_derive)]
use std::fmt;
use std::slice;

use cql_ffi::types::cass_size_t;
use libc::types::os::arch::c95::c_char;
use cql_bindgen::CassString as _CassString;
use cql_bindgen::cass_string_init;
use cql_bindgen::cass_string_init2;

#[repr(C)]
#[derive(Copy)]
pub struct CassString(pub _CassString);

impl fmt::Debug for CassString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {unsafe{
        //if self.length > 1000000 {panic!("wtf: {}", self.length)};
        let data = self.0.data as *const u8;
        let slice = slice::from_raw_buf(&data,self.0.length as usize);
        let vec = slice.to_vec();
        match String::from_utf8(vec) {
            Ok(str_buf) => write!(f, "{}", str_buf),
            Err(err) => panic!(err)
        }
    }}
}

//~ impl ::std::default::Default for CassString {
    //~ fn default() -> CassString { unsafe { ::std::mem::zeroed() } }
//~ }

impl CassString {
    pub unsafe fn init(null_terminated: *const c_char) -> CassString {CassString(cass_string_init(null_terminated))}
    pub unsafe fn init2(data: *const c_char, length: cass_size_t) -> CassString {CassString(cass_string_init2(data,length))}
}
