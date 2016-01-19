use std::fmt::{Debug, Display, Formatter};
use std::{fmt, mem, slice, str};
use std::ffi::{CStr, CString, NulError};
use std::error::Error;
use std::net::AddrParseError;

use cassandra_sys::cass_error_desc;
use cassandra_sys::CASS_ERROR_LIB_BAD_PARAMS;
use cassandra_sys::CASS_ERROR_LIB_NO_STREAMS;
use cassandra_sys::CASS_ERROR_LAST_ENTRY;
use cassandra_sys::CASS_ERROR_SSL_IDENTITY_MISMATCH;
use cassandra_sys::CASS_ERROR_SSL_INVALID_PEER_CERT;
use cassandra_sys::CASS_ERROR_SSL_NO_PEER_CERT;
use cassandra_sys::CASS_ERROR_SSL_INVALID_PRIVATE_KEY;
use cassandra_sys::CASS_ERROR_SSL_INVALID_CERT;
use cassandra_sys::CASS_ERROR_SSL_PROTOCOL_ERROR;
use cassandra_sys::CASS_ERROR_SERVER_UNPREPARED;
use cassandra_sys::CASS_ERROR_SERVER_ALREADY_EXISTS;
use cassandra_sys::CASS_ERROR_SERVER_CONFIG_ERROR;
use cassandra_sys::CASS_ERROR_SERVER_INVALID_QUERY;
use cassandra_sys::CASS_ERROR_SERVER_UNAUTHORIZED;
use cassandra_sys::CASS_ERROR_SERVER_SYNTAX_ERROR;
use cassandra_sys::CASS_ERROR_SERVER_READ_TIMEOUT;
use cassandra_sys::CASS_ERROR_SERVER_WRITE_TIMEOUT;
use cassandra_sys::CASS_ERROR_SERVER_TRUNCATE_ERROR;
use cassandra_sys::CASS_ERROR_SERVER_IS_BOOTSTRAPPING;
use cassandra_sys::CASS_ERROR_SERVER_OVERLOADED;
use cassandra_sys::CASS_ERROR_SERVER_UNAVAILABLE;
use cassandra_sys::CASS_ERROR_SERVER_BAD_CREDENTIALS;
use cassandra_sys::CASS_ERROR_SERVER_PROTOCOL_ERROR;
use cassandra_sys::CASS_ERROR_SERVER_SERVER_ERROR;
use cassandra_sys::CASS_ERROR_LIB_UNABLE_TO_CLOSE;
use cassandra_sys::CASS_ERROR_LIB_UNABLE_TO_CONNECT;
use cassandra_sys::CASS_ERROR_LIB_NOT_IMPLEMENTED;
use cassandra_sys::CASS_ERROR_LIB_NULL_VALUE;
use cassandra_sys::CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL;
use cassandra_sys::CASS_ERROR_LIB_NAME_DOES_NOT_EXIST;
use cassandra_sys::CASS_ERROR_LIB_INVALID_STATEMENT_TYPE;
use cassandra_sys::CASS_ERROR_LIB_CALLBACK_ALREADY_SET;
use cassandra_sys::CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE;
use cassandra_sys::CASS_ERROR_LIB_REQUEST_TIMED_OUT;
use cassandra_sys::CASS_ERROR_LIB_INVALID_VALUE_TYPE;
use cassandra_sys::CASS_ERROR_LIB_INVALID_ITEM_COUNT;
use cassandra_sys::CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS;
use cassandra_sys::CASS_ERROR_LIB_NO_HOSTS_AVAILABLE;
use cassandra_sys::CASS_ERROR_LIB_WRITE_ERROR;
use cassandra_sys::CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD;
use cassandra_sys::CASS_ERROR_LIB_REQUEST_QUEUE_FULL;
use cassandra_sys::CASS_ERROR_LIB_UNEXPECTED_RESPONSE;
use cassandra_sys::CASS_ERROR_LIB_HOST_RESOLUTION;
use cassandra_sys::CASS_ERROR_LIB_MESSAGE_ENCODE;
use cassandra_sys::CASS_ERROR_LIB_UNABLE_TO_INIT;
use cassandra_sys::CASS_OK;
use cassandra_sys::cass_error_num_arg_types;
use cassandra_sys::cass_error_result_arg_type;
use cassandra_sys::cass_error_result_code;
use cassandra_sys::cass_error_result_consistency;
use cassandra_sys::cass_error_result_data_present;
use cassandra_sys::cass_error_result_free;
use cassandra_sys::cass_error_result_function;
use cassandra_sys::cass_error_result_keyspace;
use cassandra_sys::cass_error_result_num_failures;
use cassandra_sys::cass_error_result_responses_received;
use cassandra_sys::cass_error_result_responses_required;
use cassandra_sys::cass_error_result_table;
use cassandra_sys::cass_error_result_write_type;

