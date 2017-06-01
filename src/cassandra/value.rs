

use cassandra::data_type::ConstDataType;

use cassandra::error::CassError;
use cassandra::inet::Inet;
use cassandra::iterator::MapIterator;
use cassandra::iterator::SetIterator;
use cassandra::util::Protected;
use cassandra::uuid::Uuid;
use cassandra_sys::CASS_OK;
use cassandra_sys::CASS_ERROR_LIB_INVALID_VALUE_TYPE;
use cassandra_sys::CASS_VALUE_TYPE_ASCII;
use cassandra_sys::CASS_VALUE_TYPE_BIGINT;
use cassandra_sys::CASS_VALUE_TYPE_BLOB;
use cassandra_sys::CASS_VALUE_TYPE_BOOLEAN;
use cassandra_sys::CASS_VALUE_TYPE_COUNTER;
use cassandra_sys::CASS_VALUE_TYPE_CUSTOM;
use cassandra_sys::CASS_VALUE_TYPE_DATE;
use cassandra_sys::CASS_VALUE_TYPE_DECIMAL;
use cassandra_sys::CASS_VALUE_TYPE_DOUBLE;
use cassandra_sys::CASS_VALUE_TYPE_FLOAT;
use cassandra_sys::CASS_VALUE_TYPE_INET;
use cassandra_sys::CASS_VALUE_TYPE_INT;
use cassandra_sys::CASS_VALUE_TYPE_LAST_ENTRY;
use cassandra_sys::CASS_VALUE_TYPE_LIST;
use cassandra_sys::CASS_VALUE_TYPE_MAP;
use cassandra_sys::CASS_VALUE_TYPE_SET;
use cassandra_sys::CASS_VALUE_TYPE_SMALL_INT;
use cassandra_sys::CASS_VALUE_TYPE_TEXT;
use cassandra_sys::CASS_VALUE_TYPE_TIME;
use cassandra_sys::CASS_VALUE_TYPE_TIMESTAMP;
use cassandra_sys::CASS_VALUE_TYPE_TIMEUUID;
use cassandra_sys::CASS_VALUE_TYPE_TINY_INT;
use cassandra_sys::CASS_VALUE_TYPE_TUPLE;
use cassandra_sys::CASS_VALUE_TYPE_UDT;
use cassandra_sys::CASS_VALUE_TYPE_UNKNOWN;
use cassandra_sys::CASS_VALUE_TYPE_UUID;
use cassandra_sys::CASS_VALUE_TYPE_VARCHAR;
use cassandra_sys::CASS_VALUE_TYPE_VARINT;
use cassandra_sys::CassValue as _CassValue;
use cassandra_sys::CassValueType as _CassValueType;
#[allow(unused_imports)]
use cassandra_sys::cass_collection_append_decimal;
use cassandra_sys::cass_iterator_from_collection;
use cassandra_sys::cass_iterator_from_map;
use cassandra_sys::cass_true;
use cassandra_sys::cass_value_data_type;
use cassandra_sys::cass_value_get_bool;
use cassandra_sys::cass_value_get_bytes;
use cassandra_sys::cass_value_get_double;
use cassandra_sys::cass_value_get_float;
use cassandra_sys::cass_value_get_inet;
use cassandra_sys::cass_value_get_int16;
use cassandra_sys::cass_value_get_int32;
use cassandra_sys::cass_value_get_int64;
use cassandra_sys::cass_value_get_int8;
use cassandra_sys::cass_value_get_string;
use cassandra_sys::cass_value_get_uuid;
use cassandra_sys::cass_value_is_collection;
use cassandra_sys::cass_value_is_null;
#[allow(unused_imports)]
use cassandra_sys::cass_value_item_count;
#[allow(unused_imports)]
use cassandra_sys::cass_value_primary_sub_type;
#[allow(unused_imports)]
use cassandra_sys::cass_value_secondary_sub_type;
use cassandra_sys::cass_value_type;
use errors::*;
use std::ffi::CString;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use std::mem;
use std::slice;
use std::str;

/// A single primitive value or a collection of values.
pub struct Value(*const _CassValue);

impl Protected<*const _CassValue> for Value {
    fn inner(&self) -> *const _CassValue { self.0 }
    fn build(inner: *const _CassValue) -> Self { Value(inner) }
}

#[derive(Debug)]
/// The various types of types that a Cassandra value can be
#[allow(missing_docs)]
pub struct ValueType(_CassValueType);

impl ValueType {
    #[allow(missing_docs)]
    pub fn build(typ: _CassValueType) -> Self { ValueType(typ) }
}

