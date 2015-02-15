//Unlike the rest of the cass_ files, this is hand created and is a minimal set of helpers to make consuming the low level api palatable
use std::raw;
use std::mem;

use cql_ffi::string::CassString;
use cql_ffi::value::CassValue;
use cql_ffi::error::CassError;
use cql_ffi::uuid::CassUuid;

pub fn str2cass_string(query:&str) -> CassString {unsafe{
    CassString::init2(query.as_ptr() as *const i8,query.len() as u64)
}}

pub fn str2ref(query:&str) -> *const i8 {
    query.as_ptr() as *const i8
}

pub fn cassvalue2cassstring<'a>(value:&CassValue) -> Result<CassString,CassError> {unsafe{
    let cass_string = mem::uninitialized();
    let _ = CassValue::get_string(value, cass_string);
    Ok(cass_string)
}}


//~ pub fn gencassuuid<'a>(uuid_gen:&'a mut CassUuidGen) -> Result<CassUuid,CassError> {unsafe{
        //~ let mut key = mem::uninitialized();
        //~ cass_uuid_gen_time(uuid_gen, &mut key);
        //~ Ok(key)
//~ }}


pub fn cassvalue2cassuuid<'a>(value:&CassValue) -> Result<CassUuid,CassError> {unsafe{
    let mut cass_uuid:CassUuid = mem::uninitialized();
    let _ = CassValue::get_uuid(value, &mut cass_uuid);
    Ok(cass_uuid)
}}
//pub fn cass_value_get_uuid(value: *const CassValue, output: *mut CassUuid) -> CassError;


//~ pub fn cassuuid2string<'a>(uuid:CassUuid) -> Result<String,CassError> {unsafe{
    //~ let cass_uuid:*mut i8 = mem::uninitialized();
    //~ cass_uuid_string(uuid, cass_uuid);
    //~ let cass_uuid:*const i8 = cass_uuid;
    //~ Ok(String::from_utf8_lossy(c_str_to_bytes(&cass_uuid)).into_owned())
//~ }}

//pub fn cass_uuid_string(uuid: CassUuid, output: *mut c_char);


#[allow(unused)]
unsafe fn raw_byte_repr<'a, T>(ptr: &'a T) -> &'a [u8] {
        mem::transmute(raw::Slice{
            data: ptr as *const _ as *const u8,
            len: mem::size_of::<T>(),
        })
    }
