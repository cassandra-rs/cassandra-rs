use cql_bindgen::CassValue as _CassValue;

use cql_ffi::uuid::CassUuid;
use cql_ffi::value::CassValueType;
use cql_ffi::inet::CassInet;
use cql_ffi::iterator::set_iterator::SetIterator;
use cql_ffi::iterator::map_iterator::MapIterator;
use cql_ffi::error::CassErrorTypes;

use std::mem;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;
use std::str;
use std::slice;
use std::ffi::CString;

use cql_bindgen::CASS_VALUE_TYPE_ASCII;
use cql_bindgen::cass_value_get_int32;
use cql_bindgen::cass_value_get_int64;
use cql_bindgen::cass_value_get_float;
use cql_bindgen::cass_value_get_double;
use cql_bindgen::cass_value_get_bool;
use cql_bindgen::cass_value_get_uuid;
use cql_bindgen::cass_value_get_string;
use cql_bindgen::cass_value_get_inet;
use cql_bindgen::cass_iterator_from_map;
use cql_bindgen::cass_iterator_from_collection;
use cql_bindgen::cass_value_type;


use cql_ffi::error::CassError;

#[repr(C)]
#[derive(Copy,Debug,Clone)]
pub enum CassColumnType {
    PARTITION_KEY = 0,
    CLUSTERING_KEY = 1,
    REGULAR = 2,
    COMPACT_VALUE = 3,
    STATIC = 4,
    UNKNOWN = 5,
}

pub struct CassColumn(pub *const _CassValue);

impl Debug for CassColumn {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        match self.get_type() {
            CassValueType::UNKNOWN => write!(f, "UNKNOWN Cassandra type"),
            CassValueType::CUSTOM => write!(f, "CUSTOM Cassandra type"),
            CassValueType::ASCII => write!(f, "ASCII Cassandra type"),
            CassValueType::BIGINT => write!(f, "BIGINT Cassandra type"),
            CassValueType::BLOB => write!(f, "BLOB Cassandra type"),
            CassValueType::BOOLEAN => write!(f, "BOOLEAN Cassandra type"),
            CassValueType::COUNTER => write!(f, "COUNTER Cassandra type"),
            CassValueType::DECIMAL => write!(f, "DECIMAL Cassandra type"),
            CassValueType::DOUBLE => write!(f, "DOUBLE Cassandra type"),
            CassValueType::FLOAT => write!(f, "FLOAT Cassandra type"),
            CassValueType::INT => write!(f, "INT Cassandra type"),
            CassValueType::TEXT => write!(f, "TEXT Cassandra type"),
            CassValueType::TIMESTAMP => write!(f, "TIMESTAMP Cassandra type"),
            CassValueType::UUID => write!(f, "UUID Cassandra type"),
            CassValueType::VARCHAR => write!(f, "VARCHAR: {:?}", self.get_string()),
            CassValueType::VARINT => Ok(()),
            CassValueType::TIMEUUID => write!(f, "TIMEUUID Cassandra type"),
            CassValueType::INET => write!(f, "INET Cassandra type"),
            CassValueType::LIST => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item ))
                }
                Ok(())
            },
            CassValueType::MAP => {
               for item in self.map_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item ))
                }
                Ok(())
            },
            CassValueType::SET => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "SET {:?}", item ))
                }
                Ok(())
            },
        }
    }
}

trait AsTypedColumn {
    fn get(col:CassColumn) -> Result<Self,CassError>;
}

impl AsTypedColumn for bool {
    fn get(col:CassColumn) -> Result<Self,CassError> {
        col.get_bool()
    }
}

impl CassColumn {
    pub fn get_type(&self) -> CassValueType {unsafe{CassValueType::build(cass_value_type(self.0))}}
    
    pub unsafe fn get_inet<'a>(&'a self, mut output: CassInet) -> Result<CassInet,CassError> {CassError::build(cass_value_get_inet(self.0,&mut output.0)).wrap(output)}

    pub fn get_string(&self) -> Result<String,CassError> {unsafe{
        match cass_value_type(self.0) {
            CASS_VALUE_TYPE_ASCII => {
                let message:CString = mem::zeroed();
                let msg_ptr = &mut message.as_ptr();
                let message_length = mem::zeroed();
                cass_value_get_string(self.0, msg_ptr, message_length);

                let slice = slice::from_raw_parts(*msg_ptr as *const u8,message_length as usize);
                Ok(str::from_utf8(slice).unwrap().to_string())

            },
            err => panic!("incorrect type: {:?}", err)
        }
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

    pub fn map_iter(&self) -> Result<MapIterator,CassError> {unsafe{
        match self.get_type() {
            CassValueType::MAP => Ok(MapIterator(cass_iterator_from_map(self.0))),
            type_no => {
                println!("wrong_type: {:?}", type_no);
                Err(CassError::build(CassErrorTypes::LIB_INVALID_VALUE_TYPE as u32))
            }
        }
    }}

    pub fn set_iter(&self) -> Result<SetIterator,CassError> {unsafe{
        match self.get_type() {
            CassValueType::SET => Ok(SetIterator(cass_iterator_from_collection(self.0))),
            _ => Err(CassError::build(1))
        }
    }}


}
