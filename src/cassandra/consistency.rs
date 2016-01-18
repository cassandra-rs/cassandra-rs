use cassandra_sys::CassConsistency as _CassConsistency;

use std::ffi::CStr;

use cassandra_sys::cass_consistency_string;

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

pub mod protected {
    use cassandra::consistency::Consistency;
    use cassandra_sys::CassConsistency as _CassConsistency;
    pub fn build(consistency: _CassConsistency) -> Consistency {
        Consistency(consistency)
    }
    pub fn inner(consistency: Consistency) -> _CassConsistency {
        consistency.0
    }
}
