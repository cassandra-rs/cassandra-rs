use cql_bindgen::CassMetrics as _CassMetrics;

pub struct SessionMetrics(pub *const _CassMetrics);