impl Protected<_CassValueType> for ValueType {
    fn inner(&self) -> _CassValueType { self.0 }
    fn build(inner: _CassValueType) -> Self { ValueType(inner) }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_null() {
            Ok(())
        } else {
            match self.get_type().0 {
                CASS_VALUE_TYPE_UNKNOWN => write!(f, "{:?}", "unknown"),
                CASS_VALUE_TYPE_CUSTOM => write!(f, "{:?}", "custom"),
                CASS_VALUE_TYPE_ASCII |
                CASS_VALUE_TYPE_TEXT |
                CASS_VALUE_TYPE_VARCHAR => write!(f, "{:?}", self.get_string().unwrap()),
                CASS_VALUE_TYPE_DECIMAL => write!(f, "{:?}", self.get_bytes().unwrap()),
                CASS_VALUE_TYPE_COUNTER => write!(f, "{:?}", self.get_i64().unwrap()),
                CASS_VALUE_TYPE_BIGINT => write!(f, "{:?}", self.get_i64().unwrap()),
                CASS_VALUE_TYPE_DATE => write!(f, "{:?}", self.get_string().unwrap()),
                CASS_VALUE_TYPE_TIME => write!(f, "{:?}", self.get_string().unwrap()),
                CASS_VALUE_TYPE_VARINT => write!(f, "{:?}", self.get_bytes().unwrap()),
                CASS_VALUE_TYPE_BOOLEAN => write!(f, "{:?}", self.get_bool().unwrap()),
                CASS_VALUE_TYPE_DOUBLE => write!(f, "{:?}", self.get_dbl().unwrap()),
                CASS_VALUE_TYPE_FLOAT => write!(f, "{:?}", self.get_flt().unwrap()),
                CASS_VALUE_TYPE_BLOB => write!(f, "{:?}", self.get_bytes().unwrap()),
                CASS_VALUE_TYPE_INT => write!(f, "{:?}", self.get_i32().unwrap()),
                CASS_VALUE_TYPE_SMALL_INT => write!(f, "{:?}", self.get_i16().unwrap()),
                CASS_VALUE_TYPE_TINY_INT => write!(f, "{:?}", self.get_i8().unwrap()),
                CASS_VALUE_TYPE_INET => write!(f, "{:?}", self.get_inet().unwrap()),
                CASS_VALUE_TYPE_TIMESTAMP => write!(f, "{:?}", self.get_i64().unwrap()),
                CASS_VALUE_TYPE_TIMEUUID => write!(f, "TIMEUUID: {}", self.get_uuid().unwrap()),
                CASS_VALUE_TYPE_LAST_ENTRY => unimplemented!(),
                CASS_VALUE_TYPE_UUID => write!(f, "UUID: {}", self.get_uuid().unwrap()),
                CASS_VALUE_TYPE_SET |
                CASS_VALUE_TYPE_LIST => {
                    write!(f, "[")?;
                    for item in self.get_set().expect("set must be a set") {
                        write!(f, "SET {:?} ", item)?
                    }
                    write!(f, "]")?;
                    Ok(())
                }
                CASS_VALUE_TYPE_MAP => {
                    for item in self.get_map().expect("map must be a map") {
                        write!(f, "MAP {:?}:{:?}", item.0, item.1)?
                    }
                    Ok(())
                }
                CASS_VALUE_TYPE_UDT => {
                    debug!("unimplemented for udt!");
                    //                    for item in self.as_map_iterator().unwrap() {
                    //                        try!(write!(f, "MAP {:?}:{:?}", item.0,item.1))
                    //                    }
                    Ok(())
                }
                CASS_VALUE_TYPE_TUPLE => {
                    debug!("unimplemented for tuple!");
                    //                    for item in self.as_map_iterator().unwrap() {
                    //                        try!(write!(f, "MAP {:?}:{:?}", item.0,item.1))
                    //                    }
                    Ok(())
                }
                // FIXME
                // err => write!(f, "{:?}", err),
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_null() {
            Ok(())
        } else {
            match self.get_type().0 {
                CASS_VALUE_TYPE_UNKNOWN => write!(f, "{}", "unknown"),
                CASS_VALUE_TYPE_CUSTOM => write!(f, "{}", "custom"),
                CASS_VALUE_TYPE_ASCII => write!(f, "{}", self.get_string().unwrap()),
                CASS_VALUE_TYPE_BIGINT => write!(f, "{}", self.get_i64().unwrap()),
                CASS_VALUE_TYPE_VARCHAR => write!(f, "{}", self.get_string().unwrap()),
                CASS_VALUE_TYPE_BOOLEAN => write!(f, "{}", self.get_bool().unwrap()),
                CASS_VALUE_TYPE_DOUBLE => write!(f, "{}", self.get_dbl().unwrap()),
                CASS_VALUE_TYPE_FLOAT => write!(f, "{}", self.get_flt().unwrap()),
                CASS_VALUE_TYPE_INT => write!(f, "{}", self.get_i32().unwrap()),
                CASS_VALUE_TYPE_TIMEUUID => write!(f, "TIMEUUID: {}", self.get_uuid().unwrap()),
                CASS_VALUE_TYPE_SET => {
                    write!(f, "[")?;
                    for item in self.get_set().expect("set must be a set") {
                        write!(f, "{} ", item)?
                    }
                    write!(f, "]")?;
                    Ok(())
                }
                CASS_VALUE_TYPE_MAP => {
                    for item in self.get_map().expect("map must be a map") {
                        write!(f, "MAP {}:{}", item.0, item.1)?
                    }
                    Ok(())
                }
                // FIXME
                _ => write!(f, "unknown type"),
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
    //
    //    }

    /// Gets the name of the keyspace.
    #[allow(cast_possible_truncation)]
    pub fn get_bytes(&self) -> Result<&[u8]> {
        unsafe {
            let mut output = mem::zeroed();
            let mut output_size = mem::zeroed();
            let result = cass_value_get_bytes(self.0, &mut output, &mut output_size);
            // raw2utf8(output, output_size).unwrap()
            let slice = slice::from_raw_parts(output, output_size as usize);
            result.to_result(slice).chain_err(|| "")
        }
    }
    // pub fn get_decimal<'a>(&'a self, mut output: String) ->
    // Result<String,CassError> {unsafe{
    // CassError::build(cass_value_get_decimal(self.0,&mut
    // output)).wrap(output)
    //    }}

    /// Get the type of this Cassandra value
    pub fn get_type(&self) -> ValueType { unsafe { ValueType(cass_value_type(self.0)) } }

    /// Get the data type of this Cassandra value
    pub fn data_type(&self) -> ConstDataType { unsafe { ConstDataType(cass_value_data_type(self.0)) } }

    /// Returns true if a specified value is null.
    pub fn is_null(&self) -> bool { unsafe { cass_value_is_null(self.0) == cass_true } }

    /// Returns true if a specified value is a collection.
    pub fn is_collection(&self) -> bool { unsafe { cass_value_is_collection(self.0) == cass_true } }


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

    /// Gets this value as a set iterator.
    pub fn get_set(&self) -> Result<SetIterator> {
        unsafe {
            match self.get_type().0 {
                CASS_VALUE_TYPE_SET => Ok(SetIterator::build(cass_iterator_from_collection(self.0))),
                _ => Err("LIB_INVALID_VALUE_TYPE".into()),
            }
        }
    }

    /// Gets this value as a map iterator.
    pub fn get_map(&self) -> Result<MapIterator> {
        unsafe {
            match self.get_type().0 {
                CASS_VALUE_TYPE_MAP => Ok(MapIterator::build(cass_iterator_from_map(self.0))),
                _ => Err("LIB_INVALID_VALUE_TYPE".into()),
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

    /// Get this value as a string
    #[allow(cast_possible_truncation)]
    pub fn get_string(&self) -> Result<&str> {
        unsafe {
            let message: CString = mem::zeroed();
            let mut message = message.as_ptr();
            let mut message_length = mem::zeroed();
            cass_value_get_string(self.0, &mut message, &mut (message_length));

            let slice = slice::from_raw_parts(message as *const u8, message_length as usize);
            match cass_value_get_string(self.0, &mut message, &mut (message_length)) {
                CASS_OK => str::from_utf8(slice).chain_err(|| ""),
                err => Err(err).chain_err(|| ""),
            }
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

    /// Get this value as an Inet
    pub fn get_inet(&self) -> Result<Inet> {
        unsafe {
            let output: Inet = mem::zeroed();
            cass_value_get_inet(self.0, &mut Inet::inner(&output))
                .to_result(output)
                .chain_err(|| "")
        }
    }

    /// Get this value as an i32
    pub fn get_i32(&self) -> Result<i32> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_int32(self.0, &mut output)
                .to_result(output)
                .chain_err(|| "")
        }
    }

    /// Get this value as an i16
    pub fn get_i16(&self) -> Result<i16> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_int16(self.0, &mut output)
                .to_result(output)
                .chain_err(|| "")
        }
    }

    /// Get this value as an i8
    pub fn get_i8(&self) -> Result<i8> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_int8(self.0, &mut output)
                .to_result(output)
                .chain_err(|| "")
        }
    }

    /// Get this value as an i64
    pub fn get_i64(&self) -> Result<i64> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_int64(self.0, &mut output)
                .to_result(output)
                .chain_err(|| "")
        }
    }

    /// Get this value as a float
    pub fn get_flt(&self) -> Result<f32> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_float(self.0, &mut output)
                .to_result(output)
                .chain_err(|| "")
        }
    }

    /// Get this value as a double
    pub fn get_dbl(&self) -> Result<f64> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_double(self.0, &mut output)
                .to_result(output)
                .chain_err(|| "")
        }
    }

    /// Get this value asa boolean
    pub fn get_bool(&self) -> Result<bool> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_bool(self.0, &mut output)
                .to_result(output == cass_true)
                .chain_err(|| "")
        }
    }

    /// Get this value as a UUID
    pub fn get_uuid(&self) -> Result<Uuid> {
        unsafe {
            let mut uuid = mem::zeroed();
            cass_value_get_uuid(self.0, &mut uuid)
                .to_result(Uuid::build(uuid))
                .chain_err(|| "")
        }
    }
}
