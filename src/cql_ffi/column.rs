use std::mem;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::str;
use std::slice;

use cql_bindgen::CASS_VALUE_TYPE_ASCII;
use cql_bindgen::CASS_VALUE_TYPE_VARCHAR;
use cql_bindgen::CASS_VALUE_TYPE_TEXT;
use cql_bindgen::CASS_OK;
use cql_bindgen::cass_value_get_int32;
use cql_bindgen::cass_value_get_int64;
use cql_bindgen::cass_value_get_float;
use cql_bindgen::cass_value_get_double;
use cql_bindgen::cass_value_get_bool;
use cql_bindgen::cass_value_get_uuid;
use cql_bindgen::cass_value_get_string;
use cql_bindgen::cass_value_get_inet;
use cql_bindgen::cass_iterator_from_map;
use cql_bindgen::cass_iterator_from_user_type;
use cql_bindgen::cass_iterator_from_collection;
use cql_bindgen::cass_value_type;
use cql_bindgen::CassValue as _Value;

use cql_ffi::uuid::Uuid;
//use cql_ffi::udt::UserType;
use cql_ffi::value::ValueType;
use cql_ffi::collection::set::SetIterator;
use cql_ffi::inet::Inet;
use cql_ffi::collection::map::MapIterator;
use cql_ffi::udt::UserTypeIterator;
use cql_ffi::error::CassandraErrorTypes;
use cql_ffi::error::CassandraError;

#[repr(C)]
#[derive(Copy,Debug,Clone)]
pub enum ColumnType {
    PARTITION_KEY = 0,
    CLUSTERING_KEY = 1,
    REGULAR = 2,
    COMPACT_VALUE = 3,
    STATIC = 4,
    UNKNOWN = 5,
}

pub struct Column(pub *const _Value);

impl Debug for Column {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.get_type() {
            ValueType::UNKNOWN => write!(f, "UNKNOWN Cassandra type"),
            ValueType::CUSTOM => write!(f, "CUSTOM Cassandra type"),
            ValueType::ASCII => write!(f, "ASCII Cassandra type"),
            ValueType::BIGINT => write!(f, "BIGINT Cassandra type"),
            ValueType::BLOB => write!(f, "BLOB Cassandra type"),
            ValueType::BOOLEAN => write!(f, "BOOLEAN Cassandra type"),
            ValueType::COUNTER => write!(f, "COUNTER Cassandra type"),
            ValueType::DECIMAL => write!(f, "DECIMAL Cassandra type"),
            ValueType::DOUBLE => write!(f, "DOUBLE Cassandra type"),
            ValueType::FLOAT => write!(f, "FLOAT Cassandra type"),
            ValueType::INT => write!(f, "INT Cassandra type"),
            ValueType::TEXT => write!(f, "TEXT Cassandra type"),
            ValueType::TIMESTAMP => write!(f, "TIMESTAMP Cassandra type"),
            ValueType::UUID => write!(f, "UUID Cassandra type"),
            ValueType::VARCHAR => write!(f, "VARCHAR: {:?}", self.get_string()),
            ValueType::VARINT => Ok(()),
            ValueType::TIMEUUID => write!(f, "TIMEUUID Cassandra type"),
            ValueType::INET => write!(f, "INET Cassandra type"),
            ValueType::LIST => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item ))
                }
                Ok(())
            }
            ValueType::MAP => {
                for item in self.map_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item ))
                }
                Ok(())
            }
            ValueType::SET => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "SET {:?}", item ))
                }
                Ok(())
            }
            ValueType::UDT => write!(f, "UDT Cassandra type"),
            ValueType::TUPLE => write!(f, "Tuple Cassandra type"),
            ValueType::LASTENTRY => write!(f, "LAST_ENTRY Cassandra type"),
        }
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.get_type() {
            ValueType::UNKNOWN => write!(f, "UNKNOWN Cassandra type"),
            ValueType::CUSTOM => write!(f, "CUSTOM Cassandra type"),
            ValueType::ASCII => write!(f, "ASCII Cassandra type"),
            ValueType::BIGINT => write!(f, "BIGINT Cassandra type"),
            ValueType::BLOB => write!(f, "BLOB Cassandra type"),
            ValueType::BOOLEAN => write!(f, "BOOLEAN Cassandra type"),
            ValueType::COUNTER => write!(f, "COUNTER Cassandra type"),
            ValueType::DECIMAL => write!(f, "DECIMAL Cassandra type"),
            ValueType::DOUBLE => write!(f, "DOUBLE Cassandra type"),
            ValueType::FLOAT => write!(f, "FLOAT Cassandra type"),
            ValueType::INT => write!(f, "INT Cassandra type"),
            ValueType::TEXT => write!(f, "TEXT Cassandra type"),
            ValueType::TIMESTAMP => write!(f, "TIMESTAMP Cassandra type"),
            ValueType::UUID => write!(f, "UUID Cassandra type"),
            ValueType::VARCHAR => write!(f, "{}", self.get_string().unwrap()),
            ValueType::VARINT => Ok(()),
            ValueType::TIMEUUID => write!(f, "TIMEUUID Cassandra type"),
            ValueType::INET => write!(f, "INET Cassandra type"),
            ValueType::LIST => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item ))
                }
                Ok(())
            }
            ValueType::MAP => {
                for item in self.map_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item ))
                }
                Ok(())
            }
            ValueType::SET => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "SET {:?}", item ))
                }
                Ok(())
            }
            ValueType::UDT => write!(f, "UDT Cassandra type"),
            ValueType::TUPLE => write!(f, "Tuple Cassandra type"),
            ValueType::LASTENTRY => write!(f, "LAST_ENTRY Cassandra type"),
        }
    }
}

