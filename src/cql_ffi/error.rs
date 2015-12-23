use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::ffi::{CStr, CString, NulError};
use std::error::Error;
use std::str::from_utf8;

use cql_bindgen::cass_error_desc;
use cql_bindgen::CASS_ERROR_LIB_BAD_PARAMS;
use cql_bindgen::CASS_ERROR_LIB_NO_STREAMS;
use cql_bindgen::CASS_ERROR_LAST_ENTRY;
use cql_bindgen::CASS_ERROR_SSL_IDENTITY_MISMATCH;
use cql_bindgen::CASS_ERROR_SSL_INVALID_PEER_CERT;
use cql_bindgen::CASS_ERROR_SSL_NO_PEER_CERT;
use cql_bindgen::CASS_ERROR_SSL_INVALID_PRIVATE_KEY;
use cql_bindgen::CASS_ERROR_SSL_INVALID_CERT;
use cql_bindgen::CASS_ERROR_SSL_PROTOCOL_ERROR;
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
use cql_bindgen::cass_error_num_arg_types;
use cql_bindgen::cass_error_result_arg_type;
use cql_bindgen::cass_error_result_code;
use cql_bindgen::cass_error_result_consistency;
use cql_bindgen::cass_error_result_data_present;
use cql_bindgen::cass_error_result_free;
use cql_bindgen::cass_error_result_function;
use cql_bindgen::cass_error_result_keyspace;
use cql_bindgen::cass_error_result_num_failures;
use cql_bindgen::cass_error_result_responses_received;
use cql_bindgen::cass_error_result_responses_required;
use cql_bindgen::cass_error_result_table;
use cql_bindgen::cass_error_result_write_type;

use cql_ffi::consistency::Consistency;
use cql_ffi::write_type::WriteType;
use cql_bindgen::CassError as _CassError;

use cql_bindgen::CassErrorResult as _CassErrorResult;

#[repr(C)]
pub enum CassErrorSource {
    NONE = 0isize,
    LIB = 1,
    SERVER = 2,
    SSL = 3,
    COMPRESSION = 4,
}

pub enum CassRustError {
    NulInString(NulError),
}

pub enum CassError {
    Lib(CassLibError),
    Server(CassServerError),
    Ssl(CassSslError),
    Rust(CassRustError),
}
pub struct CassLibError(_CassError);
pub struct CassServerError(_CassError);
pub struct CassSslError(_CassError);

impl Error for CassError {
    fn description(&self) -> &str {
        self.desc()
        // let c_buf: *const i8 = self.desc();
        // let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        // from_utf8(buf).unwrap()
    }
}

impl From<NulError> for CassError {
    fn from(err: NulError) -> CassError { CassError::Rust(CassRustError::NulInString(err)) }
}

impl Display for CassError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.desc())
        //        let c_buf: *const i8 = self.desc();
        //        let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        //        match str::from_utf8(buf) {
        //            Ok(str_slice) => write!(f, "{}", str_slice),
        //            Err(err) => panic!("unreachable? {:?}", err),
        //        }
    }
}

impl Debug for CassError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.desc())
        //        let c_buf: *const i8 = self.desc();
        //        let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        //        match str::from_utf8(buf) {
        //            Ok(str_slice) => write!(f, "{:?}", str_slice),
        //            Err(err) => panic!("unreachable? {:?}", err),
        //        }
    }
}

// impl From<NulError> for CassError {
//    fn from(err: NulError) -> CassError {
//        CliError::Io(err)
//    }
// }

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
    LAST_ENTRY = 50331654,
}

impl CassError {
    pub fn wrap<T>(self, wrappee: T) -> Result<T, CassError> {
        match self {
            CassError::Server(ref err) => {
                match err.0 {
                    CASS_OK => Ok(wrappee),
                    err => Err(CassError::build(err)),
                }
            }
            CassError::Lib(ref err) => {
                match err.0 {
                    CASS_OK => Ok(wrappee),
                    err => Err(CassError::build(err)),
                }
            }
            CassError::Ssl(ref err) => {
                match err.0 {
                    CASS_OK => Ok(wrappee),
                    err => Err(CassError::build(err)),
                }
            }
            CassError::Rust(err) => Err(CassError::Rust(err)),

        }

    }

