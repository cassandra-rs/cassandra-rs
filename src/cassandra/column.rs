// use cassandra::error::CassError;

use cassandra::inet::Inet;
use cassandra::iterator::MapIterator;
use cassandra::iterator::SetIterator;
use cassandra::iterator::UserTypeIterator;
use cassandra::util::Protected;
use cassandra::uuid::Uuid;
use cassandra::value::ValueType;
use cassandra_sys::CASS_ERROR_LIB_INVALID_VALUE_TYPE;

// use cassandra_sys::Enum_CassValueType_::*;
use cassandra_sys::CASS_OK;
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
use cassandra_sys::CassValue as _Value;
use cassandra_sys::cass_iterator_fields_from_user_type;
use cassandra_sys::cass_iterator_from_collection;
use cassandra_sys::cass_iterator_from_map;
use cassandra_sys::{cass_true, cass_false};
use cassandra_sys::cass_value_get_bool;

#[allow(unused_imports)]
use cassandra_sys::cass_value_get_decimal;
use cassandra_sys::cass_value_get_double;
use cassandra_sys::cass_value_get_float;
use cassandra_sys::cass_value_get_inet;
use cassandra_sys::cass_value_get_int16;
use cassandra_sys::cass_value_get_int32;
use cassandra_sys::cass_value_get_int64;
use cassandra_sys::cass_value_get_int8;
use cassandra_sys::cass_value_get_string;
use cassandra_sys::cass_value_get_uint32;
use cassandra_sys::cass_value_get_uuid;
use cassandra_sys::cass_value_is_null;
use cassandra_sys::cass_value_type;
use errors::*;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem;
use std::slice;
use std::str;
// use decimal::d128;

// #[repr(C)]
// #[derive(Copy,Debug,Clone)]
//  ///The type of Cassandra column being referenced
// pub enum ColumnType {
//    ///A Cassandra partition key column
//    PARTITION_KEY = 0,
//    ///A Cassandra clustering key column
//    CLUSTERING_KEY = 1,
//    ///A "normal" column
//    REGULAR = 2,
//    ///For compact tables?
//    COMPACT_VALUE = 3,
//    ///A Cassandra static column
//    STATIC = 4,
//    ///An unknown column type. FIXME not sure if ever used
//    UNKNOWN = 5,
// }

/// Representation of a Cassandra column
pub struct Column(*const _Value);

impl Protected<*const _Value> for Column {
    fn inner(&self) -> *const _Value { self.0 }
    fn build(inner: *const _Value) -> Self { Column(inner) }
}

