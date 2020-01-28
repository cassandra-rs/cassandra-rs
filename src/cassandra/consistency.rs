use crate::cassandra::util::Protected;
use crate::cassandra_sys::CassConsistency_;

use crate::cassandra_sys::cass_consistency_string;

use std::ffi::CStr;

/// A Cassandra consistency level.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[allow(missing_docs)] // Meanings are defined in CQL documentation.
#[allow(non_camel_case_types)] // Names are traditional.
pub enum Consistency {
    UNKNOWN,
    ANY,
    ONE,
    TWO,
    THREE,
    QUORUM,
    ALL,
    LOCAL_QUORUM,
    EACH_QUORUM,
    SERIAL,
    LOCAL_SERIAL,
    LOCAL_ONE,
}

enhance_nullary_enum!(Consistency, CassConsistency_, {
    (UNKNOWN, CASS_CONSISTENCY_UNKNOWN, "UNKNOWN"),
    (ANY, CASS_CONSISTENCY_ANY, "ANY"),
    (ONE, CASS_CONSISTENCY_ONE, "ONE"),
    (TWO, CASS_CONSISTENCY_TWO, "TWO"),
    (THREE, CASS_CONSISTENCY_THREE, "THREE"),
    (QUORUM, CASS_CONSISTENCY_QUORUM, "QUORUM"),
    (ALL, CASS_CONSISTENCY_ALL, "ALL"),
    (LOCAL_QUORUM, CASS_CONSISTENCY_LOCAL_QUORUM, "LOCAL_QUORUM"),
    (EACH_QUORUM, CASS_CONSISTENCY_EACH_QUORUM, "EACH_QUORUM"),
    (SERIAL, CASS_CONSISTENCY_SERIAL, "SERIAL"),
    (LOCAL_SERIAL, CASS_CONSISTENCY_LOCAL_SERIAL, "LOCAL_SERIAL"),
    (LOCAL_ONE, CASS_CONSISTENCY_LOCAL_ONE, "LOCAL_ONE"),
});