    pub fn build_from_rust(err: CassRustError) -> CassError { CassError::Rust(err) }

    pub fn build(val: u32) -> CassError {
        match val {
            // 0 => CassError(CASS_OK),
            1 => CassError::Lib(CassLibError(CASS_ERROR_LIB_BAD_PARAMS)),
            2 => CassError::Lib(CassLibError(CASS_ERROR_LIB_NO_STREAMS)),
            3 => CassError::Lib(CassLibError(CASS_ERROR_LIB_UNABLE_TO_INIT)),
            4 => CassError::Lib(CassLibError(CASS_ERROR_LIB_MESSAGE_ENCODE)),
            5 => CassError::Lib(CassLibError(CASS_ERROR_LIB_HOST_RESOLUTION)),
            6 => CassError::Lib(CassLibError(CASS_ERROR_LIB_UNEXPECTED_RESPONSE)),
            7 => CassError::Lib(CassLibError(CASS_ERROR_LIB_REQUEST_QUEUE_FULL)),
            8 => CassError::Lib(CassLibError(CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD)),
            9 => CassError::Lib(CassLibError(CASS_ERROR_LIB_WRITE_ERROR)),
            10 | 16777226 => CassError::Lib(CassLibError(CASS_ERROR_LIB_NO_HOSTS_AVAILABLE)),
            11 => CassError::Lib(CassLibError(CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS)),
            12 => CassError::Lib(CassLibError(CASS_ERROR_LIB_INVALID_ITEM_COUNT)),
            13 => CassError::Lib(CassLibError(CASS_ERROR_LIB_INVALID_VALUE_TYPE)),
            14 => CassError::Lib(CassLibError(CASS_ERROR_LIB_REQUEST_TIMED_OUT)),
            15 => CassError::Lib(CassLibError(CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE)),
            16 => CassError::Lib(CassLibError(CASS_ERROR_LIB_CALLBACK_ALREADY_SET)),
            17 => CassError::Lib(CassLibError(CASS_ERROR_LIB_INVALID_STATEMENT_TYPE)),
            18 => CassError::Lib(CassLibError(CASS_ERROR_LIB_NAME_DOES_NOT_EXIST)),
            19 => CassError::Lib(CassLibError(CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL)),
            20 => CassError::Lib(CassLibError(CASS_ERROR_LIB_NULL_VALUE)),
            21 => CassError::Lib(CassLibError(CASS_ERROR_LIB_NOT_IMPLEMENTED)),
            22 => CassError::Lib(CassLibError(CASS_ERROR_LIB_UNABLE_TO_CONNECT)),
            23 => CassError::Lib(CassLibError(CASS_ERROR_LIB_UNABLE_TO_CLOSE)),
            33554432 => CassError::Server(CassServerError(CASS_ERROR_SERVER_SERVER_ERROR)),
            33554442 => CassError::Server(CassServerError(CASS_ERROR_SERVER_PROTOCOL_ERROR)),
            33554688 => CassError::Server(CassServerError(CASS_ERROR_SERVER_BAD_CREDENTIALS)),
            33558528 => CassError::Server(CassServerError(CASS_ERROR_SERVER_UNAVAILABLE)),
            33558529 => CassError::Server(CassServerError(CASS_ERROR_SERVER_OVERLOADED)),
            33558530 => CassError::Server(CassServerError(CASS_ERROR_SERVER_IS_BOOTSTRAPPING)),
            33558531 => CassError::Server(CassServerError(CASS_ERROR_SERVER_TRUNCATE_ERROR)),
            33558784 => CassError::Server(CassServerError(CASS_ERROR_SERVER_WRITE_TIMEOUT)),
            33559040 => CassError::Server(CassServerError(CASS_ERROR_SERVER_READ_TIMEOUT)),
            33562624 => CassError::Server(CassServerError(CASS_ERROR_SERVER_SYNTAX_ERROR)),
            33562880 => CassError::Server(CassServerError(CASS_ERROR_SERVER_UNAUTHORIZED)),
            33563136 => CassError::Server(CassServerError(CASS_ERROR_SERVER_INVALID_QUERY)),
            33563392 => CassError::Server(CassServerError(CASS_ERROR_SERVER_CONFIG_ERROR)),
            33563648 => CassError::Server(CassServerError(CASS_ERROR_SERVER_ALREADY_EXISTS)),
            33563904 => CassError::Server(CassServerError(CASS_ERROR_SERVER_UNPREPARED)),
            50331649 => CassError::Server(CassServerError(CASS_ERROR_SSL_INVALID_CERT)),
            50331650 => CassError::Server(CassServerError(CASS_ERROR_SSL_INVALID_PRIVATE_KEY)),
            50331651 => CassError::Ssl(CassSslError(CASS_ERROR_SSL_NO_PEER_CERT)),
            50331652 => CassError::Ssl(CassSslError(CASS_ERROR_SSL_INVALID_PEER_CERT)),
            50331653 => CassError::Ssl(CassSslError(CASS_ERROR_SSL_IDENTITY_MISMATCH)),
            50331654 => CassError::Ssl(CassSslError(CASS_ERROR_SSL_PROTOCOL_ERROR)),
            50331655 => CassError::Ssl(CassSslError(CASS_ERROR_LAST_ENTRY)),
            err_no => {
                debug!("unhandled error number: {}", err_no);
                unimplemented!();
                // CassError(err_no)
            }
        }
    }
}

