use crate::cassandra::inet::Inet;
use crate::cassandra::iterator::MapIterator;
use crate::cassandra::iterator::SetIterator;
// use decimal::d128;
use crate::cassandra::error::*;
use crate::cassandra::util::Protected;
use crate::cassandra::uuid::Uuid;
use crate::cassandra::value::{write_map, write_set, Value, ValueType};

use crate::cassandra_sys::cass_iterator_from_collection;
use crate::cassandra_sys::cass_iterator_from_map;
use crate::cassandra_sys::cass_true;
use crate::cassandra_sys::cass_value_get_bool;
use crate::cassandra_sys::cass_value_get_decimal;
use crate::cassandra_sys::cass_value_get_double;
use crate::cassandra_sys::cass_value_get_float;
use crate::cassandra_sys::cass_value_get_inet;
use crate::cassandra_sys::cass_value_get_int16;
use crate::cassandra_sys::cass_value_get_int32;
use crate::cassandra_sys::cass_value_get_int64;
use crate::cassandra_sys::cass_value_get_int8;
use crate::cassandra_sys::cass_value_get_string;
use crate::cassandra_sys::cass_value_get_uint32;
use crate::cassandra_sys::cass_value_get_uuid;
use crate::cassandra_sys::cass_value_type;
use crate::cassandra_sys::CASS_ERROR_LIB_INVALID_VALUE_TYPE;
use crate::cassandra_sys::CASS_VALUE_TYPE_ASCII;
use crate::cassandra_sys::CASS_VALUE_TYPE_BIGINT;
use crate::cassandra_sys::CASS_VALUE_TYPE_BLOB;
use crate::cassandra_sys::CASS_VALUE_TYPE_BOOLEAN;
use crate::cassandra_sys::CASS_VALUE_TYPE_COUNTER;
use crate::cassandra_sys::CASS_VALUE_TYPE_CUSTOM;
use crate::cassandra_sys::CASS_VALUE_TYPE_DATE;
use crate::cassandra_sys::CASS_VALUE_TYPE_DECIMAL;
use crate::cassandra_sys::CASS_VALUE_TYPE_DOUBLE;
use crate::cassandra_sys::CASS_VALUE_TYPE_FLOAT;
use crate::cassandra_sys::CASS_VALUE_TYPE_INET;
use crate::cassandra_sys::CASS_VALUE_TYPE_INT;
use crate::cassandra_sys::CASS_VALUE_TYPE_LAST_ENTRY;
use crate::cassandra_sys::CASS_VALUE_TYPE_LIST;
use crate::cassandra_sys::CASS_VALUE_TYPE_MAP;
use crate::cassandra_sys::CASS_VALUE_TYPE_SET;
use crate::cassandra_sys::CASS_VALUE_TYPE_SMALL_INT;
use crate::cassandra_sys::CASS_VALUE_TYPE_TEXT;
use crate::cassandra_sys::CASS_VALUE_TYPE_TIME;
use crate::cassandra_sys::CASS_VALUE_TYPE_TIMESTAMP;
use crate::cassandra_sys::CASS_VALUE_TYPE_TIMEUUID;
use crate::cassandra_sys::CASS_VALUE_TYPE_TINY_INT;
use crate::cassandra_sys::CASS_VALUE_TYPE_TUPLE;
use crate::cassandra_sys::CASS_VALUE_TYPE_UDT;
use crate::cassandra_sys::CASS_VALUE_TYPE_UNKNOWN;
use crate::cassandra_sys::CASS_VALUE_TYPE_UUID;
use crate::cassandra_sys::CASS_VALUE_TYPE_VARCHAR;
use crate::cassandra_sys::CASS_VALUE_TYPE_VARINT;

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
        write!(f, "{} Cassandra type", self.get_type())
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} Cassandra type", self.get_type())
    }
}

impl Field {
    /// Gets the name of this field
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Gets the type of this field
    pub fn get_type(&self) -> ValueType {
        self.value.get_type()
    }

    /// Gets the value of this field
    pub fn get_value(&self) -> &Value {
        &self.value
    }
}
