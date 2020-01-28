use crate::cassandra::util::Protected;
use crate::cassandra_sys::CassMetrics as _CassMetrics;

/// Metrics about the current session.
#[allow(missing_docs)] // See DataStax docs
#[derive(Debug)]
pub struct SessionMetrics {
    pub min_us: u64,
    pub max_us: u64,
    pub mean_us: u64,
    pub stddev_us: u64,
    pub median_us: u64,
    pub percentile_75th_us: u64,
    pub percentile_95th_us: u64,
    pub percentile_98th_us: u64,
    pub percentile_99th_us: u64,
    pub percentile_999th_us: u64,

    pub mean_rate_per_sec: f64,
    pub one_minute_rate_per_seq: f64,
    pub five_minute_rate_per_sec: f64,
    pub fifteen_minute_rate_per_sec: f64,

    pub total_connections: u64,
    pub available_connections: u64,
    pub exceeded_pending_requests_water_mark: u64,
    pub exceeded_write_bytes_water_mark: u64,

    pub connection_timeouts: u64,
    pub pending_request_timeouts: u64,
    pub request_timeouts: u64,
}

impl SessionMetrics {
    /// Build session metrics from underlying type.
    pub(crate) fn build(inner: *const _CassMetrics) -> Self {
        let inner = unsafe { &*inner };
        SessionMetrics {
            min_us: inner.requests.min,
            max_us: inner.requests.max,
            mean_us: inner.requests.mean,
            stddev_us: inner.requests.stddev,
            median_us: inner.requests.median,
            percentile_75th_us: inner.requests.percentile_75th,
            percentile_95th_us: inner.requests.percentile_95th,
            percentile_98th_us: inner.requests.percentile_98th,
            percentile_99th_us: inner.requests.percentile_99th,
            percentile_999th_us: inner.requests.percentile_999th,
            mean_rate_per_sec: inner.requests.mean_rate,
            one_minute_rate_per_seq: inner.requests.one_minute_rate,
            five_minute_rate_per_sec: inner.requests.five_minute_rate,
            fifteen_minute_rate_per_sec: inner.requests.fifteen_minute_rate,
            total_connections: inner.stats.total_connections,
            available_connections: inner.stats.available_connections,
            exceeded_pending_requests_water_mark: inner.stats.exceeded_pending_requests_water_mark,
            exceeded_write_bytes_water_mark: inner.stats.exceeded_write_bytes_water_mark,
            connection_timeouts: inner.errors.connection_timeouts,
            pending_request_timeouts: inner.errors.pending_request_timeouts,
            request_timeouts: inner.errors.request_timeouts,
        }
    }
}
