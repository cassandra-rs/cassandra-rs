use std::ffi::CString;

use cql_bindgen::CassCollection as _CassCollection;
use cql_bindgen::cass_collection_new;
use cql_bindgen::cass_collection_free;
use cql_bindgen::cass_collection_append_int32;
use cql_bindgen::cass_collection_append_int64;
use cql_bindgen::cass_collection_append_float;
use cql_bindgen::cass_collection_append_double;
use cql_bindgen::cass_collection_append_bool;
use cql_bindgen::cass_collection_append_bytes;
use cql_bindgen::cass_collection_append_uuid;
use cql_bindgen::cass_collection_append_string;
use cql_bindgen::cass_collection_append_inet;
use cql_bindgen::cass_collection_append_decimal;

use cql_ffi::collection::cass_collection::CassCollectionType;
use cql_ffi::error::CassError;
use cql_ffi::uuid::CassUuid;
use cql_ffi::inet::CassInet;
use cql_ffi::types::cass_size_t;

pub struct CassList(pub *mut _CassCollection);


impl Drop for CassList {
    fn drop(&mut self) {
        self.free()
    }
}

impl CassList {
    pub fn new(item_count: cass_size_t) -> CassList {unsafe{
        CassList(cass_collection_new(CassCollectionType::LIST as u32,item_count))
    }}

    fn free(&mut self) {unsafe{
        cass_collection_free(self.0)
    }}

    pub fn append_int32<'a>(&'a mut self, value: i32) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_collection_append_int32(self.0,value)).wrap(self)
    }}

    pub fn append_int64<'a>(&'a mut self, value: i64) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_collection_append_int64(self.0,value)).wrap(self)
    }}

    pub fn append_float<'a>(&'a mut self, value: f32) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_collection_append_float(self.0,value)).wrap(self)
    }}

    pub fn append_double<'a>(&'a mut self, value: f64) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_collection_append_double(self.0,value)).wrap(self)
    }}

    pub fn append_bool<'a>(&'a mut self, value: bool) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_collection_append_bool(self.0,if value {1} else {0})).wrap(self)
    }}

    pub fn append_string<'a>(&'a mut self, value: &str) -> Result<&'a Self,CassError> {unsafe{
        let cstr = CString::new(value).unwrap();
        let result = cass_collection_append_string(self.0,cstr.as_ptr());
        CassError::build(result).wrap(self)
    }}

    pub fn append_bytes<'a>(&'a mut self, value: Vec<u8>) -> Result<&'a Self,CassError> {unsafe{
        let bytes = cass_collection_append_bytes(self.0,value[..].as_ptr(), value.len() as u64);
        CassError::build(bytes).wrap(self)
    }}

    pub fn append_uuid<'a>(&'a mut self, value: CassUuid) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_collection_append_uuid(self.0,value.0)).wrap(self)
    }}

    pub fn append_inet<'a>(&'a mut self, value: CassInet) -> Result<&'a Self,CassError> {unsafe{
        CassError::build(cass_collection_append_inet(self.0,value.0)).wrap(self)
    }}

//FIXME rust doesn't have good decimal support yet
//    pub fn append_decimal<'a>(&'a mut self, value: String) -> Result<&'a Self,CassError> {unsafe{
//        CassError::build(cass_collection_append_decimal(self.0,value)).wrap(self)
//    }}
    

}
