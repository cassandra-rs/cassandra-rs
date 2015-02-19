#![allow(dead_code)]
#![allow(raw_pointer_derive)]
//use std::fmt;
use std::ops::Deref;
use std::string::DerefString;   
use std::string::String;
use cql_ffi::helpers::str_to_ref;

use cql_ffi::types::cass_size_t;
use libc::types::os::arch::c95::c_char;
use cql_bindgen::CassString as _CassString;
use cql_bindgen::cass_string_init;
use cql_bindgen::cass_string_init2;
use cql_ffi::error::CassError;

use std::fmt::Formatter;
use std::fmt;
use std::fmt::Debug;

use std::str::FromStr;
use std::string::ToString;

#[repr(C)]
#[derive(Copy)]
pub struct CassString(pub _CassString);

//~ impl Deref for CassString {
    //~ type Target = str;
    //~ fn deref<'a>(&'a self) -> &'a str {unsafe{
        //~ let data = self.0.data as *mut u8;
        //~ &String::from_raw_parts(data,self.0.length as usize, self.0.length as usize)[]
    //~ }}
//~ }

impl ToString for CassString {
    fn to_string(&self) -> String {unsafe{
        let data = self.0.data as *mut u8;
        String::from_raw_parts(data,self.0.length as usize, self.0.length as usize)
        //self.0.length.to_string()
    }}
}
impl FromStr for CassString {
    type Err = CassError;
    fn from_str(string:&str) -> Result<Self,CassError> {unsafe{
        let cass_str = cass_string_init2(str_to_ref(string),string.len() as u64);
        Ok(CassString(cass_str))
    }}
}

impl Debug for CassString {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        write!(f, "{:?}", ToString::to_string(self))
    }      
}

//~ impl ::std::default::Default for CassString {
    //~ fn default() -> CassString { unsafe { ::std::mem::zeroed() } }
//~ }

impl CassString {
    pub fn build(str:&str) -> Self {FromStr::from_str(str).unwrap()}
    fn init(null_terminated: *const c_char) -> Self {unsafe{CassString(cass_string_init(null_terminated))}}
    pub unsafe fn init2(data: *const c_char, length: cass_size_t) -> Self {CassString(cass_string_init2(data,length))}
}
