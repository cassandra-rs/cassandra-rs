use cassandra::util::Protected;
use cassandra_sys::CassConsistency_ as CassConsistency;

use cassandra_sys::cass_consistency_string;

use std::ffi::CStr;
use std::fmt::{self, Display};
use std::str::FromStr;

/// A Cassandra consistency level.
#[derive(Debug, Eq, PartialEq)]
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

impl Display for Consistency {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", match *self {
            Consistency::UNKNOWN => "UNKNOWN",
            Consistency::ANY => "ANY",
            Consistency::ONE => "ONE",
            Consistency::TWO => "TWO",
            Consistency::THREE => "THREE",
            Consistency::QUORUM => "QUORUM",
            Consistency::ALL => "ALL",
            Consistency::LOCAL_QUORUM => "LOCAL_QUORUM",
            Consistency::EACH_QUORUM => "EACH_QUORUM",
            Consistency::SERIAL => "SERIAL",
            Consistency::LOCAL_SERIAL => "LOCAL_SERIAL",
            Consistency::LOCAL_ONE => "LOCAL_ONE",
        })
    }
}

impl FromStr for Consistency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNKNOWN" => Ok(Consistency::UNKNOWN),
            "ANY" => Ok(Consistency::ANY),
            "ONE" => Ok(Consistency::ONE),
            "TWO" => Ok(Consistency::TWO),
            "THREE" => Ok(Consistency::THREE),
            "QUORUM" => Ok(Consistency::QUORUM),
            "ALL" => Ok(Consistency::ALL),
            "LOCAL_QUORUM" => Ok(Consistency::LOCAL_QUORUM),
            "EACH_QUORUM" => Ok(Consistency::EACH_QUORUM),
            "SERIAL" => Ok(Consistency::SERIAL),
            "LOCAL_SERIAL" => Ok(Consistency::LOCAL_SERIAL),
            "LOCAL_ONE" => Ok(Consistency::LOCAL_ONE),
            _ => Err("Unrecognized consistency level: ".to_string() + s),
        }
    }
}

impl Protected<CassConsistency> for Consistency {
    fn build(inner: CassConsistency) -> Self {
        match inner {
            CassConsistency::CASS_CONSISTENCY_UNKNOWN => Consistency::UNKNOWN,
            CassConsistency::CASS_CONSISTENCY_ANY => Consistency::ANY,
            CassConsistency::CASS_CONSISTENCY_ONE => Consistency::ONE,
            CassConsistency::CASS_CONSISTENCY_TWO => Consistency::TWO,
            CassConsistency::CASS_CONSISTENCY_THREE => Consistency::THREE,
            CassConsistency::CASS_CONSISTENCY_QUORUM => Consistency::QUORUM,
            CassConsistency::CASS_CONSISTENCY_ALL => Consistency::ALL,
            CassConsistency::CASS_CONSISTENCY_LOCAL_QUORUM => Consistency::LOCAL_QUORUM,
            CassConsistency::CASS_CONSISTENCY_EACH_QUORUM => Consistency::EACH_QUORUM,
            CassConsistency::CASS_CONSISTENCY_SERIAL => Consistency::SERIAL,
            CassConsistency::CASS_CONSISTENCY_LOCAL_SERIAL => Consistency::LOCAL_SERIAL,
            CassConsistency::CASS_CONSISTENCY_LOCAL_ONE => Consistency::LOCAL_ONE,
        }
    }

    fn inner(&self) -> CassConsistency {
        match *self {
            Consistency::UNKNOWN => CassConsistency::CASS_CONSISTENCY_UNKNOWN,
            Consistency::ANY => CassConsistency::CASS_CONSISTENCY_ANY,
            Consistency::ONE => CassConsistency::CASS_CONSISTENCY_ONE,
            Consistency::TWO => CassConsistency::CASS_CONSISTENCY_TWO,
            Consistency::THREE => CassConsistency::CASS_CONSISTENCY_THREE,
            Consistency::QUORUM => CassConsistency::CASS_CONSISTENCY_QUORUM,
            Consistency::ALL => CassConsistency::CASS_CONSISTENCY_ALL,
            Consistency::LOCAL_QUORUM => CassConsistency::CASS_CONSISTENCY_LOCAL_QUORUM,
            Consistency::EACH_QUORUM => CassConsistency::CASS_CONSISTENCY_EACH_QUORUM,
            Consistency::SERIAL => CassConsistency::CASS_CONSISTENCY_SERIAL,
            Consistency::LOCAL_SERIAL => CassConsistency::CASS_CONSISTENCY_LOCAL_SERIAL,
            Consistency::LOCAL_ONE => CassConsistency::CASS_CONSISTENCY_LOCAL_ONE,
        }
    }
}