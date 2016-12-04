use cassandra::error::CassError;
use cassandra::inet::Inet;
use cassandra::iterator::MapIterator;
use cassandra::iterator::SetIterator;
// use decimal::d128;
use cassandra::util::Protected;

use cassandra::uuid::Uuid;
use cassandra::value::{Value, ValueType};
use cassandra_sys::CASS_ERROR_LIB_INVALID_VALUE_TYPE;

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
use cassandra_sys::cass_iterator_from_collection;
use cassandra_sys::cass_iterator_from_map;
use cassandra_sys::cass_true;
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
use cassandra_sys::cass_value_type;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem;
use std::slice;
use std::str;


// #[repr(C)]
// #[derive(Copy,Debug,Clone)]
// #[allow(missing_docs)]
// pub enum FieldType {
//    PARTITION_KEY = 0,
//    CLUSTERING_KEY = 1,
//    REGULAR = 2,
//    COMPACT_VALUE = 3,
//    STATIC = 4,
//    UNKNOWN = 5,
// }

// impl FieldType {
//    //    pub fn build(type_num: u32) -> Result<FieldType, u32> {
//    //        match type_num {
//    //            //            0 => Ok(PARTITION_KEY),
//    //            //            1 => Ok(CLUSTERING_KEY),
//    //            //            2 => Ok(REGULAR),
//    //            //            3 => Ok(COMPACT_VALUE),
//    //            //            4 => Ok(STATIC),
//    //            //            5 => Ok(UNKNOWN),
//    //            err => Err(err),
//    //        }
//    //    }
// }

/// A field's metadata
pub struct Field {
    /// The field's name
    pub name: String,
    /// The field's value
    pub value: Value,
}

