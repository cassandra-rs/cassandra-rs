use cassandra_sys::CassMetrics as _CassMetrics;

///A view into server metrics FIXME not meaingfully implemented
pub struct SessionMetrics(*const _CassMetrics);

pub mod protected {
    use cassandra::metrics::SessionMetrics;
    use cassandra_sys::CassMetrics as _CassMetrics;
    pub fn build_session_metrics(metrics: *const _CassMetrics) -> SessionMetrics {
        SessionMetrics(metrics)
    }
}
