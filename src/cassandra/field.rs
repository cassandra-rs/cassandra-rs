use cassandra::inet::Inet;
use cassandra::iterator::MapIterator;
use cassandra::iterator::SetIterator;
// use decimal::d128;
use cassandra::util::Protected;
use cassandra::uuid::Uuid;
use cassandra::value::{Value, ValueType, write_set, write_map};
use cassandra::error::*;

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
use cassandra_sys::cass_iterator_from_collection;
use cassandra_sys::cass_iterator_from_map;
use cassandra_sys::cass_true;
use cassandra_sys::cass_value_get_bool;
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
    pub fn get_name(&self) -> String { self.name.clone() }

    /// Gets the type of this field
    pub fn get_type(&self) -> ValueType { self.value.get_type() }

    /// Gets the value of this field
    pub fn get_value(&self) -> &Value { &self.value }
}
