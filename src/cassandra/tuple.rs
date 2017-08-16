use cassandra::collection::Set;
use cassandra::data_type::ConstDataType;
use cassandra::data_type::DataType;
use cassandra::inet::AsInet;
use cassandra::user_type::UserType;
use cassandra::util::Protected;
use cassandra::uuid::Uuid;
use cassandra::error::*;

use cassandra_sys::CassTuple as _Tuple;
use cassandra_sys::cass_false;
use cassandra_sys::cass_true;
use cassandra_sys::cass_tuple_data_type;
use cassandra_sys::cass_tuple_free;
use cassandra_sys::cass_tuple_new;
use cassandra_sys::cass_tuple_new_from_data_type;
use cassandra_sys::cass_tuple_set_bool;
use cassandra_sys::cass_tuple_set_bytes;
use cassandra_sys::cass_tuple_set_collection;
use cassandra_sys::cass_tuple_set_decimal;
use cassandra_sys::cass_tuple_set_double;
use cassandra_sys::cass_tuple_set_float;
use cassandra_sys::cass_tuple_set_inet;
use cassandra_sys::cass_tuple_set_int16;
use cassandra_sys::cass_tuple_set_int32;
use cassandra_sys::cass_tuple_set_int64;
use cassandra_sys::cass_tuple_set_int8;
use cassandra_sys::cass_tuple_set_null;
use cassandra_sys::cass_tuple_set_string;
use cassandra_sys::cass_tuple_set_tuple;
use cassandra_sys::cass_tuple_set_uint32;
use cassandra_sys::cass_tuple_set_user_type;
use cassandra_sys::cass_tuple_set_uuid;

use std::ffi::CString;
use std::net::SocketAddr;

/// A tuple of values.
#[derive(Debug)]
pub struct Tuple(*mut _Tuple);

// The underlying C type has no thread-local state, but does not support access
// from multiple threads: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for Tuple {}

impl Protected<*mut _Tuple> for Tuple {
    fn inner(&self) -> *mut _Tuple { self.0 }
    fn build(inner: *mut _Tuple) -> Self { Tuple(inner) }
}

impl Tuple {
    /// Creates a new tuple.
    pub fn new(item_count: usize) -> Self { unsafe { Tuple(cass_tuple_new(item_count)) } }

    /// Creates a new tuple from an existing data type.
    pub fn new_from_data_type(data_type: DataType) -> Tuple {
        unsafe { Tuple(cass_tuple_new_from_data_type(data_type.inner())) }
    }

    /// Gets the data type of a tuple.
    pub fn data_type(&mut self) -> ConstDataType { unsafe { ConstDataType(cass_tuple_data_type(self.0)) } }

    /// Sets an null in a tuple at the specified index.
    pub fn set_null(&mut self, index: usize) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_null(self.0, index)
                .to_result(self)
        }
    }

    /// Sets a "tinyint" in a tuple at the specified index.
    pub fn set_int8(&mut self, index: usize, value: i8) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_int8(self.0, index, value)
                .to_result(self)
        }
    }

    /// Sets an "smallint" in a tuple at the specified index.
    pub fn set_int16(&mut self, index: usize, value: i16) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_int16(self.0, index, value)
                .to_result(self)
        }
    }

    /// Sets an "int" in a tuple at the specified index.
    pub fn set_int32(&mut self, index: usize, value: i32) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_int32(self.0, index, value)
                .to_result(self)
        }
    }

    /// Sets a "date" in a tuple at the specified index.
    pub fn set_uint32(&mut self, index: usize, value: u32) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_uint32(self.0, index, value)
                .to_result(self)
        }
    }

    /// Sets a "bigint", "counter", "timestamp" or "time" in a tuple at the
    /// specified index.
    pub fn set_int64(&mut self, index: usize, value: i64) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_int64(self.0, index, value)
                .to_result(self)
        }
    }

    /// Sets a "float" in a tuple at the specified index.
    pub fn set_float(&mut self, index: usize, value: f32) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_float(self.0, index, value)
                .to_result(self)
        }
    }

    /// Sets a "double" in a tuple at the specified index.
    pub fn set_double(&mut self, index: usize, value: f64) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_double(self.0, index, value)
                .to_result(self)
        }
    }

    /// Sets a "boolean" in a tuple at the specified index.
    pub fn set_bool(&mut self, index: usize, value: bool) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_bool(self.0, index, if value { cass_true } else { cass_false })
                .to_result(self)
        }
    }

    /// Sets an "ascii", "text" or "varchar" in a tuple at the specified index.
    pub fn set_string<S>(&mut self, index: usize, value: S) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_tuple_set_string(self.0,
                                  index,
                                  CString::new(value.into())?.as_ptr())
                .to_result(self)
        }
    }

    /// Sets a "blob", "varint" or "custom" in a tuple at the specified index.
    pub fn set_bytes(&mut self, index: usize, value: Vec<u8>) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_bytes(self.0, index, value.as_ptr(), value.len())
                .to_result(self)
        }
    }

    /// Sets a "uuid" or "timeuuid" in a tuple at the specified index.
    pub fn set_uuid<S>(&mut self, index: usize, value: S) -> Result<&mut Self>
        where S: Into<Uuid> {
        unsafe {
            cass_tuple_set_uuid(self.0, index, value.into().inner())
                .to_result(self)
        }
    }

    /// Sets an "inet" in a tuple at the specified index.
    pub fn set_inet(&mut self, index: usize, value: SocketAddr) -> Result<&mut Self> {
        let inet = AsInet::as_cass_inet(&value);
        unsafe {
            cass_tuple_set_inet(self.0, index, inet.inner())
                .to_result(self)
        }
    }

    /// Sets a "list", "map" or "set" in a tuple at the specified index.
    pub fn set_collection<S>(&mut self, index: usize, value: S) -> Result<&mut Self>
        where S: Into<Set> {
        unsafe {
            cass_tuple_set_collection(self.0, index, value.into().inner())
                .to_result(self)
        }
    }

    /// Sets a "tuple" in a tuple at the specified index.
    pub fn set_tuple(&mut self, index: usize, value: Tuple) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_tuple(self.0, index, value.0)
                .to_result(self)
        }
    }

    /// Sets a "udt" in a tuple at the specified index.
    pub fn set_user_type(&mut self, index: usize, value: &UserType) -> Result<&mut Self> {
        unsafe {
            cass_tuple_set_user_type(self.0, index, value.inner())
                .to_result(self)
        }
    }
}

impl Drop for Tuple {
    fn drop(&mut self) { unsafe { cass_tuple_free(self.0) } }
}
