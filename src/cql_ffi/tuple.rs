use cql_bindgen::cass_tuple_new;
use cql_bindgen::cass_tuple_new_from_data_type;
use cql_bindgen::cass_tuple_free;
use cql_bindgen::cass_tuple_data_type;
use cql_bindgen::cass_tuple_set_null;
use cql_bindgen::cass_tuple_set_int32;
use cql_bindgen::cass_tuple_set_int64;
use cql_bindgen::cass_tuple_set_float;
use cql_bindgen::cass_tuple_set_double;
use cql_bindgen::cass_tuple_set_bool;
use cql_bindgen::cass_tuple_set_string;
//use cql_bindgen::cass_tuple_set_string_n;
use cql_bindgen::cass_tuple_set_bytes;
use cql_bindgen::cass_tuple_set_uuid;
use cql_bindgen::cass_tuple_set_inet;
//use cql_bindgen::cass_tuple_set_decimal;
//use cql_bindgen::cass_tuple_set_collection;
//use cql_bindgen::cass_tuple_set_tuple;
//use cql_bindgen::cass_tuple_set_user_type;
//use cql_bindgen::cass_iterator_from_tuple;

use std::ffi::CString;

use cql_ffi::inet::AsCassInet;
use std::net::SocketAddr;
use cql_bindgen::CassTuple as _CassTuple;
use cql_ffi::uuid::CassUuid;
use cql_ffi::udt::CassDataType;
use cql_ffi::udt::CassConstDataType;
use cql_ffi::error::CassError;

pub struct CassTuple(pub *mut _CassTuple);

impl CassTuple {
    pub fn new(item_count: u64) -> Self {
        unsafe {
            CassTuple(cass_tuple_new(item_count))
        }
    }

    pub fn data_type(&mut self) -> CassConstDataType {
        unsafe {
            CassConstDataType(cass_tuple_data_type(self.0))
        }
    }

    pub fn new_from_data_type(data_type: CassDataType) -> CassTuple {
        unsafe {
            CassTuple(cass_tuple_new_from_data_type(data_type.0))
        }
    }

    pub fn set_null(&mut self, index: u64) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_tuple_set_null(self.0, index)
            ).wrap(())
        }
    }

    pub fn set_int32(&mut self, index: u64, value: i32) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_tuple_set_int32(self.0, index, value)
            ).wrap(())
        }
    }

    pub fn set_int64(&mut self, index: u64, value: i64) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_tuple_set_int64(self.0, index, value)
            ).wrap(())
        }
    }

    pub fn set_float(&mut self, index: u64, value: f32) -> Result<(), CassError> {
        unsafe {
            CassError::build(cass_tuple_set_float(self.0, index, value)).wrap(())
        }
    }

    pub fn set_double(&mut self, index: u64, value: f64) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_tuple_set_double(self.0, index, value)
            ).wrap(())
        }
    }

    pub fn set_bool(&mut self, index: u64, value: bool) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_tuple_set_bool(
                    self.0,
                    index,
                    if value {1} else {0}
                )
            ).wrap(())
        }
    }

    pub fn set_string<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let value = CString::new(value.into()).unwrap();
            CassError::build(
                cass_tuple_set_string(self.0, index, value.as_ptr())
            ).wrap(())
        }
    }

    pub fn set_inet(&mut self, index: u64, value: SocketAddr) -> Result<(), CassError> {
        let inet = AsCassInet::as_cass_inet(&value);
        unsafe {
            CassError::build(
                cass_tuple_set_inet(
                    self.0,
                    index,
                    inet.0,
                )
            ).wrap(())
        }
    }

    pub fn set_bytes(&mut self, index: u64, value: Vec<u8>) -> Result<(), CassError> {
        unsafe {
            CassError::build(
                cass_tuple_set_bytes(
                    self.0,
                    index,
                    value.as_ptr(),
                    value.len() as u64
                )
            ).wrap(())
        }
    }

    pub fn set_uuid<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<CassUuid>
    {
        unsafe {
            CassError::build(
                cass_tuple_set_uuid(self.0, index, value.into().0)
            ).wrap(())
        }
    }
}

impl Drop for CassTuple {
    fn drop(&mut self) {
        unsafe {
            cass_tuple_free(self.0)
        }
    }
}
