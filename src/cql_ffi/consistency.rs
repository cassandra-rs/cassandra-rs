#![allow(dead_code)]
#![allow(non_camel_case_types)]

use cql_bindgen::CassConsistency as _CassConsistency;

pub struct CassConsistency(pub _CassConsistency);