use cassandra::consistency::Consistency;
use cassandra::write_type::WriteType;
use cassandra_sys::CassError as _CassError;
use cassandra::consistency;
use cassandra::util::Protected;

use cassandra_sys::CassErrorResult as _CassErrorResult;

// #[repr(C)]
// //Upstream Cassandra errors can cone from multiple portions of the code, including both client and server code
// pub enum CassErrorSource {
//    ///No known source FIXME not sure if ever used
//    NONE = 0isize,
//    ///Error messages originating from the C++ driver code
//    LIB = 1,
//    ///Error messages originating from the Cassandra server
//    SERVER = 2,
//    ///Error messages originating from the SSL library linked from the client
//    SSL = 3,
//    ///Error message originating from the compression libraries linked from the client
//    COMPRESSION = 4,
// }

///These are Rust errors that are wrapped so that all errors returned by this driver can
///fall under the umbrella of CassError
pub enum CassRustError {
    ///NulErrors should only occur when you pass a string containing an internal null to
    ///to the driver code. The driver converts these to CStrings to use FFI to call out to
    ///the C++ driver, and C strings can't contain nulls. Don't do that.
    NulInString(NulError),
    ///Should only occur if you pass an invalidly formatted IP address string to the driver
    BadAddress(AddrParseError),
}

///All types of errors that this driver can return
pub enum CassError {
    ///An error signaled by the C++ driver
    Lib(CassLibError),
    ///An error signaled by the server and sent to the client over CQL transport
    Server(CassServerError),
    ///An error signaled by the client-linked SSL library
    Ssl(CassSslError),
    ///An error generated within rust code directly
    Rust(CassRustError),
}

///An error generated by the C++ driver
pub struct CassLibError {
    err: _CassError,
    msg: String,
}

///An error signaled by the server and sent to the client over CQL transport
pub struct CassServerError(_CassError);
///An error signaled by the client-linked SSL library
pub struct CassSslError(_CassError);

impl Error for CassError {
    fn description(&self) -> &str {
        self.desc()
        // let c_buf: *const i8 = self.desc();
        // let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        // from_utf8(buf).unwrap()
    }
}

impl From<AddrParseError> for CassError {
    fn from(err: AddrParseError) -> CassError {
        CassError::Rust(CassRustError::BadAddress(err))
    }
}

impl From<NulError> for CassError {
    fn from(err: NulError) -> CassError {
        CassError::Rust(CassRustError::NulInString(err))
    }
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
#[allow(missing_docs)]
#[allow(dead_code)]
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
    ///Takes an upstream error and wraps it into the appropriate CassError
    pub fn wrap<T>(self, wrappee: T) -> Result<T, CassError> {
        match self {
            CassError::Server(ref err) => {
                match err.0 {
                    CASS_OK => Ok(wrappee),
                    _ => Err(CassError::build(err.0, None)),
                }
            }
            CassError::Lib(ref err) => {
                match err.err {
                    CASS_OK => Ok(wrappee),
                    _ => Err(CassError::build(err.err, Some(&err.msg))),
                }
            }
            CassError::Ssl(ref err) => {
                println!("ssl: {}", err.0);
                match err.0 {
                    CASS_OK => Ok(wrappee),
                    _ => Err(CassError::build(err.0, None)),
                }
            }
            CassError::Rust(err) => {
                println!("rust");
                Err(CassError::Rust(err))
            }

        }
    }

