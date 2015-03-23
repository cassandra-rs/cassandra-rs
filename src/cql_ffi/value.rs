#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;
use std::ffi::CString;
use std::str;
use std::slice;

use cql_ffi::types::cass_size_t;
use cql_ffi::error::CassError;
use cql_ffi::inet::CassInet;
use cql_ffi::uuid::CassUuid;
use cql_ffi::iterator::set_iterator::SetIterator;
use cql_ffi::iterator::map_iterator::MapIterator;
use cql_ffi::decimal::CassDecimal;

use cql_bindgen::CassValue as _CassValue;
use cql_bindgen::cass_value_secondary_sub_type;
use cql_bindgen::cass_value_primary_sub_type;
use cql_bindgen::cass_value_item_count;
use cql_bindgen::cass_value_is_collection;
use cql_bindgen::cass_value_is_null;
use cql_bindgen::cass_value_type;
use cql_bindgen::cass_value_get_decimal;
use cql_bindgen::cass_value_get_inet;
use cql_bindgen::cass_value_get_string;
use cql_bindgen::cass_value_get_bytes;
use cql_bindgen::cass_value_get_uuid;
use cql_bindgen::cass_value_get_bool;
use cql_bindgen::cass_value_get_double;
use cql_bindgen::cass_value_get_float;
use cql_bindgen::cass_value_get_int64;
use cql_bindgen::cass_value_get_int32;
use cql_bindgen::cass_iterator_from_collection;
use cql_bindgen::cass_iterator_from_map;


use cql_bindgen::CASS_VALUE_TYPE_UNKNOWN;
use cql_bindgen::CASS_VALUE_TYPE_CUSTOM;
use cql_bindgen::CASS_VALUE_TYPE_ASCII;
use cql_bindgen::CASS_VALUE_TYPE_BIGINT;
use cql_bindgen::CASS_VALUE_TYPE_BLOB;
use cql_bindgen::CASS_VALUE_TYPE_BOOLEAN;
use cql_bindgen::CASS_VALUE_TYPE_COUNTER;
use cql_bindgen::CASS_VALUE_TYPE_DECIMAL;
use cql_bindgen::CASS_VALUE_TYPE_DOUBLE;
use cql_bindgen::CASS_VALUE_TYPE_FLOAT;
use cql_bindgen::CASS_VALUE_TYPE_INT;
use cql_bindgen::CASS_VALUE_TYPE_TEXT;
use cql_bindgen::CASS_VALUE_TYPE_TIMESTAMP;
use cql_bindgen::CASS_VALUE_TYPE_UUID;
use cql_bindgen::CASS_VALUE_TYPE_VARCHAR;
use cql_bindgen::CASS_VALUE_TYPE_TIMEUUID;
use cql_bindgen::CASS_VALUE_TYPE_INET;
use cql_bindgen::CASS_VALUE_TYPE_LIST;
use cql_bindgen::CASS_VALUE_TYPE_SET;
use cql_bindgen::CASS_VALUE_TYPE_MAP;
use cql_bindgen::CASS_VALUE_TYPE_VARINT;

use std::mem;

pub struct CassValue(pub *const _CassValue);

#[derive(Debug,Copy,PartialEq)]
pub enum CassValueType {
    UNKNOWN = CASS_VALUE_TYPE_UNKNOWN as isize,
    CUSTOM = CASS_VALUE_TYPE_CUSTOM as isize,
    ASCII = CASS_VALUE_TYPE_ASCII as isize,
    BIGINT = CASS_VALUE_TYPE_BIGINT as isize,
    BLOB = CASS_VALUE_TYPE_BLOB as isize,
    BOOLEAN = CASS_VALUE_TYPE_BOOLEAN as isize,
    COUNTER = CASS_VALUE_TYPE_COUNTER as isize,
    DECIMAL = CASS_VALUE_TYPE_DECIMAL as isize,
    DOUBLE = CASS_VALUE_TYPE_DOUBLE as isize,
    FLOAT = CASS_VALUE_TYPE_FLOAT as isize,
    INT = CASS_VALUE_TYPE_INT as isize,
    TEXT = CASS_VALUE_TYPE_TEXT as isize,
    TIMESTAMP = CASS_VALUE_TYPE_TIMESTAMP as isize,
    UUID = CASS_VALUE_TYPE_UUID as isize,
    VARCHAR = CASS_VALUE_TYPE_VARCHAR as isize,
    VARINT = CASS_VALUE_TYPE_VARINT as isize,
    TIMEUUID = CASS_VALUE_TYPE_TIMEUUID as isize,
    INET = CASS_VALUE_TYPE_INET as isize,
    LIST = CASS_VALUE_TYPE_LIST as isize,
    MAP = CASS_VALUE_TYPE_MAP as isize,
    SET = CASS_VALUE_TYPE_SET as isize,
}