impl Debug for Column {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.get_type().inner() {
            CASS_VALUE_TYPE_UNKNOWN => write!(f, "UNKNOWN Cassandra type"),
            CASS_VALUE_TYPE_CUSTOM => write!(f, "CUSTOM Cassandra type"),
            CASS_VALUE_TYPE_ASCII => write!(f, "ASCII Cassandra type"),
            CASS_VALUE_TYPE_BIGINT => write!(f, "BIGINT Cassandra type"),
            CASS_VALUE_TYPE_BLOB => write!(f, "BLOB Cassandra type"),
            CASS_VALUE_TYPE_BOOLEAN => write!(f, "BOOLEAN Cassandra type"),
            CASS_VALUE_TYPE_COUNTER => write!(f, "COUNTER Cassandra type"),
            CASS_VALUE_TYPE_DATE => write!(f, "DATE Cassandra type"),
            CASS_VALUE_TYPE_TIME => write!(f, "TIME Cassandra type"),
            CASS_VALUE_TYPE_DECIMAL => write!(f, "DECIMAL Cassandra type"),
            CASS_VALUE_TYPE_DOUBLE => write!(f, "DOUBLE Cassandra type"),
            CASS_VALUE_TYPE_FLOAT => write!(f, "FLOAT Cassandra type"),
            CASS_VALUE_TYPE_INT => write!(f, "INT Cassandra type"),
            CASS_VALUE_TYPE_SMALL_INT => write!(f, "SMALL_INT Cassandra type"),
            CASS_VALUE_TYPE_TINY_INT => write!(f, "TINY_INT Cassandra type"),
            CASS_VALUE_TYPE_TEXT => write!(f, "TEXT Cassandra type"),
            CASS_VALUE_TYPE_TIMESTAMP => write!(f, "TIMESTAMP Cassandra type"),
            CASS_VALUE_TYPE_UUID => write!(f, "UUID Cassandra type"),
            CASS_VALUE_TYPE_VARCHAR => write!(f, "VARCHAR: {:?}", self.get_string()),
            CASS_VALUE_TYPE_VARINT => Ok(()),
            CASS_VALUE_TYPE_TIMEUUID => write!(f, "TIMEUUID Cassandra type"),
            CASS_VALUE_TYPE_INET => write!(f, "INET Cassandra type"),
            CASS_VALUE_TYPE_LIST => {
                for item in self.set_iter().expect("a list should always be able to return a set iterator") {
                    write!(f, "LIST {:?}", item)?
                }
                Ok(())
            }
            CASS_VALUE_TYPE_MAP => {
                for item in self.map_iter().expect("a map should always be able to return a map iterator") {
                    write!(f, "LIST {:?}", item)?
                }
                Ok(())
            }
            CASS_VALUE_TYPE_SET => {
                for item in self.set_iter().expect("a set must be able to return a set iterator") {
                    write!(f, "SET {:?}", item)?
                }
                Ok(())
            }
            CASS_VALUE_TYPE_UDT => write!(f, "UDT Cassandra type"),
            CASS_VALUE_TYPE_TUPLE => write!(f, "Tuple Cassandra type"),
            CASS_VALUE_TYPE_LAST_ENTRY => write!(f, "LAST_ENTRY Cassandra type"),
        }
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.get_type().inner() {
            CASS_VALUE_TYPE_UNKNOWN => write!(f, "UNKNOWN Cassandra type"),
            CASS_VALUE_TYPE_CUSTOM => write!(f, "CUSTOM Cassandra type"),
            CASS_VALUE_TYPE_ASCII => write!(f, "ASCII Cassandra type"),
            CASS_VALUE_TYPE_BIGINT => write!(f, "BIGINT Cassandra type"),
            CASS_VALUE_TYPE_BLOB => write!(f, "BLOB Cassandra type"),
            CASS_VALUE_TYPE_BOOLEAN => write!(f, "BOOLEAN Cassandra type"),
            CASS_VALUE_TYPE_COUNTER => write!(f, "COUNTER Cassandra type"),
            CASS_VALUE_TYPE_DECIMAL => write!(f, "DECIMAL Cassandra type"),
            CASS_VALUE_TYPE_DOUBLE => write!(f, "DOUBLE Cassandra type"),
            CASS_VALUE_TYPE_FLOAT => write!(f, "FLOAT Cassandra type"),
            CASS_VALUE_TYPE_DATE => write!(f, "DATE Cassandra type"),
            CASS_VALUE_TYPE_TIME => write!(f, "TIME Cassandra type"),
            CASS_VALUE_TYPE_INT => write!(f, "INT Cassandra type"),
            CASS_VALUE_TYPE_SMALL_INT => write!(f, "SMALL INT Cassandra type"),
            CASS_VALUE_TYPE_TINY_INT => write!(f, "TINY INT Cassandra type"),
            CASS_VALUE_TYPE_TEXT => write!(f, "TEXT Cassandra type"),
            CASS_VALUE_TYPE_TIMESTAMP => write!(f, "TIMESTAMP Cassandra type"),
            CASS_VALUE_TYPE_UUID => write!(f, "UUID Cassandra type"),
            CASS_VALUE_TYPE_VARCHAR => write!(f, "{}", self.get_string().unwrap()),
            CASS_VALUE_TYPE_VARINT => Ok(()),
            CASS_VALUE_TYPE_TIMEUUID => write!(f, "TIMEUUID Cassandra type"),
            CASS_VALUE_TYPE_INET => write!(f, "INET Cassandra type"),
            CASS_VALUE_TYPE_LIST => {
                for item in self.set_iter().expect("a list must be able to return a set iterator") {
                    write!(f, "LIST {}", item)?
                }
                Ok(())
            }
            CASS_VALUE_TYPE_MAP => {
                for item in self.map_iter().expect("a map must be able to return a map iterator") {
                    write!(f, "MAP {}:{}", item.0, item.1)?
                }
                Ok(())
            }
            CASS_VALUE_TYPE_SET => {
                for item in self.set_iter().expect("a set must be able to return a set iterator") {
                    write!(f, "SET {}", item)?
                }
                Ok(())
            }
            CASS_VALUE_TYPE_UDT => write!(f, "UDT Cassandra type"),
            CASS_VALUE_TYPE_TUPLE => write!(f, "Tuple Cassandra type"),
            CASS_VALUE_TYPE_LAST_ENTRY => write!(f, "LAST_ENTRY Cassandra type"),
        }
    }
}