impl Debug for Field {
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
            CASS_VALUE_TYPE_INT => write!(f, "INT Cassandra type"),
            CASS_VALUE_TYPE_SMALL_INT => write!(f, "SMALL INT Cassandra type"),
            CASS_VALUE_TYPE_TINY_INT => write!(f, "TINY INT Cassandra type"),
            CASS_VALUE_TYPE_TEXT => write!(f, "TEXT Cassandra type"),
            CASS_VALUE_TYPE_TIMESTAMP => write!(f, "TIMESTAMP Cassandra type"),
            CASS_VALUE_TYPE_TIME => write!(f, "TIME Cassandra type"),
            CASS_VALUE_TYPE_DATE => write!(f, "DATE Cassandra type"),
            CASS_VALUE_TYPE_UUID => write!(f, "UUID Cassandra type"),
            CASS_VALUE_TYPE_VARCHAR => write!(f, "VARCHAR: {:?}", self.get_string()),
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
                    write!(f, "LIST {}-{}", item.0, item.1)?
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

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.get_type().inner() {
            CASS_VALUE_TYPE_UNKNOWN => write!(f, "UNKNOWN Cassandra type"),
            CASS_VALUE_TYPE_CUSTOM => write!(f, "CUSTOM Cassandra type"),
            CASS_VALUE_TYPE_ASCII => write!(f, "ASCII Cassandra type"),
            CASS_VALUE_TYPE_BIGINT => write!(f, "BIGINT Cassandra type"),
            CASS_VALUE_TYPE_BLOB => write!(f, "BLOB Cassandra type"),
            CASS_VALUE_TYPE_BOOLEAN => write!(f, "BOOLEAN Cassandra type"),
            CASS_VALUE_TYPE_DATE => write!(f, "DATE Cassandra type"),
            CASS_VALUE_TYPE_TIME => write!(f, "TIME Cassandra type"),
            CASS_VALUE_TYPE_COUNTER => write!(f, "COUNTER Cassandra type"),
            CASS_VALUE_TYPE_DECIMAL => write!(f, "DECIMAL Cassandra type"),
            CASS_VALUE_TYPE_DOUBLE => write!(f, "DOUBLE Cassandra type"),
            CASS_VALUE_TYPE_FLOAT => write!(f, "FLOAT Cassandra type"),
            CASS_VALUE_TYPE_INT => write!(f, "INT Cassandra type"),
            CASS_VALUE_TYPE_SMALL_INT => write!(f, "SMALL_INT Cassandra type"),
            CASS_VALUE_TYPE_TINY_INT => write!(f, "TINY_INT Cassandra type"),
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
                    write!(f, "MAP {}-{}", item.0, item.1)?
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

// trait AsTypedColumn {
//    type T;
//    fn get(col: Column) -> Result<Self::T, CassError>;
// }
//
// impl AsTypedColumn for bool {
//    type T = Self;
//    fn get(col: Column) -> Result<Self, CassError> { col.get_bool() }
// }
//

impl Field {
    /// Gets the name of this field
    pub fn get_name(&self) -> String { self.name.clone() }

    /// Gets the type of this field
    pub fn get_type(&self) -> ValueType { unsafe { ValueType::build(cass_value_type(self.value.inner())) } }

    /// Gets the value of an inet field
    pub fn get_inet(&self) -> Result<Inet, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_inet(self.value.inner(), &mut output)).wrap(Inet::build(output))
        }
    }

    /// Gets the value of an u32 field
    pub fn get_u32(&self, mut output: u32) -> Result<u32, CassError> {
        unsafe { CassError::build(cass_value_get_uint32(self.value.inner(), &mut output)).wrap(output) }
    }

    /// Gets the value of an i8 field
    pub fn get_int8(&self, mut output: i8) -> Result<i8, CassError> {
        unsafe { CassError::build(cass_value_get_int8(self.value.inner(), &mut output)).wrap(output) }
    }

    /// Gets the value of an i16 field
    pub fn get_int16(&self, mut output: i16) -> Result<i16, CassError> {
        unsafe { CassError::build(cass_value_get_int16(self.value.inner(), &mut output)).wrap(output) }
    }
    //    //    pub fn get_decimal(&self, mut output: d128) -> Result<d128, CassError> {
    //    //        let _ = output;
    //    //        unimplemented!()
    //    //        // unsafe { CassError::build(cass_value_get_decimal(self.0, &mut output)).wrap(output) }
    //    //    }
    //

    /// Gets the value of an ASCII, Text, or Varchar field
    #[allow(cast_possible_truncation)]
    pub fn get_string(&self) -> Result<String, CassError> {
        unsafe {
            match cass_value_type(self.value.inner()) {
                CASS_VALUE_TYPE_ASCII |
                CASS_VALUE_TYPE_TEXT |
                CASS_VALUE_TYPE_VARCHAR => {
                    let mut message = mem::zeroed();
                    let mut message_length = mem::zeroed();
                    match cass_value_get_string(self.value.inner(), &mut message, &mut message_length) {
                        CASS_OK => {
                            let slice = slice::from_raw_parts(message as *const u8, message_length as usize);
                            Ok(str::from_utf8(slice).expect("must be utf8").to_owned())
                        }
                        err => Err(CassError::build(err)),
                    }


                }
                other => panic!("unsupported type: {:?}", other), //FIXME
            }
        }
    }

    /// Gets the value of an i32 field
    pub fn get_int32(&self) -> Result<i32, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_int32(self.value.inner(), &mut output)).wrap(output)
        }
    }

    /// Gets the value of an i64 field
    pub fn get_int64(&self) -> Result<i64, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_int64(self.value.inner(), &mut output)).wrap(output)
        }
    }

    /// Gets the value of a float field
    pub fn get_float(&self) -> Result<f32, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_float(self.value.inner(), &mut output)).wrap(output)
        }
    }

    /// Gets the value of a double field
    pub fn get_double(&self) -> Result<f64, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_double(self.value.inner(), &mut output)).wrap(output)
        }
    }

    /// Gets the value of a bool field
    pub fn get_bool(&self) -> Result<bool, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_bool(self.value.inner(), &mut output)).wrap(output == cass_true)
        }
    }

    /// Gets the value of a uuid field
    pub fn get_uuid(&self) -> Result<Uuid, CassError> {
        unsafe {
            let mut uuid = mem::zeroed();
            CassError::build(cass_value_get_uuid(self.value.inner(), &mut uuid)).wrap(Uuid::build(uuid))
        }
    }

    /// Gets the value of a map field as an iterator
    pub fn map_iter(&self) -> Result<MapIterator, CassError> {
        unsafe {
            match self.get_type().inner() {
                CASS_VALUE_TYPE_MAP => Ok(MapIterator::build(cass_iterator_from_map(self.value.inner()))),
                _ => Err(CassError::build(CASS_ERROR_LIB_INVALID_VALUE_TYPE)),
            }
        }
    }

    /// Gets the value of a set field as an iterator
    pub fn set_iter(&self) -> Result<SetIterator, CassError> {
        unsafe {
            match self.get_type().inner() {
                CASS_VALUE_TYPE_SET => Ok(SetIterator::build(cass_iterator_from_collection(self.value.inner()))),
                _ => Err(CassError::build(CASS_ERROR_LIB_INVALID_VALUE_TYPE)),
            }
        }
    }

    //        pub fn use_type_iter(&self) -> Result<UserTypeIterator, CassError> {
    //            unsafe {
    //                match self.get_type() {
    //                    ValueType::UDT => Ok(UserTypeIterator(cass_iterator_from_user_type(self.0))),
    //                    _ => Err(CassError::build(1)),
    //                }
    //            }
    //        }
}