impl CassValueType {
    pub fn build(_type:u32) -> Self {
        match _type {
            CASS_VALUE_TYPE_UNKNOWN     => CassValueType::UNKNOWN,
            CASS_VALUE_TYPE_CUSTOM      => CassValueType::CUSTOM,
            CASS_VALUE_TYPE_ASCII       => CassValueType::ASCII,
            CASS_VALUE_TYPE_BIGINT      => CassValueType::BIGINT,
            CASS_VALUE_TYPE_BLOB        => CassValueType::BLOB,
            CASS_VALUE_TYPE_BOOLEAN     => CassValueType::BOOLEAN,
            CASS_VALUE_TYPE_COUNTER     => CassValueType::COUNTER,
            CASS_VALUE_TYPE_DECIMAL     => CassValueType::DECIMAL,
            CASS_VALUE_TYPE_DOUBLE      => CassValueType::DOUBLE,
            CASS_VALUE_TYPE_FLOAT       => CassValueType::FLOAT,
            CASS_VALUE_TYPE_INT         => CassValueType::INT,
            CASS_VALUE_TYPE_TEXT        => CassValueType::TEXT,
            CASS_VALUE_TYPE_TIMESTAMP   => CassValueType::TIMESTAMP,
            CASS_VALUE_TYPE_UUID        => CassValueType::UUID,
            CASS_VALUE_TYPE_VARCHAR     => CassValueType::VARCHAR,
            CASS_VALUE_TYPE_VARINT      => CassValueType::VARINT,
            CASS_VALUE_TYPE_TIMEUUID    => CassValueType::TIMEUUID,
            CASS_VALUE_TYPE_INET        => CassValueType::INET,
            CASS_VALUE_TYPE_LIST        => CassValueType::LIST,
            CASS_VALUE_TYPE_MAP         => CassValueType::MAP,
            CASS_VALUE_TYPE_SET         => CassValueType::SET,
            _ => panic!("impossible value type")
        }
    }   
}

impl Debug for CassValue {

    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        match self.is_null() {
            true => return Ok(()),
            false => match self.get_type() {
                CassValueType::UNKNOWN          => write!(f, "{:?}", "unknown"),
                CassValueType::CUSTOM           => write!(f, "{:?}", "custom"),
                CassValueType::ASCII            => write!(f, "{:?}", self.get_string().unwrap()),
                CassValueType::BIGINT           => write!(f, "{:?}", self.get_int64().unwrap()),
                CassValueType::VARCHAR          => write!(f, "{:?}", self.get_string().unwrap()),
                CassValueType::BOOLEAN          => write!(f, "{:?}", self.get_bool().unwrap()),
                CassValueType::DOUBLE           => write!(f, "{:?}", self.get_double().unwrap()),
                CassValueType::FLOAT            => write!(f, "{:?}", self.get_float().unwrap()),
                CassValueType::INT              => write!(f, "{:?}", self.get_int32().unwrap()),
                CassValueType::TIMEUUID         => write!(f, "TIMEUUID: {:?}", self.get_uuid().unwrap()),
                CassValueType::SET      => {
                    try!(write!(f, "["));
                    for item in self.as_collection_iterator() {try!(write!(f,"{:?} ",item))}
                    try!(write!(f, "]"));
                    Ok(())
                }
                CassValueType::MAP => {
                    for item in self.as_map_iterator() {
                        try!(write!(f, "LIST {:?}", item ))
                    }
                    Ok(())
                },
            
                //FIXME
                err => write!(f, "{:?}", err),
            }
        }
    }
}