trait AsTypedColumn {
    type T;
    fn get(col: Column) -> Result<Self::T, CassandraError>;
}

impl AsTypedColumn for bool {
    type T = Self;
    fn get(col: Column) -> Result<Self, CassandraError> {
        col.get_bool()
    }
}

impl Column {
    pub fn get_type(&self) -> ValueType {
        unsafe {
            ValueType::build(cass_value_type(self.0))
        }
    }

    pub unsafe fn get_inet(&self, mut output: Inet) -> Result<Inet, CassandraError> {
        CassandraError::build(cass_value_get_inet(self.0,&mut output.0)).wrap(output)
    }

    pub fn get_string(&self) -> Result<String, CassandraError> {
        unsafe {
            match cass_value_type(self.0) {
                CASS_VALUE_TYPE_ASCII | CASS_VALUE_TYPE_TEXT | CASS_VALUE_TYPE_VARCHAR => {
                    let mut message = mem::zeroed();
                    let mut message_length = mem::zeroed();
                    match cass_value_get_string(self.0, &mut message, &mut message_length) {
                        CASS_OK => {
                            let slice = slice::from_raw_parts(message as *const u8,
                                                              message_length as usize);
                            Ok(str::from_utf8(slice).unwrap().to_owned())
                        }
                        err => Err(CassandraError::build(err)),
                    }


                }
                err => Err(CassandraError::build(err)),
            }
        }
    }

    pub fn get_int32(&self) -> Result<i32, CassandraError> {
        unsafe {
            let mut output = mem::zeroed();
            CassandraError::build(cass_value_get_int32(self.0,&mut output)).wrap(output)
        }
    }

    pub fn get_int64(&self) -> Result<i64, CassandraError> {
        unsafe {
            let mut output = mem::zeroed();
            CassandraError::build(cass_value_get_int64(self.0,&mut output)).wrap(output)
        }
    }

    pub fn get_float(&self) -> Result<f32, CassandraError> {
        unsafe {
            let mut output = mem::zeroed();
            CassandraError::build(cass_value_get_float(self.0,&mut output)).wrap(output)
        }
    }

    pub fn get_double(&self) -> Result<f64, CassandraError> {
        unsafe {
            let mut output = mem::zeroed();
            CassandraError::build(cass_value_get_double(self.0,&mut output)).wrap(output)
        }
    }

    pub fn get_bool(&self) -> Result<bool, CassandraError> {
        unsafe {
            let mut output = mem::zeroed();
            CassandraError::build(cass_value_get_bool(self.0,&mut output)).wrap(output > 0)
        }
    }

    pub fn get_uuid(&self) -> Result<Uuid, CassandraError> {
        unsafe {
            let mut output: Uuid = mem::zeroed();
            CassandraError::build(cass_value_get_uuid(self.0,&mut output.0)).wrap(output)
        }
    }

    pub fn map_iter(&self) -> Result<MapIterator, CassandraError> {
        unsafe {
            match self.get_type() {
                ValueType::MAP => Ok(MapIterator(cass_iterator_from_map(self.0))),
                _ => Err(CassandraError::build(CassandraErrorTypes::LIB_INVALID_VALUE_TYPE as u32)),
            }
        }
    }

    pub fn set_iter(&self) -> Result<SetIterator, CassandraError> {
        unsafe {
            match self.get_type() {
                ValueType::SET => Ok(SetIterator(cass_iterator_from_collection(self.0))),
                _ => Err(CassandraError::build(1)),
            }
        }
    }

    pub fn use_type_iter(&self) -> Result<UserTypeIterator, CassandraError> {
        unsafe {
            match self.get_type() {
                ValueType::UDT => Ok(UserTypeIterator(cass_iterator_from_user_type(self.0))),
                _ => Err(CassandraError::build(1)),
            }
        }
    }


}
