use cql_bindgen::CassConsistency as _CassConsistency;

use std::ffi::CStr;

use cql_bindgen::cass_consistency_string;

pub struct Consistency(pub _CassConsistency);

impl ToString for Consistency {
    fn to_string(&self) -> String {
        unsafe {
            let my_string = cass_consistency_string(self.0);
            CStr::from_ptr(my_string).to_string_lossy().into_owned()
        }
    }
}