impl CassValue {
    pub fn fill_uuid(&self, mut output: CassUuid) -> Result<CassUuid,CassError> {unsafe{
        CassError::build(cass_value_get_uuid(self.0,&mut output.0)).wrap(output)
    }}

    pub fn fill_string<'a>(&'a self, mut output: &str) -> Result<String,CassError> {unsafe{
        let output = mem::zeroed();
        let output_length = mem::zeroed();
        let err = cass_value_get_string(self.0,output, output_length);

        let slice = slice::from_raw_parts(output as *const u8,output_length as usize);
        let string = str::from_utf8(slice).unwrap().to_string();
        CassError::build(err).wrap(string)
    }}

    pub fn get_bytes<'a>(&'a self) -> Result<Vec<u8>,CassError> {unsafe{
        let mut output = mem::zeroed();
        let output_size = mem::zeroed();
        let result = cass_value_get_bytes(self.0,&mut output, output_size);
        let slice = Vec::from_raw_buf(output as *const u8,output_size as usize);        
        CassError::build(result).wrap(slice)
    }}

    pub fn get_decimal<'a>(&'a self, mut output: CassDecimal) -> Result<CassDecimal,CassError> {unsafe{
        CassError::build(cass_value_get_decimal(self.0,&mut output.0)).wrap(output)
    }}

    pub fn get_type(&self) -> CassValueType {unsafe{
        CassValueType::build(cass_value_type(self.0))
    }}

    pub fn is_null(&self) -> bool {unsafe{
        if cass_value_is_null(self.0) > 0 {true} else {false}
    }}

    pub fn is_collection(&self) -> bool {unsafe{
        if cass_value_is_collection(self.0) > 0 {true} else {false}
    }}

    pub fn item_count(&self) -> cass_size_t {unsafe{
        cass_value_item_count(self.0)
    }}

    pub fn primary_sub_type(&self) -> CassValueType {unsafe{
        CassValueType::build(cass_value_primary_sub_type(self.0))
    }}

    pub fn secondary_sub_type(&self) -> CassValueType {unsafe{
        CassValueType::build(cass_value_secondary_sub_type(self.0))
    }}

    pub fn as_collection_iterator(&self) -> SetIterator {unsafe{
        SetIterator(cass_iterator_from_collection(self.0))
    }}

    pub fn as_map_iterator(&self) -> MapIterator {unsafe{
        MapIterator(cass_iterator_from_map(self.0))
    }}

    //~ pub fn map_iter(&self) -> Result<MapIterator,CassError> {unsafe{
        //~ match self.get_type() {
            //~ CassValueType::MAP => Ok(MapIterator(cass_iterator_from_map(self.0))),
            //~ type_no => {
                //~ println!("wrong_type: {:?}", type_no);
                //~ Err(CassError::build(CassErrorTypes::LIB_INVALID_VALUE_TYPE as u32))
            //~ }
        //~ }
    //~ }}

    pub fn get_string(&self) -> Result<String, CassError> {unsafe{
        let message:CString = mem::zeroed();
        let mut message = message.as_ptr();
        let mut message_length:u64 = mem::zeroed();
        cass_value_get_string(self.0, &mut message, &mut message_length);

        let slice = slice::from_raw_parts(message as *const u8,message_length as usize);
        let err = CassError::build(cass_value_get_string(self.0, &mut message, &mut message_length));
        err.wrap(str::from_utf8(slice).unwrap().to_string())
    }}
    
    //~ pub fn get_string(&self) -> Result<String,CassError> {unsafe{
        //~ let mut output = mem::zeroed();
        //~ let mut output_size = mem::zeroed();
        //~ let output = &mut output;
        //~ let foo = self.0;
        //~ cass_value_get_string(foo, output, output_size);
        //~ let err = CassError::build(cass_value_get_string(self.0, output, output_size));

        //~ let slice = slice::from_raw_parts(output,output_size as usize);
        //~ let string = str::from_utf8(slice).unwrap().to_string();

        

        //~ err.wrap(string)
    //~ }}

    pub unsafe fn get_inet<'a>(&'a self, mut output: CassInet) -> Result<CassInet,CassError> {CassError::build(cass_value_get_inet(self.0,&mut output.0)).wrap(output)}
    
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

}
