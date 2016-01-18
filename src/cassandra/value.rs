use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::ffi::CString;
use std::str;
use std::slice;

use cassandra::error::CassError;
use cassandra::inet::Inet;
use cassandra::uuid;
use cassandra::uuid::Uuid;
use cassandra::iterator::MapIterator;
use cassandra::iterator::SetIterator;
use cassandra::error::CassErrorTypes;
use cassandra_sys::CassValue as _CassValue;
#[allow(unused_imports)]
use cassandra_sys::cass_value_secondary_sub_type;
#[allow(unused_imports)]
use cassandra_sys::cass_value_primary_sub_type;
#[allow(unused_imports)]
use cassandra_sys::cass_value_item_count;
use cassandra_sys::cass_value_is_collection;
use cassandra_sys::cass_value_is_null;
use cassandra_sys::cass_value_type;
#[allow(unused_imports)]
use cassandra_sys::cass_collection_append_decimal;
use cassandra_sys::cass_value_get_inet;
use cassandra_sys::cass_value_get_string;
use cassandra_sys::cass_value_get_bytes;
use cassandra_sys::cass_value_get_uuid;
use cassandra_sys::cass_value_get_bool;
use cassandra_sys::cass_value_get_double;
use cassandra_sys::cass_value_get_float;
use cassandra_sys::cass_value_get_int64;
use cassandra_sys::cass_value_get_int32;
use cassandra_sys::cass_iterator_from_collection;
use cassandra_sys::cass_iterator_from_map;
use cassandra_sys::cass_value_data_type;


use cassandra_sys::CASS_VALUE_TYPE_UNKNOWN;
use cassandra_sys::CASS_VALUE_TYPE_CUSTOM;
use cassandra_sys::CASS_VALUE_TYPE_ASCII;
use cassandra_sys::CASS_VALUE_TYPE_BIGINT;
use cassandra_sys::CASS_VALUE_TYPE_BLOB;
use cassandra_sys::CASS_VALUE_TYPE_BOOLEAN;
use cassandra_sys::CASS_VALUE_TYPE_COUNTER;
use cassandra_sys::CASS_VALUE_TYPE_DECIMAL;
use cassandra_sys::CASS_VALUE_TYPE_DOUBLE;
use cassandra_sys::CASS_VALUE_TYPE_FLOAT;
use cassandra_sys::CASS_VALUE_TYPE_INT;
use cassandra_sys::CASS_VALUE_TYPE_TEXT;
use cassandra_sys::CASS_VALUE_TYPE_TIMESTAMP;
use cassandra_sys::CASS_VALUE_TYPE_UUID;
use cassandra_sys::CASS_VALUE_TYPE_VARCHAR;
use cassandra_sys::CASS_VALUE_TYPE_TIMEUUID;
use cassandra_sys::CASS_VALUE_TYPE_INET;
use cassandra_sys::CASS_VALUE_TYPE_LIST;
use cassandra_sys::CASS_VALUE_TYPE_SET;
use cassandra_sys::CASS_VALUE_TYPE_MAP;
use cassandra_sys::CASS_VALUE_TYPE_VARINT;
use cassandra_sys::CASS_VALUE_TYPE_UDT;
use cassandra_sys::CASS_VALUE_TYPE_TUPLE;
use cassandra_sys::CASS_VALUE_TYPE_LAST_ENTRY;

use cassandra::inet::protected::inner as inet_protected;
use cassandra::data_type::ConstDataType;
use cassandra::iterator;

use std::mem;

///A single primitive value or a collection of values.
pub struct Value(*const _CassValue);

pub mod protected {
	use cassandra::value::Value;
	use cassandra_sys::CassValue as _Value;
	pub fn build(value:*const _Value) -> Value {
		Value(value)
	}
	
	pub fn inner(value:&Value) -> *const _Value {
		value.0
	}
}

#[derive(Debug)]
///The various types of types that a Cassandra value can be
#[allow(missing_docs)]
pub enum ValueType {
    ///An unknown cassandra type. returned if an index was out of bound or a columnn name wasn't found
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
    UDT = CASS_VALUE_TYPE_UDT as isize,
    TUPLE = CASS_VALUE_TYPE_TUPLE as isize,
    LASTENTRY = CASS_VALUE_TYPE_LAST_ENTRY as isize,
}

