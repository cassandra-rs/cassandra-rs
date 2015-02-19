use cql_bindgen::CassValue as _CassValue;

use cql_ffi::uuid::CassUuid;
use cql_ffi::string::CassString;
use cql_ffi::inet::CassInet;
use cql_ffi::iterator::CassIterator;
use cql_ffi::iterator::MapIterator;

use std::mem;

use cql_bindgen::cass_value_get_int32;
use cql_bindgen::cass_value_get_int64;
use cql_bindgen::cass_value_get_float;
use cql_bindgen::cass_value_get_double;
use cql_bindgen::cass_value_get_bool;
use cql_bindgen::cass_value_get_uuid;
use cql_bindgen::cass_value_get_string;
use cql_bindgen::cass_value_get_inet;
use cql_bindgen::cass_iterator_from_map;


use cql_ffi::error::CassError;

#[repr(C)]
#[derive(Copy,Debug)]
pub enum CassColumnType {
    PARTITION_KEY = 0,
    CLUSTERING_KEY = 1,
    REGULAR = 2,
    COMPACT_VALUE = 3,
    STATIC = 4,
    UNKNOWN = 5,
}

pub struct CassColumn(pub *const _CassValue);

impl CassColumn {
    pub unsafe fn get_inet<'a>(&'a self, mut output: CassInet) -> Result<CassInet,CassError> {CassError::build(cass_value_get_inet(self.0,&mut output.0)).wrap(output)}

    pub fn get_string(&self) -> Result<CassString,CassError> {unsafe{
        let mut output:CassString = mem::zeroed();
        CassError::build(cass_value_get_string(self.0,&mut output.0)).wrap(output)
    }}

    
    pub fn get_int32(&self) -> Result<i32,CassError> {unsafe{
        let mut output = mem::zeroed();
        CassError::build(cass_value_get_int32(self.0,&mut output)).wrap(output)
    }}

    pub fn get_int64(&self) -> Result<i64,CassError> {unsafe{
        let mut output = mem::zeroed();
        CassError::build(cass_value_get_int64(self.0,&mut output)).wrap(output)
    }}

    pub fn get_float(&self) -> Result<f32,CassError> {unsafe{
        let mut output = mem::zeroed();
        CassError::build(cass_value_get_float(self.0,&mut output)).wrap(output)
    }}

    pub fn get_double(&self) -> Result<f64,CassError> {unsafe{
        let mut output = mem::zeroed();
        CassError::build(cass_value_get_double(self.0,&mut output)).wrap(output)
    }}

    pub fn get_bool(&self) -> Result<bool,CassError> {unsafe{
        let mut output = mem::zeroed();
        CassError::build(cass_value_get_bool(self.0,&mut output)).wrap(if output > 0 {true} else {false})
    }}

    pub fn get_uuid(&self) -> Result<CassUuid,CassError> {unsafe{
        let mut output:CassUuid = mem::zeroed();CassError::build(cass_value_get_uuid(self.0,&mut output.0)).wrap(output)
    }}

    pub fn map_iterator(&self) -> MapIterator {unsafe{MapIterator(cass_iterator_from_map(self.0))}}


}