pub struct CassErrorResult(pub *const _CassErrorResult);

impl CassErrorResult {
    ///Gets error code for the error result. This error code will always
    ///have an server error source.
    pub fn result_code(&self) -> u32 { unsafe { cass_error_result_code(self.0) } }

    ///Gets consistency that triggered the error result of the
    ///following types:
    ///
    ///<ul>
    ///  <li>CASS_ERROR_SERVER_READ_TIMEOUT</li>
    ///  <li>CASS_ERROR_SERVER_WRITE_TIMEOUT</li>
    ///  <li>CASS_ERROR_SERVER_READ_FAILURE</li>
    ///  <li>CASS_ERROR_SERVER_WRITE_FAILURE</li>
    ///  <li>CASS_ERROR_SERVER_UNAVAILABLE</li>
    /// </ul>
    pub fn result_consistency(&self) -> Consistency { unsafe { Consistency(cass_error_result_consistency(self.0)) } }

    /// Gets the actual number of received responses, received acknowledgments
    ///or alive nodes for following error result types, respectively:
    ///
    ///<ul>
    ///  <li>CASS_ERROR_SERVER_READ_TIMEOUT</li>
    ///   <li>CASS_ERROR_SERVER_WRITE_TIMEOUT</li>
    ///   <li>CASS_ERROR_SERVER_READ_FAILURE</li>
    ///   <li>CASS_ERROR_SERVER_WRITE_FAILURE</li>
    ///   <li>CASS_ERROR_SERVER_UNAVAILABLE</li>
    /// </ul>
    pub fn responses_received(&self) -> i32 { unsafe { cass_error_result_responses_received(self.0) } }

    /// Gets required responses, required acknowledgments or required alive nodes
    ///needed to successfully complete the request for following error result types,
    ///respectively:
    ///
    ///<ul>
    ///  <li>CASS_ERROR_SERVER_READ_TIMEOUT</li>
    ///   <li>CASS_ERROR_SERVER_WRITE_TIMEOUT</li>
    ///   <li>CASS_ERROR_SERVER_READ_FAILURE</li>
    ///   <li>CASS_ERROR_SERVER_WRITE_FAILURE</li>
    ///   <li>CASS_ERROR_SERVER_UNAVAILABLE</li>
    /// </ul>
    pub fn responses_required(&self) -> i32 { unsafe { cass_error_result_responses_required(self.0) } }

    ///Gets the number of nodes that experienced failures for the following error types:
    ///
    ///<ul>
    ///   <li>CASS_ERROR_SERVER_READ_FAILURE</li>
    ///   <li>CASS_ERROR_SERVER_WRITE_FAILURE</li>
    /// </ul>
    pub fn num_failures(&self) -> i32 { unsafe { cass_error_result_num_failures(self.0) } }

