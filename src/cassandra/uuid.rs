use std::fmt::Formatter;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::mem;
use std::ffi::{CStr, CString};
use std::str;


use cassandra_sys::CASS_OK;
use cassandra_sys::CassUuid as _Uuid;
use cassandra_sys::CassUuidGen as _UuidGen;
use cassandra_sys::cass_uuid_gen_new;
use cassandra_sys::cass_uuid_gen_free;
use cassandra_sys::cass_uuid_gen_time;
use cassandra_sys::cass_uuid_gen_new_with_node;
use cassandra_sys::cass_uuid_gen_random;
use cassandra_sys::cass_uuid_gen_from_time;
use cassandra_sys::cass_uuid_min_from_time;
use cassandra_sys::cass_uuid_max_from_time;
use cassandra_sys::cass_uuid_timestamp;
use cassandra_sys::cass_uuid_version;
use cassandra::util::Protected;

use cassandra_sys::cass_uuid_string;
// use cassandra_sys::raw2utf8;
use cassandra_sys::cass_uuid_from_string;

use cassandra::error::CassError;

// const CASS_UUID_STRING_LENGTH: usize = 37;


#[derive(Copy,Clone)]
///Version 1 (time-based) or version 4 (random) UUID.
pub struct Uuid(_Uuid);

impl Protected<_Uuid> for Uuid {
    fn inner(&self) -> _Uuid {
        self.0
    }
    fn build(inner: _Uuid) -> Self {
        Uuid(inner)
    }
}

impl Default for Uuid {
    fn default() -> Uuid {
        unsafe { ::std::mem::zeroed() }
    }
}

///A UUID generator object.
///
///Instances of the UUID generator object are thread-safe to generate UUIDs.
pub struct UuidGen(*mut _UuidGen);
unsafe impl Sync for UuidGen {}
unsafe impl Send for UuidGen {}

impl Drop for UuidGen {
    fn drop(&mut self) {
        unsafe { cass_uuid_gen_free(self.0) }
    }
}

// impl Debug for Uuid {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//        write!(f, "{:?}", self.to_string())
//    }
// }

// FIXME!!!!!!!
impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unsafe {
            let mut string = mem::zeroed();
            cass_uuid_string(self.0, &mut string);
            let slice = CStr::from_ptr(&string);
            write!(f, "{}", str::from_utf8(slice.to_bytes()).unwrap())
        }
    }
}

impl Uuid {
    /// Generates a V1 (time) UUID for the specified time.
    pub fn min_from_time(&mut self, time: u64) {
        unsafe { cass_uuid_min_from_time(time, &mut self.0) }
    }

    ///Sets the UUID to the minimum V1 (time) value for the specified tim
    pub fn max_from_time(&mut self, time: u64) {
        unsafe { cass_uuid_max_from_time(time, &mut self.0) }
    }

    ///Gets the timestamp for a V1 UUID
    pub fn timestamp(&self) -> u64 {
        unsafe { cass_uuid_timestamp(self.0) }
    }

    ///Gets the version for a UUID
    pub fn version(&self) -> u8 {
        unsafe { cass_uuid_version(self.0) }
    }
}

impl str::FromStr for Uuid {
    type Err = CassError;
    fn from_str(str: &str) -> Result<Uuid, CassError> {
        unsafe {
            let mut uuid = mem::zeroed();
            match cass_uuid_from_string(try!(CString::new(str)).as_ptr(), &mut uuid) {
                CASS_OK => Ok(Uuid(uuid)),
                err => Err(CassError::build(err, None)),
            }
        }
    }
}


impl UuidGen {
    ///Creates a new thread-safe UUID generator
    pub fn new() -> Self {
        unsafe { UuidGen(cass_uuid_gen_new()) }
    }

    ///Creates a new UUID generator with custom node information.
    ///<b>Note:</b> This object is thread-safe. It is best practice to create and reuse
    ///a single object per application.
    pub fn new_with_node(node: u64) -> UuidGen {
        unsafe { UuidGen(cass_uuid_gen_new_with_node(node)) }
    }

    ///Generates a V1 (time) UUID.
    pub fn gen_time(&self) -> Uuid {
        unsafe {
            let mut output: _Uuid = mem::zeroed();
            cass_uuid_gen_time(self.0, &mut output);
            Uuid(output)
        }
    }

    ///Generates a new V4 (random) UUID
    pub fn gen_random(&self) -> Uuid {
        unsafe {
            let mut output: _Uuid = mem::zeroed();
            cass_uuid_gen_random(self.0, &mut output);
            Uuid(output)
        }
    }

    ///Generates a V1 (time) UUID for the specified time.
    pub fn gen_from_time(&self, timestamp: u64) -> Uuid {
        unsafe {
            let mut output: _Uuid = mem::zeroed();
            cass_uuid_gen_from_time(self.0, timestamp, &mut output);
            Uuid(output)
        }
    }
}
