use cassandra_sys::CassConsistency as _CassConsistency;

use std::ffi::CStr;

use cassandra_sys::cass_consistency_string;
use cassandra::util::Protected;



///A Cassandra consistency level
pub struct Consistency(_CassConsistency);

impl ToString for Consistency {
    fn to_string(&self) -> String {
        unsafe {
            let my_string = cass_consistency_string(self.0);
            CStr::from_ptr(my_string).to_string_lossy().into_owned()
        }
    }
}

impl Protected<_CassConsistency> for Consistency {
    fn inner(&self) -> _CassConsistency {
        self.0
    }
    fn build(inner: _CassConsistency) -> Self {
        Consistency(inner)
    }
}