    ///Creates a new CassError from a local Rust error
    pub fn build_from_rust(err: CassRustError) -> CassError {
        CassError::Rust(err)
    }

    ///Bui;ds a new CassError object out of an integer code. Some errors have
    ///hardcoded static messages, others should get one passed to them
    pub fn build(val: u32, msg: Option<&str>) -> CassError {
        let msg = msg.unwrap_or("").to_owned();
        match val {
            0 => {
                CassError::Lib(CassLibError {
                    err: CASS_OK,
                    msg: msg,
                })
            }
            1 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_BAD_PARAMS,
                    msg: msg,
                })
            }
            2 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_NO_STREAMS,
                    msg: msg,
                })
            }
            3 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_UNABLE_TO_INIT,
                    msg: msg,
                })
            }
            4 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_MESSAGE_ENCODE,
                    msg: msg,
                })
            }
            5 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_HOST_RESOLUTION,
                    msg: msg,
                })
            }
            6 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_UNEXPECTED_RESPONSE,
                    msg: msg,
                })
            }
            7 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_REQUEST_QUEUE_FULL,
                    msg: msg,
                })
            }
            8 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD,
                    msg: msg,
                })
            }
            9 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_WRITE_ERROR,
                    msg: msg,
                })
            }
            10 | 16777226 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_NO_HOSTS_AVAILABLE,
                    msg: msg,
                })
            }
            11 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS,
                    msg: msg,
                })
            }
            12 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_INVALID_ITEM_COUNT,
                    msg: msg,
                })
            }
            13 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_INVALID_VALUE_TYPE,
                    msg: msg,
                })
            }
            14 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_REQUEST_TIMED_OUT,
                    msg: msg,
                })
            }
            15 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE,
                    msg: msg,
                })
            }
            16 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_CALLBACK_ALREADY_SET,
                    msg: msg,
                })
            }
            17 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_INVALID_STATEMENT_TYPE,
                    msg: msg,
                })
            }
            18 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_NAME_DOES_NOT_EXIST,
                    msg: msg,
                })
            }
            19 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL,
                    msg: msg,
                })
            }
            20 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_NULL_VALUE,
                    msg: msg,
                })
            }
            21 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_NOT_IMPLEMENTED,
                    msg: msg,
                })
            }
            22 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_UNABLE_TO_CONNECT,
                    msg: msg,
                })
            }
            23 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_UNABLE_TO_CLOSE,
                    msg: msg,
                })
            }
            16777229 => {
                CassError::Lib(CassLibError {
                    err: CASS_ERROR_LIB_INVALID_VALUE_TYPE,
                    msg: msg,
                })
            }
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
                panic!("unhandled error number: {}", err_no);
                // CassError(err_no)
            }
        }
    }
}

///An error result of a request
pub struct CassErrorResult(*const _CassErrorResult);

impl Protected<*const _CassErrorResult> for CassErrorResult {
    fn inner(&self) -> *const _CassErrorResult {
        self.0
    }
    fn build(inner: *const _CassErrorResult) -> Self {
        CassErrorResult(inner)
    }
}

impl CassErrorResult {
    ///Gets error code for the error result. This error code will always
    ///have an server error source.
    pub fn result_code(&self) -> u32 {
        unsafe { cass_error_result_code(self.0) }
    }

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
    pub fn result_consistency(&self) -> Consistency {
        unsafe { Consistency::build(cass_error_result_consistency(self.0)) }
    }

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
    pub fn responses_received(&self) -> i32 {
        unsafe { cass_error_result_responses_received(self.0) }
    }

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
    pub fn responses_required(&self) -> i32 {
        unsafe { cass_error_result_responses_required(self.0) }
    }

    ///Gets the number of nodes that experienced failures for the following error types:
    ///
    ///<ul>
    ///   <li>CASS_ERROR_SERVER_READ_FAILURE</li>
    ///   <li>CASS_ERROR_SERVER_WRITE_FAILURE</li>
    /// </ul>
    pub fn num_failures(&self) -> i32 {
        unsafe { cass_error_result_num_failures(self.0) }
    }

