use cassandra::util::Protected;
use cassandra_sys::CassMetrics as _CassMetrics;

/// A view into server metrics FIXME not meaingfully implemented
pub struct SessionMetrics(*const _CassMetrics);


impl Protected<*const _CassMetrics> for SessionMetrics {
    fn inner(&self) -> *const _CassMetrics { self.0 }
    fn build(inner: *const _CassMetrics) -> Self { SessionMetrics(inner) }
}
