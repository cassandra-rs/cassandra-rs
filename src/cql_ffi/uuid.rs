#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(missing_copy_implementations)]

use std::fmt::Formatter;
use std::fmt;
use std::fmt::Debug;
use std::mem;


use cql_ffi::types::cass_uint64_t;
use cql_ffi::types::cass_uint8_t;
use cql_bindgen::CassUuid as _CassUuid;
use cql_bindgen::CassUuidGen as _CassUuidGen;
use cql_bindgen::cass_uuid_gen_new;
use cql_bindgen::cass_uuid_gen_free;
use cql_bindgen::cass_uuid_gen_time;
use cql_bindgen::cass_uuid_gen_new_with_node;
use cql_bindgen::cass_uuid_gen_random;
use cql_bindgen::cass_uuid_gen_from_time;
use cql_bindgen::cass_uuid_min_from_time;
use cql_bindgen::cass_uuid_max_from_time;
use cql_bindgen::cass_uuid_timestamp;
use cql_bindgen::cass_uuid_version;
use cql_bindgen::cass_uuid_string;
//use cql_bindgen::cass_uuid_from_string;

//use cql_ffi::error::CassError;

#[derive(Copy)]
pub struct CassUuid(pub _CassUuid);

impl ::std::default::Default for CassUuid {
    fn default() -> CassUuid { unsafe { ::std::mem::zeroed() } }
}

pub struct CassUuidGen(*mut _CassUuidGen);

impl Drop for CassUuidGen {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl Debug for CassUuid {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string())
    }      
}

impl CassUuid {
    pub unsafe fn min_from_time(&mut self, time: cass_uint64_t) {cass_uuid_min_from_time(time,&mut self.0)}
    pub unsafe fn max_from_time(&mut self, time: cass_uint64_t) {cass_uuid_max_from_time(time,&mut self.0)}
    pub unsafe fn timestamp(&self) -> u64 {cass_uuid_timestamp(self.0)}
    pub unsafe fn version(&self) -> cass_uint8_t {cass_uuid_version(self.0)}
    
    pub fn to_string(&self) {unsafe{let output = mem::zeroed();cass_uuid_string(self.0,output)}}
    //pub unsafe fn from_string(&mut self, str: *const c_char) -> Result<(),CassError> {CassError::build(cass_uuid_from_string(str,&mut self.0))}
}

impl CassUuidGen {
    pub unsafe fn new() -> Self {CassUuidGen(cass_uuid_gen_new())}
    
    pub unsafe fn new_with_node(node: cass_uint64_t) -> CassUuidGen {CassUuidGen(cass_uuid_gen_new_with_node(node))}
    
    unsafe fn free(&self) {cass_uuid_gen_free(self.0)}
    
    pub unsafe fn time(&self) -> CassUuid {
        let mut output = mem::zeroed();
        cass_uuid_gen_time(self.0,&mut output);
        CassUuid(output)
    }
    
    pub unsafe fn fill_random(&self, mut output: CassUuid) {cass_uuid_gen_random(self.0, &mut output.0)}
    
    pub unsafe fn random(&self) -> CassUuid {
        let mut output:CassUuid = mem::zeroed();
        cass_uuid_gen_random(self.0, &mut output.0);
        output
    }
    pub unsafe fn from_time(&self, timestamp: cass_uint64_t, mut output: CassUuid){cass_uuid_gen_from_time(self.0,timestamp, &mut output.0)}

}
