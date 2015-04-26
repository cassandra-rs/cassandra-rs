#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt::{Debug,Display,Formatter};
use std::fmt;
use std::str;
use std::ffi::CStr;
use std::error::Error;
use std::str::from_utf8;

use libc::types::os::arch::c95::c_char;
use cql_bindgen::cass_error_desc;
use cql_bindgen::CASS_ERROR_LIB_BAD_PARAMS;
use cql_bindgen::CASS_ERROR_LIB_NO_STREAMS;
use cql_bindgen::CASS_ERROR_LAST_ENTRY;
use cql_bindgen::CASS_ERROR_SSL_IDENTITY_MISMATCH;
use cql_bindgen::CASS_ERROR_SSL_INVALID_PEER_CERT;
use cql_bindgen::CASS_ERROR_SSL_NO_PEER_CERT;
use cql_bindgen::CASS_ERROR_SSL_INVALID_PRIVATE_KEY;
use cql_bindgen::CASS_ERROR_SSL_INVALID_CERT;
use cql_bindgen::CASS_ERROR_SERVER_UNPREPARED;
use cql_bindgen::CASS_ERROR_SERVER_ALREADY_EXISTS;
use cql_bindgen::CASS_ERROR_SERVER_CONFIG_ERROR;
use cql_bindgen::CASS_ERROR_SERVER_INVALID_QUERY;
use cql_bindgen::CASS_ERROR_SERVER_UNAUTHORIZED;
use cql_bindgen::CASS_ERROR_SERVER_SYNTAX_ERROR;
use cql_bindgen::CASS_ERROR_SERVER_READ_TIMEOUT;
use cql_bindgen::CASS_ERROR_SERVER_WRITE_TIMEOUT;
use cql_bindgen::CASS_ERROR_SERVER_TRUNCATE_ERROR;
use cql_bindgen::CASS_ERROR_SERVER_IS_BOOTSTRAPPING;
use cql_bindgen::CASS_ERROR_SERVER_OVERLOADED;
use cql_bindgen::CASS_ERROR_SERVER_UNAVAILABLE;
use cql_bindgen::CASS_ERROR_SERVER_BAD_CREDENTIALS;
use cql_bindgen::CASS_ERROR_SERVER_PROTOCOL_ERROR;
use cql_bindgen::CASS_ERROR_SERVER_SERVER_ERROR;
use cql_bindgen::CASS_ERROR_LIB_UNABLE_TO_CLOSE;
use cql_bindgen::CASS_ERROR_LIB_UNABLE_TO_CONNECT;
use cql_bindgen::CASS_ERROR_LIB_NOT_IMPLEMENTED;
use cql_bindgen::CASS_ERROR_LIB_NULL_VALUE;
use cql_bindgen::CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL;
use cql_bindgen::CASS_ERROR_LIB_NAME_DOES_NOT_EXIST;
use cql_bindgen::CASS_ERROR_LIB_INVALID_STATEMENT_TYPE;
use cql_bindgen::CASS_ERROR_LIB_CALLBACK_ALREADY_SET;
use cql_bindgen::CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE;
use cql_bindgen::CASS_ERROR_LIB_REQUEST_TIMED_OUT;
use cql_bindgen::CASS_ERROR_LIB_INVALID_VALUE_TYPE;
use cql_bindgen::CASS_ERROR_LIB_INVALID_ITEM_COUNT;
use cql_bindgen::CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS;
use cql_bindgen::CASS_ERROR_LIB_NO_HOSTS_AVAILABLE;
use cql_bindgen::CASS_ERROR_LIB_WRITE_ERROR;
use cql_bindgen::CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD;
use cql_bindgen::CASS_ERROR_LIB_REQUEST_QUEUE_FULL;
use cql_bindgen::CASS_ERROR_LIB_UNEXPECTED_RESPONSE;
use cql_bindgen::CASS_ERROR_LIB_HOST_RESOLUTION;
use cql_bindgen::CASS_ERROR_LIB_MESSAGE_ENCODE;
use cql_bindgen::CASS_ERROR_LIB_UNABLE_TO_INIT;
use cql_bindgen::CASS_OK;

