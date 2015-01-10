#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::types::os::arch::c95::c_uint;
use libc::types::os::arch::c95::c_char;

pub type Enum_CassErrorSource_ = c_uint;
pub const CASS_ERROR_SOURCE_NONE: c_uint = 0;
pub const CASS_ERROR_SOURCE_LIB: c_uint = 1;
pub const CASS_ERROR_SOURCE_SERVER: c_uint = 2;
pub const CASS_ERROR_SOURCE_SSL: c_uint = 3;
pub const CASS_ERROR_SOURCE_COMPRESSION: c_uint = 4;
pub type CassErrorSource = Enum_CassErrorSource_;
type Enum_CassError_ = c_uint;
pub const CASS_OK: c_uint = 0;
pub const CASS_ERROR_LIB_BAD_PARAMS: c_uint = 16777217;
pub const CASS_ERROR_LIB_NO_STREAMS: c_uint = 16777218;
pub const CASS_ERROR_LIB_UNABLE_TO_INIT: c_uint = 16777219;
pub const CASS_ERROR_LIB_MESSAGE_ENCODE: c_uint = 16777220;
pub const CASS_ERROR_LIB_HOST_RESOLUTION: c_uint = 16777221;
pub const CASS_ERROR_LIB_UNEXPECTED_RESPONSE: c_uint = 16777222;
pub const CASS_ERROR_LIB_REQUEST_QUEUE_FULL: c_uint = 16777223;
pub const CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD: c_uint = 16777224;
pub const CASS_ERROR_LIB_WRITE_ERROR: c_uint = 16777225;
pub const CASS_ERROR_LIB_NO_HOSTS_AVAILABLE: c_uint = 16777226;
pub const CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS: c_uint = 16777227;
pub const CASS_ERROR_LIB_INVALID_ITEM_COUNT: c_uint = 16777228;
pub const CASS_ERROR_LIB_INVALID_VALUE_TYPE: c_uint = 16777229;
pub const CASS_ERROR_LIB_REQUEST_TIMED_OUT: c_uint = 16777230;
pub const CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE: c_uint = 16777231;
pub const CASS_ERROR_LIB_CALLBACK_ALREADY_SET: c_uint = 16777232;
pub const CASS_ERROR_LIB_INVALID_STATEMENT_TYPE: c_uint = 16777233;
pub const CASS_ERROR_LIB_NAME_DOES_NOT_EXIST: c_uint = 16777234;
pub const CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL: c_uint =
    16777235;
pub const CASS_ERROR_LIB_NULL_VALUE: c_uint = 16777236;
pub const CASS_ERROR_LIB_NOT_IMPLEMENTED: c_uint = 16777237;
pub const CASS_ERROR_SERVER_SERVER_ERROR: c_uint = 33554432;
pub const CASS_ERROR_SERVER_PROTOCOL_ERROR: c_uint = 33554442;
pub const CASS_ERROR_SERVER_BAD_CREDENTIALS: c_uint = 33554688;
pub const CASS_ERROR_SERVER_UNAVAILABLE: c_uint = 33558528;
pub const CASS_ERROR_SERVER_OVERLOADED: c_uint = 33558529;
pub const CASS_ERROR_SERVER_IS_BOOTSTRAPPING: c_uint = 33558530;
pub const CASS_ERROR_SERVER_TRUNCATE_ERROR: c_uint = 33558531;
pub const CASS_ERROR_SERVER_WRITE_TIMEOUT: c_uint = 33558784;
pub const CASS_ERROR_SERVER_READ_TIMEOUT: c_uint = 33559040;
pub const CASS_ERROR_SERVER_SYNTAX_ERROR: c_uint = 33562624;
pub const CASS_ERROR_SERVER_UNAUTHORIZED: c_uint = 33562880;
pub const CASS_ERROR_SERVER_INVALID_QUERY: c_uint = 33563136;
pub const CASS_ERROR_SERVER_CONFIG_ERROR: c_uint = 33563392;
pub const CASS_ERROR_SERVER_ALREADY_EXISTS: c_uint = 33563648;
pub const CASS_ERROR_SERVER_UNPREPARED: c_uint = 33563904;
pub const CASS_ERROR_SSL_INVALID_CERT: c_uint = 50331649;
pub const CASS_ERROR_SSL_INVALID_PRIVATE_KEY: c_uint = 50331650;
pub const CASS_ERROR_SSL_NO_PEER_CERT: c_uint = 50331651;
pub const CASS_ERROR_SSL_INVALID_PEER_CERT: c_uint = 50331652;
pub const CASS_ERROR_SSL_IDENTITY_MISMATCH: c_uint = 50331653;
pub const CASS_ERROR_LAST_ENTRY: c_uint = 50331654;
pub type CassError = Enum_CassError_;

extern "C" {
    pub fn cass_error_desc(error: CassError) -> *const c_char;
}