    ///Determines whether the actual data was present in the responses from the
    ///replicas for the following error result types:
    ///
    /// <ul>
    ///   <li>CASS_ERROR_SERVER_READ_TIMEOUT</li>
    ///   <li>CASS_ERROR_SERVER_READ_FAILURE</li>
    /// </ul>
    pub fn data_present(&self) -> bool {
        unsafe { if cass_error_result_data_present(self.0) > 0 { true } else { false } }
    }


    ///Gets the write type of a request for the following error result types:
    ///
    /// <ul>
    ///   <li>CASS_ERROR_SERVER_WRITE_TIMEOUT</li>
    ///   <li>CASS_ERROR_SERVER_WRITE_FAILURE</li>
    /// </ul>
    pub fn write_type(&self) -> WriteType { unsafe { WriteType(cass_error_result_write_type(self.0)) } }

    ///Gets the affected keyspace for the following error result types:
    ///
    /// <ul>
    ///   <li>CASS_ERROR_SERVER_ALREADY_EXISTS</li>
    ///   <li>CASS_ERROR_SERVER_FUNCTION_FAILURE</li>
    ///</ul>
    pub fn keyspace(&self, function: &str) -> Result<(), CassError> {
        unsafe {
            match cass_error_result_keyspace(self.0,
                                             &mut (function.as_ptr() as *const i8),
                                             &mut (function.len() as u64)) {
                CASS_OK => Ok(()),
                err => Err(CassError::build(err)),
            }
        }
    }

    ///Gets the affected table for the already exists error
    ///(CASS_ERROR_SERVER_ALREADY_EXISTS) result type.
    pub fn table(&self, table: &str) -> Result<(), CassError> {
        unsafe {
            match cass_error_result_table(self.0,
                                          &mut (table.as_ptr() as *const i8),
                                          &mut (table.len() as u64)) {
                CASS_OK => Ok(()),
                err => Err(CassError::build(err)),
            }
        }
    }

    ///Gets the affected function for the function failure error
    ///(CASS_ERROR_SERVER_FUNCTION_FAILURE) result type.
    pub fn function(&self, function: &str) -> Result<(), CassError> {
        unsafe {
            match cass_error_result_function(self.0,
                                             &mut (function.as_ptr() as *const i8),
                                             &mut (function.len() as u64)) {
                CASS_OK => Ok(()),
                err => Err(CassError::build(err)),
            }
        }
    }

    ///Gets the number of argument types for the function failure error
    ///(CASS_ERROR_SERVER_FUNCTION_FAILURE) result type.
    pub fn num_arg_types(error_result: CassErrorResult) -> u64 { unsafe { cass_error_num_arg_types(error_result.0) } }

    ///Gets the argument type at the specified index for the function failure
    ///error (CASS_ERROR_SERVER_FUNCTION_FAILURE) result type.
    pub fn arg_type(&self, index: u64, arg_type: &str) -> u32 {
        unsafe {
            let cstr = CString::new(arg_type).unwrap();
            cass_error_result_arg_type(self.0,
                                       index,
                                       &mut cstr.as_ptr(),
                                       &mut (cstr.to_bytes().len() as u64))
        }
    }
}
impl Drop for CassErrorResult {
    fn drop(&mut self) { unsafe { cass_error_result_free(self.0) } }
}

impl CassError {
    fn pointer_to_string<'a>(c_buf: *const i8) -> &'a str {
        let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        from_utf8(buf).unwrap()
    }

    pub fn desc(&self) -> &str {
        unsafe {
            match self {
                &CassError::Lib(ref err) => CassError::pointer_to_string(cass_error_desc(err.0)),
                &CassError::Ssl(ref err) => CassError::pointer_to_string(cass_error_desc(err.0)),
                &CassError::Server(ref err) => CassError::pointer_to_string(cass_error_desc(err.0)),
                &CassError::Rust(ref err) => {
                    match err {
                        &CassRustError::NulInString(_) => "Tried to create a CString with a nul in the middle",
                    }
                }
            }
        }
    }

    pub fn debug(&self) { println!("{:?}", self) }
}
