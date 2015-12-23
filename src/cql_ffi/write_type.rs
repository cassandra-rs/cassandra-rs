use std::ffi::CStr;

use cql_bindgen::cass_write_type_string;
use cql_bindgen::CassWriteType;


pub struct WriteType(pub CassWriteType);

impl WriteType {
    pub fn write_type_string(&self) -> String {
        unsafe { CStr::from_ptr(cass_write_type_string(self.0)).to_str().unwrap().to_owned() }
    }
}
