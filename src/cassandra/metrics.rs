use cassandra_sys::CassMetrics as _CassMetrics;

pub struct SessionMetrics(pub *const _CassMetrics);