// pub trait AsTypedColumn {
//    type T;
//    fn get(T) -> Result<Self::T, CassError>;
// }

impl From<Column> for Result<bool> {
    fn from(col: Column) -> Result<bool> { col.get_bool() }
}

// impl Into<Result<bool,CassError>> for Column {
//    fn into(self) -> Result<bool,CassError> {
//        self.get_bool()
//    }
// }

// impl From<Column> for bool {
//    fn from(col:Column) -> bool {
//        col.get_bool().unwrap()
//    }
// }


impl Column {
    /// Gets the type of this column.
    pub fn get_type(&self) -> ValueType { unsafe { ValueType::build(cass_value_type(self.0)) } }

    /// Gets the inet from this column or errors if you ask for the wrong type
    pub fn get_inet(&self) -> Result<Inet> {
        unsafe {
            let mut inet = mem::zeroed();
            cass_value_get_inet(self.0, &mut inet).to_result(Inet::build(inet)).chain_err(|| "")
        }
    }

    /// Gets the u32 from this column or errors if you ask for the wrong type
    pub fn get_u32(&self, mut output: u32) -> Result<u32> {
        unsafe { cass_value_get_uint32(self.0, &mut output).to_result(output).chain_err(|| "") }
    }

    /// Gets the i8 from this column or errors if you ask for the wrong type
    pub fn get_i8(&self, mut output: i8) -> Result<i8> {
        unsafe { cass_value_get_int8(self.0, &mut output).to_result(output).chain_err(|| "") }
    }

    /// Gets the i16 from this column or errors if you ask for the wrong type
    pub fn get_i16(&self, mut output: i16) -> Result<i16> {
        unsafe { cass_value_get_int16(self.0, &mut output).to_result(output).chain_err(|| "") }
    }

    //    pub fn get_decimal(&self, mut output: d128) -> Result<d128, CassError> {
    //        let _ = output;
    //        unimplemented!()
    //        // unsafe { CassError::build(cass_value_get_decimal(self.0, &mut output)).wrap(output) }
    //    }

    /// Gets the string from this column or errors if you ask for the wrong type
    #[allow(cast_possible_truncation)]
    pub fn get_string(&self) -> Result<String> {
        unsafe {
            match cass_value_type(self.0) {
                CASS_VALUE_TYPE_ASCII |
                CASS_VALUE_TYPE_TEXT |
                CASS_VALUE_TYPE_VARCHAR => {
                    let mut message = mem::zeroed();
                    let mut message_length = mem::zeroed();
                    match cass_value_get_string(self.0, &mut message, &mut message_length) {
                        CASS_OK => {
                            let slice = slice::from_raw_parts(message as *const u8, message_length as usize);
                            Ok(str::from_utf8(slice).expect("must be utf8").to_owned())
                        }
                        err => Err(err.to_result("").unwrap().into()),
                    }


                }
                other => panic!("Unsupported type: {:?}", other), //FIXME
            }
        }
    }

