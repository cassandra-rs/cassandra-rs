//Unlock the rest of the cass_ files, this is hand created and is a minimal set of helpers to make consuming the low level api palatable

use std::ffi::CString;

use cass_string::cass_string_init;
use cass_string::CassString;

pub fn str2cass_string(query:&str) -> CassString {unsafe{
    cass_string_init(cass_string_init(CString::from_slice(query.as_bytes()).as_ptr()).data)
}}

pub fn str2ref(query:&str) -> *const i8 {unsafe{
    query.as_ptr() as *const i8
}}
