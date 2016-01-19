use cassandra_sys::CassMetrics as _CassMetrics;
use cassandra::util::Protected;

///A view into server metrics FIXME not meaingfully implemented
pub struct SessionMetrics(*const _CassMetrics);


impl Protected<*const _CassMetrics> for SessionMetrics {
    fn inner(&self) -> *const _CassMetrics {
        self.0
    }
    fn build(inner: *const _CassMetrics) -> Self {
        SessionMetrics(inner)
    }
}