use cql_bindgen::CassError as _CassError;

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
#[repr(C)]
pub enum CassErrorSource {
    NONE = 0isize,
    LIB = 1,
    SERVER = 2,
    SSL = 3,
    COMPRESSION = 4
}

pub struct CassError(_CassError);

impl Error for CassError {
	fn description(&self) -> &str {
		let c_buf: *const c_char = unsafe { self.desc() };
        let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        from_utf8(buf).unwrap()
    }
}

impl Display for CassError {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        let c_buf: *const c_char = unsafe { self.desc() };
        let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        match str::from_utf8(buf) {
            Ok(str_slice) => {
                write!(f, "{:?}", str_slice)
            },
            Err(err) => panic!("unreachable? {:?}", err)
        }
    }
	
}

impl Debug for CassError {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        let c_buf: *const c_char = unsafe { self.desc() };
        let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        match str::from_utf8(buf) {
            Ok(str_slice) => {
                write!(f, "{:?}", str_slice)
            },
            Err(err) => panic!("unreachable? {:?}", err)
        }
    }
}

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
#[repr(C)]
pub enum CassErrorTypes {
    CASS_OK = 0,
    LIB_BAD_PARAMS = 16777217,
    LIB_NO_STREAMS = 16777218,
    LIB_UNABLE_TO_INIT = 16777219,
    LIB_MESSAGE_ENCODE = 16777220,
    LIB_HOST_RESOLUTION = 16777221,
    LIB_UNEXPECTED_RESPONSE = 16777222,
    LIB_REQUEST_QUEUE_FULL = 16777223,
    LIB_NO_AVAILABLE_IO_THREAD = 16777224,
    LIB_WRITE_ERROR = 16777225,
    LIB_NO_HOSTS_AVAILABLE = 16777226,
    LIB_INDEX_OUT_OF_BOUNDS = 16777227,
    LIB_INVALID_ITEM_COUNT = 16777228,
    LIB_INVALID_VALUE_TYPE = 16777229,
    LIB_REQUEST_TIMED_OUT = 16777230,
    LIB_UNABLE_TO_SET_KEYSPACE = 16777231,
    LIB_CALLBACK_ALREADY_SET = 16777232,
    LIB_INVALID_STATEMENT_TYPE = 16777233,
    LIB_NAME_DOES_NOT_EXIST = 16777234,
    LIB_UNABLE_TO_DETERMINE_PROTOCOL = 16777235,
    LIB_NULL_VALUE = 16777236,
    LIB_NOT_IMPLEMENTED = 16777237,
    LIB_UNABLE_TO_CONNECT = 16777238,
    LIB_UNABLE_TO_CLOSE = 16777239,
    SERVER_SERVER_ERROR = 33554432,
    SERVER_PROTOCOL_ERROR = 33554442,
    SERVER_BAD_CREDENTIALS = 33554688,
    SERVER_UNAVAILABLE = 33558528,
    SERVER_OVERLOADED = 33558529,
    SERVER_IS_BOOTSTRAPPING = 33558530,
    SERVER_TRUNCATE_ERROR = 33558531,
    SERVER_WRITE_TIMEOUT = 33558784,
    SERVER_READ_TIMEOUT = 33559040,
    SERVER_SYNTAX_ERROR = 33562624,
    SERVER_UNAUTHORIZED = 33562880,
    SERVER_INVALID_QUERY = 33563136,
    SERVER_CONFIG_ERROR = 33563392,
    SERVER_ALREADY_EXISTS = 33563648,
    SERVER_UNPREPARED = 33563904,
    SSL_INVALID_CERT = 50331649,
    SSL_INVALID_PRIVATE_KEY = 50331650,
    SSL_NO_PEER_CERT = 50331651,
    SSL_INVALID_PEER_CERT = 50331652,
    SSL_IDENTITY_MISMATCH = 50331653,
    LAST_ENTRY = 50331654
}

