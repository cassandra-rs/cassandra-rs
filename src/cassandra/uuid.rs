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
use cassandra_sys::cass_uuid_string;
// use cassandra_sys::raw2utf8;
use cassandra_sys::cass_uuid_from_string;

use cassandra::error::CassError;

const CASS_UUID_STRING_LENGTH: usize = 37;


#[derive(Copy,Clone)]
pub struct Uuid(pub _Uuid);

impl ::std::default::Default for Uuid {
    fn default() -> Uuid {
        unsafe { ::std::mem::zeroed() }
    }
}

pub struct UuidGen(pub *mut _UuidGen);

impl Drop for UuidGen {
    fn drop(&mut self) {
        unsafe { cass_uuid_gen_free(self.0) }
    }
}

impl Debug for Uuid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Uuid {
    pub fn min_from_time(&mut self, time: u64) {
        unsafe { cass_uuid_min_from_time(time, &mut self.0) }
    }

    pub fn max_from_time(&mut self, time: u64) {
        unsafe { cass_uuid_max_from_time(time, &mut self.0) }
    }

    pub fn timestamp(&self) -> u64 {
        unsafe { cass_uuid_timestamp(self.0) }
    }

    pub fn version(&self) -> u8 {
        unsafe { cass_uuid_version(self.0) }
    }

    // FIXME
    pub fn to_string(&self) -> String {
        unsafe {
            let mut time_str: [i8; CASS_UUID_STRING_LENGTH] = [0; CASS_UUID_STRING_LENGTH];

            cass_uuid_string(self.0, time_str[..].as_mut_ptr());
            let mut output: i8 = mem::zeroed();
            cass_uuid_string(self.0, &mut output);
            let slice = CStr::from_ptr(&output);
            str::from_utf8(slice.to_bytes()).unwrap().to_owned()
        }
    }

    pub fn from_string(str: &str) -> Result<Uuid, CassError> {
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
    pub fn new() -> Self {
        unsafe { UuidGen(cass_uuid_gen_new()) }
    }

    pub fn new_with_node(node: u64) -> UuidGen {
        unsafe { UuidGen(cass_uuid_gen_new_with_node(node)) }
    }

    pub fn get_time(&self) -> Uuid {
        unsafe {
            let mut output: _Uuid = mem::zeroed();
            cass_uuid_gen_time(self.0, &mut output);
            Uuid(output)
        }
    }

    pub fn fill_random(&self, mut output: Uuid) {
        unsafe { cass_uuid_gen_random(self.0, &mut output.0) }
    }

    pub fn random(&self) -> Uuid {
        unsafe {
            let mut output: _Uuid = mem::zeroed();
            cass_uuid_gen_random(self.0, &mut output);
            Uuid(output)
        }
    }

    pub fn with_time(&self, timestamp: u64, mut output: Uuid) {
        unsafe { cass_uuid_gen_from_time(self.0, timestamp, &mut output.0) }
    }
}