    /// Gets the blog from this column or errors if type if wrong
    pub fn get_blob(&self) -> Result<Vec<u8>> {
        unsafe {
            match cass_value_type(self.0) {
                CASS_VALUE_TYPE_BLOB => {
                    let mut message = mem::zeroed();
                    let mut message_length = mem::zeroed();
                    match cass_value_get_string(self.0, &mut message, &mut message_length) {
                        CASS_OK => Ok(Vec::from(slice::from_raw_parts(message as *const u8, message_length as usize))),
                        err => Err(err.to_result("").unwrap().into()),
                    }
                }
                _ => panic!("Value type is not BLOB"),
            }
        }
    }

    /// Gets the i32 from this column or errors if you ask for the wrong type
    pub fn get_i32(&self) -> Result<i32> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_int32(self.0, &mut output).to_result(output).chain_err(|| "")
        }
    }

    /// Gets the i64 from this column or errors if you ask for the wrong type
    pub fn get_i64(&self) -> Result<i64> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_int64(self.0, &mut output).to_result(output).chain_err(|| "")
        }
    }

    /// Gets the float from this column or errors if you ask for the wrong type
    pub fn get_float(&self) -> Result<f32> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_float(self.0, &mut output).to_result(output).chain_err(|| "")
        }
    }

    /// Gets the double from this column or errors if you ask for the wrong type
    pub fn get_double(&self) -> Result<f64> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_double(self.0, &mut output).to_result(output).chain_err(|| "")
        }
    }

    /// Gets the bool from this column or errors if you ask for the wrong type
    pub fn get_bool(&self) -> Result<bool> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_bool(self.0, &mut output)
                .to_result(output != cass_false)
                .chain_err(|| "")
        }
    }

    /// Gets the uuid from this column or errors if you ask for the wrong type
    pub fn get_uuid(&self) -> Result<Uuid> {
        unsafe {
            let mut output = mem::zeroed();
            cass_value_get_uuid(self.0, &mut output).to_result(Uuid::build(output)).chain_err(|| "")
        }
    }

    /// Tests whether the column is null.
    pub fn is_null(&self) -> bool {
        unsafe {
            cass_value_is_null(self.0) != cass_false
        }
    }

    /// Gets an iterator over the map in this column or errors if you ask for the wrong type
    pub fn map_iter(&self) -> Result<MapIterator> {
        unsafe {
            match self.get_type().inner() {
                CASS_VALUE_TYPE_MAP => Ok(MapIterator::build(cass_iterator_from_map(self.0))),
                _ => Err(CASS_ERROR_LIB_INVALID_VALUE_TYPE.to_result("").unwrap().into()),
            }
        }
    }

    /// Gets an iterator over the set in this column or errors if you ask for the wrong type
    pub fn set_iter(&self) -> Result<SetIterator> {
        unsafe {
            match self.get_type().inner() {
                CASS_VALUE_TYPE_SET => Ok(SetIterator::build(cass_iterator_from_collection(self.0))),
                _ => Err(CASS_ERROR_LIB_INVALID_VALUE_TYPE.to_result("").unwrap().into()),
            }
        }
    }

    /// Gets an iterator over the fields of the user type in this column or errors if you ask for the wrong type
    pub fn use_type_iter(&self) -> Result<UserTypeIterator> {
        unsafe {
            match self.get_type().inner() {
                CASS_VALUE_TYPE_UDT => Ok(UserTypeIterator::build(cass_iterator_fields_from_user_type(self.0))),
                _ => Err(CASS_ERROR_LIB_INVALID_VALUE_TYPE.to_result("").unwrap().into()),
            }
        }
    }
}
