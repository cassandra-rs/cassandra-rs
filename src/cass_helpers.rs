//Unlock the rest of the cass_ files, this is hand created and is a minimal set of helpers to make consuming the low level api palatable
use std::raw;
use std::mem;

use std::ffi::CString;

use cass_string::cass_string_init;
use cass_string::CassString;
use cass_value::CassValue;
use cass_error::CassError;

use cass_value::cass_value_get_string;

pub fn str2cass_string(query:&str) -> CassString {unsafe{
    let cass_str = cass_string_init(CString::from_slice(query.as_bytes()).as_ptr());
    cass_str
}}

pub fn str2ref(query:&str) -> *const i8 {
    query.as_ptr() as *const i8
}

pub fn cassvalue2cassstring<'a>(value:&'a CassValue) -> Result<CassString,CassError> {unsafe{
    let mut cass_string = mem::uninitialized();
    cass_value_get_string(value, &mut cass_string);
    Ok(cass_string)
}}

#[allow(unused)]
unsafe fn raw_byte_repr<'a, T>(ptr: &'a T) -> &'a [u8] {
        mem::transmute(raw::Slice{
            data: ptr as *const _ as *const u8,
            len: mem::size_of::<T>(),
        })
    }