impl CassError {
    pub fn wrap<'a,T>(&'a self, wrappee:T) -> Result<T,CassError> {
        match self.0 {
            CASS_OK => Ok(wrappee),
            err => Err(CassError::build(err))
        }
    }
    
    pub fn build(val:u32) -> CassError {
        match val {
            0        => CassError(CASS_OK),
            16777217 => CassError(CASS_ERROR_LIB_BAD_PARAMS),
            16777218 => CassError(CASS_ERROR_LIB_NO_STREAMS),
            16777219 => CassError(CASS_ERROR_LIB_UNABLE_TO_INIT),
            16777220 => CassError(CASS_ERROR_LIB_MESSAGE_ENCODE),
            16777221 => CassError(CASS_ERROR_LIB_HOST_RESOLUTION),
            16777222 => CassError(CASS_ERROR_LIB_UNEXPECTED_RESPONSE),
            16777223 => CassError(CASS_ERROR_LIB_REQUEST_QUEUE_FULL),
            16777224 => CassError(CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD),
            16777225 => CassError(CASS_ERROR_LIB_WRITE_ERROR),
            16777226 => CassError(CASS_ERROR_LIB_NO_HOSTS_AVAILABLE),
            16777227 => CassError(CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS),
            16777228 => CassError(CASS_ERROR_LIB_INVALID_ITEM_COUNT),
            16777229 => CassError(CASS_ERROR_LIB_INVALID_VALUE_TYPE),
            16777230 => CassError(CASS_ERROR_LIB_REQUEST_TIMED_OUT),
            16777231 => CassError(CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE),
            16777232 => CassError(CASS_ERROR_LIB_CALLBACK_ALREADY_SET),
            16777233 => CassError(CASS_ERROR_LIB_INVALID_STATEMENT_TYPE),
            16777234 => CassError(CASS_ERROR_LIB_NAME_DOES_NOT_EXIST),
            16777235 => CassError(CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL),
            16777236 => CassError(CASS_ERROR_LIB_NULL_VALUE),
            16777237 => CassError(CASS_ERROR_LIB_NOT_IMPLEMENTED),
            16777238 => CassError(CASS_ERROR_LIB_UNABLE_TO_CONNECT),
            16777239 => CassError(CASS_ERROR_LIB_UNABLE_TO_CLOSE),
            33554432 => CassError(CASS_ERROR_SERVER_SERVER_ERROR),
            33554442 => CassError(CASS_ERROR_SERVER_PROTOCOL_ERROR),
            33554688 => CassError(CASS_ERROR_SERVER_BAD_CREDENTIALS),
            33558528 => CassError(CASS_ERROR_SERVER_UNAVAILABLE),
            33558529 => CassError(CASS_ERROR_SERVER_OVERLOADED),
            33558530 => CassError(CASS_ERROR_SERVER_IS_BOOTSTRAPPING),
            33558531 => CassError(CASS_ERROR_SERVER_TRUNCATE_ERROR),
            33558784 => CassError(CASS_ERROR_SERVER_WRITE_TIMEOUT),
            33559040 => CassError(CASS_ERROR_SERVER_READ_TIMEOUT),
            33562624 => CassError(CASS_ERROR_SERVER_SYNTAX_ERROR),
            33562880 => CassError(CASS_ERROR_SERVER_UNAUTHORIZED),
            33563136 => CassError(CASS_ERROR_SERVER_INVALID_QUERY),
            33563392 => CassError(CASS_ERROR_SERVER_CONFIG_ERROR),
            33563648 => CassError(CASS_ERROR_SERVER_ALREADY_EXISTS),
            33563904 => CassError(CASS_ERROR_SERVER_UNPREPARED),
            50331649 => CassError(CASS_ERROR_SSL_INVALID_CERT),
            50331650 => CassError(CASS_ERROR_SSL_INVALID_PRIVATE_KEY),
            50331651 => CassError(CASS_ERROR_SSL_NO_PEER_CERT),
            50331652 => CassError(CASS_ERROR_SSL_INVALID_PEER_CERT),
            50331653 => CassError(CASS_ERROR_SSL_IDENTITY_MISMATCH),
            50331654 => CassError(CASS_ERROR_LAST_ENTRY),
            _ => panic!(val)
        }
    }
}


impl CassError {
    pub unsafe fn desc(&self) -> *const c_char {cass_error_desc(self.0)}
    pub fn debug(&self) {println!("{:?}",self)}

}
