#![allow(dead_code)]
#![allow(raw_pointer_derive)]
use cql_bindgen::CassString as _CassString;

use std::fmt::Formatter;
use std::fmt;
use std::fmt::Debug;

use std::string::ToString;

#[repr(C)]
pub struct CassString(pub _CassString);

//~ impl Deref for CassString {
    //~ type Target = str;
    //~ fn deref<'a>(&'a self) -> &'a str {unsafe{
        //~ let data = self.0.data as *mut u8;
        //~ &String::from_raw_parts(data,self.0.length as usize, self.0.length as usize)[]
    //~ }}
//~ }

pub trait AsCassStr {
    fn as_cass_str(&self) -> CassString;
}

impl AsCassStr for str {
    fn as_cass_str(&self) -> CassString {
        CassString(_CassString{
            data: self.as_ptr() as *const i8,
            length: self.len() as u64,
        })
    }
}

impl ToString for CassString {
    fn to_string(&self) -> String {unsafe{
        let data = self.0.data as *mut u8;
        String::from_raw_parts(data,self.0.length as usize, self.0.length as usize).clone()
    }}
}

impl Debug for CassString {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string())
    }      
}

//~ trait WriteCqlStr {
    //~ fn write_to<W>(&self, &mut W)
        //~ where W: Writer; 
//~ }

//~ impl WriteCqlStr for CassString {
    //~ fn write_to(&self, writer:&mut Self) {

    //~ }
    //~ // Straight-forward impl, write the bytes we refer to
//~ }

//~ impl<'a> WriteCqlStr for &'a str {
    //~ // Write the length, then write the bytes of the str
//~ }

//~ impl ::std::default::Default for CassString {
    //~ fn default() -> CassString { unsafe { ::std::mem::zeroed() } }
//~ }

//~ impl CassString {
    //~ pub fn build(str:&str) -> Result<Self,CassError> {
        //~ FromStr::from_str(str)
    //~ }
//~ }
