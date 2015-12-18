use cql_bindgen::CassConsistency as _CassConsistency;

use cql_bindgen::cass_consistency_string;

pub struct Consistency(pub _CassConsistency);