    ///Determines whether the actual data was present in the responses from the
    ///replicas for the following error result types:
    ///
    /// <ul>
    ///   <li>CASS_ERROR_SERVER_READ_TIMEOUT</li>
    ///   <li>CASS_ERROR_SERVER_READ_FAILURE</li>
    /// </ul>
    pub fn data_present(&self) -> bool {
        unsafe { cass_error_result_data_present(self.0) != 0 }
    }


    ///Gets the write type of a request for the following error result types:
    ///
    /// <ul>
    ///   <li>CASS_ERROR_SERVER_WRITE_TIMEOUT</li>
    ///   <li>CASS_ERROR_SERVER_WRITE_FAILURE</li>
    /// </ul>
    pub fn write_type(&self) -> WriteType {
        unsafe { WriteType(cass_error_result_write_type(self.0)) }
    }

    ///Gets the affected keyspace for the following error result types:
    ///
    /// <ul>
    ///   <li>CASS_ERROR_SERVER_ALREADY_EXISTS</li>
    ///   <li>CASS_ERROR_SERVER_FUNCTION_FAILURE</li>
    ///</ul>
    pub fn keyspace(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut length = mem::zeroed();
            match cass_error_result_keyspace(self.0, &mut name, &mut length) {
                CASS_OK => {
                    let slice = slice::from_raw_parts(name as *const u8, length as usize);
                    str::from_utf8(slice).unwrap().to_owned()
                }
                err => panic!("impossible: {}", err),
            }
        }
    }

    ///Gets the affected table for the already exists error
    ///(CASS_ERROR_SERVER_ALREADY_EXISTS) result type.
    pub fn table(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut length = mem::zeroed();
            match cass_error_result_table(self.0, &mut name, &mut length) {
                CASS_OK => {
                    let slice = slice::from_raw_parts(name as *const u8, length as usize);
                    str::from_utf8(slice).unwrap().to_owned()
                }
                err => panic!("impossible: {}", err),
            }
        }
    }

    ///Gets the affected function for the function failure error
    ///(CASS_ERROR_SERVER_FUNCTION_FAILURE) result type.
    pub fn function(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut length = mem::zeroed();
            match cass_error_result_function(self.0, &mut name, &mut length) {
                CASS_OK => {
                    let slice = slice::from_raw_parts(name as *const u8, length as usize);
                    str::from_utf8(slice).unwrap().to_owned()
                }
                err => panic!("impossible: {}", err),
            }
        }
    }

    ///Gets the number of argument types for the function failure error
    ///(CASS_ERROR_SERVER_FUNCTION_FAILURE) result type.
    pub fn num_arg_types(error_result: CassErrorResult) -> u64 {
        unsafe { cass_error_num_arg_types(error_result.0) }
    }

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
    fn drop(&mut self) {
        unsafe { cass_error_result_free(self.0) }
    }
}

impl CassError {
    fn pointer_to_string<'a>(c_buf: *const i8) -> &'a str {
        let buf: &[u8] = unsafe { CStr::from_ptr(c_buf).to_bytes() };
        str::from_utf8(buf).unwrap()
    }

    ///Gets the textual description for this error
    pub fn desc<'a>(&'a self) -> &'a str {
        unsafe {
            match *self {
                CassError::Lib(ref err) => CassError::pointer_to_string(cass_error_desc(err.err)),
                CassError::Ssl(ref err) => CassError::pointer_to_string(cass_error_desc(err.0)),
                CassError::Server(ref err) => CassError::pointer_to_string(cass_error_desc(err.0)),
                CassError::Rust(ref err) => {
                    match err {
                        &CassRustError::NulInString(_) => "Tried to create a CString with a nul in the middle",
                        /// /FIXME how do i return a slice created from self?
                        &CassRustError::BadAddress(_) => "Tried to parse an invalid ip address",
                    }
                }
            }
        }
    }
}