impl ValueType {
    #[allow(missing_docs)]
    pub fn build(type_: u32) -> Result<Self, CassError> {
        match type_ {
            CASS_VALUE_TYPE_UNKNOWN => Err(CassError::build(CassErrorTypes::LIB_INDEX_OUT_OF_BOUNDS as u32, None)),
            CASS_VALUE_TYPE_CUSTOM => Ok(ValueType::CUSTOM),
            CASS_VALUE_TYPE_ASCII => Ok(ValueType::ASCII),
            CASS_VALUE_TYPE_BIGINT => Ok(ValueType::BIGINT),
            CASS_VALUE_TYPE_BLOB => Ok(ValueType::BLOB),
            CASS_VALUE_TYPE_BOOLEAN => Ok(ValueType::BOOLEAN),
            CASS_VALUE_TYPE_COUNTER => Ok(ValueType::COUNTER),
            CASS_VALUE_TYPE_DECIMAL => Ok(ValueType::DECIMAL),
            CASS_VALUE_TYPE_DOUBLE => Ok(ValueType::DOUBLE),
            CASS_VALUE_TYPE_FLOAT => Ok(ValueType::FLOAT),
            CASS_VALUE_TYPE_INT => Ok(ValueType::INT),
            CASS_VALUE_TYPE_TEXT => Ok(ValueType::TEXT),
            CASS_VALUE_TYPE_TIMESTAMP => Ok(ValueType::TIMESTAMP),
            CASS_VALUE_TYPE_UUID => Ok(ValueType::UUID),
            CASS_VALUE_TYPE_VARCHAR => Ok(ValueType::VARCHAR),
            CASS_VALUE_TYPE_VARINT => Ok(ValueType::VARINT),
            CASS_VALUE_TYPE_TIMEUUID => Ok(ValueType::TIMEUUID),
            CASS_VALUE_TYPE_INET => Ok(ValueType::INET),
            CASS_VALUE_TYPE_LIST => Ok(ValueType::LIST),
            CASS_VALUE_TYPE_MAP => Ok(ValueType::MAP),
            CASS_VALUE_TYPE_SET => Ok(ValueType::SET),
            CASS_VALUE_TYPE_UDT => Ok(ValueType::UDT),
            err => panic!("impossible value type{}", err),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_null() {
            Ok(())
        } else {
            match self.get_type() {
                ValueType::UNKNOWN => write!(f, "{:?}", "unknown"),
                ValueType::CUSTOM => write!(f, "{:?}", "custom"),
                ValueType::ASCII => write!(f, "{:?}", self.get_string().unwrap()),
                ValueType::BIGINT => write!(f, "{:?}", self.get_i64().unwrap()),
                ValueType::VARCHAR => write!(f, "{:?}", self.get_string().unwrap()),
                ValueType::BOOLEAN => write!(f, "{:?}", self.get_bool().unwrap()),
                ValueType::DOUBLE => write!(f, "{:?}", self.get_dbl().unwrap()),
                ValueType::FLOAT => write!(f, "{:?}", self.get_flt().unwrap()),
                ValueType::INT => write!(f, "{:?}", self.get_i32().unwrap()),
                ValueType::TIMEUUID => write!(f, "TIMEUUID: {:?}", self.get_uuid().unwrap()),
                ValueType::SET => {
                    try!(write!(f, "["));
                    for item in self.get_set().unwrap() {
                        try!(write!(f, "SET {:?} ", item))
                    }
                    try!(write!(f, "]"));
                    Ok(())
                }
                ValueType::MAP => {
                    for item in self.get_map().unwrap() {
                        try!(write!(f, "MAP {:?}:{:?}", item.0, item.1))
                    }
                    Ok(())
                }
                ValueType::UDT => {
                    //                    for item in self.as_map_iterator().unwrap() {
                    //                        try!(write!(f, "MAP {:?}:{:?}", item.0,item.1))
                    //                    }
                    Ok(())
                }
                // FIXME
                err => write!(f, "{:?}", err),
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_null() {
            Ok(())
        } else {
            match self.get_type() {
                ValueType::UNKNOWN => write!(f, "{}", "unknown"),
                ValueType::CUSTOM => write!(f, "{}", "custom"),
                ValueType::ASCII => write!(f, "{}", self.get_string().unwrap()),
                ValueType::BIGINT => write!(f, "{}", self.get_i64().unwrap()),
                ValueType::VARCHAR => write!(f, "{}", self.get_string().unwrap()),
                ValueType::BOOLEAN => write!(f, "{}", self.get_bool().unwrap()),
                ValueType::DOUBLE => write!(f, "{}", self.get_dbl().unwrap()),
                ValueType::FLOAT => write!(f, "{}", self.get_flt().unwrap()),
                ValueType::INT => write!(f, "{}", self.get_i32().unwrap()),
                ValueType::TIMEUUID => write!(f, "TIMEUUID: {}", self.get_uuid().unwrap()),
                ValueType::SET => {
                    try!(write!(f, "["));
                    for item in self.get_set().unwrap() {
                        try!(write!(f, "{} ", item))
                    }
                    try!(write!(f, "]"));
                    Ok(())
                }
                ValueType::MAP => {
                    for item in self.get_map().unwrap() {
                        try!(write!(f, "MAP {}:{}", item.0, item.1))
                    }
                    Ok(())
                }
                // FIXME
                err => write!(f, "{:?}", err),
            }
        }
    }
}

impl Value {
    // FIXME a low level optimization. not sure whether to include or not
    //    pub fn fill_uuid(&self, mut uuid: Uuid) -> Result<Uuid, CassError> {
    //        unsafe { CassError::build(cass_value_get_uuid(self.0, &mut uuid.0), None).wrap(uuid) }
    //    }
    //
    // FIXME a low level optimization. not sure whether to include or not
    //    pub fn fill_string(&self) -> Result<String, CassError> {
    //        unsafe {
    //            let output = mem::zeroed();
    //            let output_length = mem::zeroed();
    //            let err = cass_value_get_string(self.0, output, output_length);
    //
    //            let slice = slice::from_raw_parts(output as *const u8, output_length as usize);
    //            let string = str::from_utf8(slice).unwrap().to_owned();
    //            CassError::build(err, None).wrap(string)
    //        }
    //    }

    //    // FIXME test this
    //    pub fn get_bytes(&self) -> Result<Vec<u8>, CassError> {
    //        unsafe {
    //            let mut output: *const u8 = mem::zeroed();
    //            let output_size = mem::zeroed();
    //            let result = cass_value_get_bytes(self.0, &mut output, output_size);
    //            // let output:*mut u8 = &mut*output;
    //            let slice = Vec::from_raw_parts(output, output_size as usize, output_size as usize);
    //            let r = CassError::build(result);
    //            r.wrap(slice)
    //        }
    //    }

    /// Gets the name of the keyspace.
    pub fn get_bytes(&self) -> Result<&[u8], CassError> {
        unsafe {
            let mut output = mem::zeroed();
            let mut output_size = mem::zeroed();
            let result = cass_value_get_bytes(self.0, &mut output, &mut output_size);
            // raw2utf8(output, output_size).unwrap()
            let slice = slice::from_raw_parts(output, output_size as usize);
            let r = CassError::build(result, None);
            r.wrap(slice)
        }
    }
    // pub fn get_decimal<'a>(&'a self, mut output: String) ->
    // Result<String,CassError> {unsafe{
    // CassError::build(cass_value_get_decimal(self.0,&mut
    // output)).wrap(output)
    //    }}

    ///Get the type of this Cassandra value
    pub fn get_type(&self) -> ValueType {
        unsafe { ValueType::build(cass_value_type(self.0)).unwrap() }
    }

    ///Get the data type of this Cassandra value
    pub fn data_type(&self) -> ConstDataType {
        unsafe { ConstDataType(cass_value_data_type(self.0)) }
    }

    ///Returns true if a specified value is null.
    pub fn is_null(&self) -> bool {
        unsafe { cass_value_is_null(self.0) > 0 }
    }

    ///Returns true if a specified value is a collection.
    pub fn is_collection(&self) -> bool {
        unsafe { cass_value_is_collection(self.0) > 0 }
    }


    //    pub fn item_count(&self) -> u64 {
    //        unsafe { cass_value_item_count(self.0) }
    //    }


    // 	///Get the primary sub-type for a collection. This returns the sub-type for a
    // 	///list or set and the key type for a map.
    //    pub fn primary_sub_type(&self) -> ValueType {
    //        unsafe { ValueType::build(cass_value_primary_sub_type(self.0)).unwrap() }
    //    }

    // 	///Get the secondary sub-type for a collection. This returns the value type for a map.
    //    pub fn secondary_sub_type(&self) -> ValueType {
    //        unsafe { ValueType::build(cass_value_secondary_sub_type(self.0)).unwrap() }
    //    }

    ///Gets this value as a set iterator.
    pub fn get_set(&self) -> Result<SetIterator, CassError> {
        unsafe {
            match self.get_type() {
                ValueType::SET => Ok(iterator::protected::CassIterator::build(cass_iterator_from_collection(self.0))),
                _ => Err(CassError::build(CassErrorTypes::LIB_INVALID_VALUE_TYPE as u32, None)),
            }
        }
    }

    ///Gets this value as a map iterator.
    pub fn get_map(&self) -> Result<MapIterator, CassError> {
        unsafe {
            match self.get_type() {
                ValueType::MAP => Ok(iterator::protected::CassIterator::build(cass_iterator_from_map(self.0))),
                _ => Err(CassError::build(CassErrorTypes::LIB_INVALID_VALUE_TYPE as u32, None)),
            }
        }
    }

    //    pub fn as_user_type_iterator(&self) -> Result<UserTypeIterator, CassError> {
    //        unsafe {
    //            match self.get_type() {
    //                ValueType::UDT => Ok(UserTypeIterator(cass_iterator_from_user_type(self.0))),
    //                _ => Err(CassError::build(CassErrorTypes::LIB_INVALID_VALUE_TYPE as u32)),
    //            }
    //        }
    //    }


    // ~ pub fn map_iter(&self) -> Result<MapIterator,CassError> {unsafe{
    // ~ match self.get_type() {
    // ~ ValueType::MAP => Ok(MapIterator(cass_iterator_from_map(self.0))),
    // ~ type_no => {
    // ~ println!("wrong_type: {:?}", type_no);
    // ~ Err(CassError::build(CassErrorTypes::LIB_INVALID_VALUE_TYPE as
    // u32))
    // ~ }
    // ~ }
    // ~ }}

    ///Get this value as a string
    pub fn get_string(&self) -> Result<String, CassError> {
        unsafe {
            let message: CString = mem::zeroed();
            let mut message = message.as_ptr();
            let mut message_length = mem::zeroed();
            cass_value_get_string(self.0, &mut message, &mut (message_length));

            let slice = slice::from_raw_parts(message as *const u8, message_length as usize);
            let err = CassError::build(cass_value_get_string(self.0, &mut message, &mut (message_length)),
                                       None);
            err.wrap(str::from_utf8(slice).unwrap().to_owned())
        }
    }

    // ~ pub fn get_string(&self) -> Result<String,CassError> {unsafe{
    // ~ let mut output = mem::zeroed();
    // ~ let mut output_size = mem::zeroed();
    // ~ let output = &mut output;
    // ~ let foo = self.0;
    // ~ cass_value_get_string(foo, output, output_size);
    // ~ let err = CassError::build(cass_value_get_string(self.0, output,
    // output_size));

    // ~ let slice = slice::from_raw_parts(output,output_size as usize);
    // ~ let string = str::from_utf8(slice).unwrap().to_string();



    // ~ err.wrap(string)
    // ~ }}

    ///Get this value as an Inet
    pub fn get_inet(&self) -> Result<Inet, CassError> {
        unsafe {
            let output: Inet = mem::zeroed();
            CassError::build(cass_value_get_inet(self.0, &mut inet_protected(&output)),
                             None)
                .wrap(output)
        }
    }

    ///Get this value as an i32
    pub fn get_i32(&self) -> Result<i32, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_int32(self.0, &mut output), None).wrap(output)
        }
    }

    ///Get this value as an i64
    pub fn get_i64(&self) -> Result<i64, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_int64(self.0, &mut output), None).wrap(output)
        }
    }

    ///Get this value as a float
    pub fn get_flt(&self) -> Result<f32, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_float(self.0, &mut output), None).wrap(output)
        }
    }

    ///Get this value as a double
    pub fn get_dbl(&self) -> Result<f64, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_double(self.0, &mut output), None).wrap(output)
        }
    }

    ///Get this value asa boolean
    pub fn get_bool(&self) -> Result<bool, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_bool(self.0, &mut output), None).wrap(output > 0)
        }
    }

    ///Get this value as a UUID
    pub fn get_uuid(&self) -> Result<Uuid, CassError> {
        unsafe {
            let mut output: Uuid = mem::zeroed();
            CassError::build(cass_value_get_uuid(self.0, &mut uuid::protected::inner(output)), None).wrap(output)
        }
    }
}
