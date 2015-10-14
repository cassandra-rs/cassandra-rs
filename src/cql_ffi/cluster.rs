use cql_ffi::ssl::CassSsl;
use cql_bindgen::CassCluster as _CassCluster;
use std::ffi::CString;

use cql_bindgen::cass_cluster_new;
use cql_bindgen::cass_cluster_free;
use cql_bindgen::cass_cluster_set_contact_points;
use cql_bindgen::cass_cluster_set_port;
use cql_bindgen::cass_cluster_set_ssl;
use cql_bindgen::cass_cluster_set_protocol_version;
use cql_bindgen::cass_cluster_set_num_threads_io;
use cql_bindgen::cass_cluster_set_queue_size_io;
use cql_bindgen::cass_cluster_set_queue_size_event;
use cql_bindgen::cass_cluster_set_queue_size_log;
use cql_bindgen::cass_cluster_set_core_connections_per_host;
use cql_bindgen::cass_cluster_set_max_connections_per_host;
use cql_bindgen::cass_cluster_set_reconnect_wait_time;
use cql_bindgen::cass_cluster_set_max_concurrent_creation;
use cql_bindgen::cass_cluster_set_max_concurrent_requests_threshold;
use cql_bindgen::cass_cluster_set_max_requests_per_flush;
use cql_bindgen::cass_cluster_set_write_bytes_high_water_mark;
use cql_bindgen::cass_cluster_set_write_bytes_low_water_mark;
use cql_bindgen::cass_cluster_set_pending_requests_high_water_mark;
use cql_bindgen::cass_cluster_set_pending_requests_low_water_mark;
use cql_bindgen::cass_cluster_set_tcp_keepalive;
use cql_bindgen::cass_cluster_set_tcp_nodelay;
use cql_bindgen::cass_cluster_set_token_aware_routing;
use cql_bindgen::cass_cluster_set_load_balance_dc_aware;
use cql_bindgen::cass_cluster_set_load_balance_round_robin;
use cql_bindgen::cass_cluster_set_credentials;
use cql_bindgen::cass_cluster_set_request_timeout;
use cql_bindgen::cass_cluster_set_connect_timeout;
use cql_bindgen::cass_cluster_set_latency_aware_routing;
use cql_bindgen::cass_cluster_set_latency_aware_routing_settings;

use cql_ffi::error::CassError;

use cql_ffi::session::CassSession;

pub struct CassCluster(pub *mut _CassCluster);

impl Drop for CassCluster {
    fn drop(&mut self) {
        unsafe {
            cass_cluster_free(self.0)
        }
    }
}

impl CassCluster {

    pub fn new() -> CassCluster {
        unsafe {
            CassCluster(cass_cluster_new())
        }
    }

    pub fn set_contact_points<S>(&mut self, contact_points: S) -> Result<&mut Self, CassError>
        where S: Into<String>
    {
        unsafe {
            let s = CString::new(contact_points.into()).unwrap();
            let err = CassError::build(cass_cluster_set_contact_points(self.0, s.as_ptr()));
            err.wrap(self)
        }
    }

    pub fn set_port(&mut self, port: i32) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_cluster_set_port(self.0,port)).wrap(self)
        }
    }

    pub fn set_ssl(&mut self, ssl: &mut CassSsl) -> &Self {
        unsafe {
            cass_cluster_set_ssl(self.0, ssl.0);
            self
        }
    }
    /// Connect to Cassandra cluster
    pub fn connect(&mut self) -> Result<CassSession, CassError> {
        CassSession::new().connect(&self).wait()
    }

    pub fn set_protocol_version(&mut self, protocol_version: i32) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_cluster_set_protocol_version(self.0,protocol_version)).wrap(self)
        }
    }

    pub fn set_num_threads_io(&mut self, num_threads: u32) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_cluster_set_num_threads_io(self.0,num_threads)).wrap(self)
        }
    }

    pub unsafe fn set_queue_size_io(&mut self, queue_size: u32) -> Result<&mut Self, CassError> {
        CassError::build(cass_cluster_set_queue_size_io(self.0,queue_size)).wrap(self)
    }

    pub unsafe fn set_queue_size_event(&mut self, queue_size: u32) -> Result<&mut Self, CassError> {
        CassError::build(cass_cluster_set_queue_size_event(self.0,queue_size)).wrap(self)
    }

    pub unsafe fn set_queue_size_log(&mut self, queue_size: u32) -> Result<&mut Self, CassError> {
        CassError::build(cass_cluster_set_queue_size_log(self.0,queue_size)).wrap(self)
    }

    pub unsafe fn set_core_connections_per_host(&mut self,
                                                num_connections: u32)
                                                -> Result<&mut Self, CassError> {
        CassError::build(
            cass_cluster_set_core_connections_per_host(
                self.0,
                num_connections
            )
        ).wrap(self)
    }

    pub unsafe fn set_max_connections_per_host(&mut self,
                                               num_connections: u32)
                                               -> Result<&mut Self, CassError> {
        CassError::build(
            cass_cluster_set_max_connections_per_host(
                self.0,
                num_connections
            )
        ).wrap(self)
    }

    pub unsafe fn set_reconnect_wait_time(&mut self, wait_time: u32) -> &Self {
        cass_cluster_set_reconnect_wait_time(self.0, wait_time);
        self
    }

    pub unsafe fn set_max_concurrent_creation(&mut self,
                                              num_connections: u32)
                                              -> Result<&mut Self, CassError> {
        CassError::build(
            cass_cluster_set_max_concurrent_creation(
                self.0,
                num_connections
            )
        ).wrap(self)
    }

    pub unsafe fn set_max_concurrent_requests_threshold(&mut self,
                                                        num_requests: u32)
                                                        -> Result<&mut Self, CassError> {
        CassError::build(
            cass_cluster_set_max_concurrent_requests_threshold(
                self.0,
                num_requests
            )
        ).wrap(self)
    }

    pub unsafe fn set_max_requests_per_flush(&mut self,
                                             num_requests: u32)
                                             -> Result<&mut Self, CassError> {
        CassError::build(cass_cluster_set_max_requests_per_flush(self.0,num_requests)).wrap(self)
    }

    pub unsafe fn set_write_bytes_high_water_mark(&mut self,
                                                  num_bytes: u32)
                                                  -> Result<&mut Self, CassError> {
        CassError::build(cass_cluster_set_write_bytes_high_water_mark(self.0,num_bytes)).wrap(self)
    }

    pub unsafe fn set_write_bytes_low_water_mark(&mut self,
                                                 num_bytes: u32)
                                                 -> Result<&mut Self, CassError> {
        CassError::build(cass_cluster_set_write_bytes_low_water_mark(self.0,num_bytes)).wrap(self)
    }

    pub unsafe fn set_pending_requests_high_water_mark(&mut self,
                                                       num_requests: u32)
                                                       -> Result<&mut Self, CassError> {
        CassError::build(
            cass_cluster_set_pending_requests_high_water_mark(
                self.0,num_requests
            )
        ).wrap(self)
    }

    pub unsafe fn set_pending_requests_low_water_mark(&mut self,
                                                      num_requests: u32)
                                                      -> Result<&mut Self, CassError> {
        CassError::build(
            cass_cluster_set_pending_requests_low_water_mark(
                self.0,
                num_requests
            )
        ).wrap(self)
    }

    pub unsafe fn set_connect_timeout(&mut self, timeout_ms: u32) -> &Self {
        cass_cluster_set_connect_timeout(self.0, timeout_ms);
        self
    }

    pub unsafe fn set_request_timeout(&mut self, timeout_ms: u32) -> &Self {
        cass_cluster_set_request_timeout(self.0, timeout_ms);
        self
    }

    pub unsafe fn set_credentials(&mut self, username: *const i8, password: *const i8) -> &Self {
        cass_cluster_set_credentials(self.0, username, password);
        self
    }

    pub fn set_load_balance_round_robin(&mut self) -> Result<&Self, CassError> {
        unsafe {
            cass_cluster_set_load_balance_round_robin(self.0);
            CassError::build(0).wrap(self)
        }
    }

    pub fn set_load_balance_dc_aware<S>(&mut self,
                                        local_dc: S,
                                        used_hosts_per_remote_dc: u32,
                                        allow_remote_dcs_for_local_cl: bool)
                                        -> Result<&Self, CassError>
        where S: Into<String>
    {
        unsafe {
            CassError::build({
            let local_dc = CString::new(local_dc.into()).unwrap();
            cass_cluster_set_load_balance_dc_aware(
                self.0,local_dc.as_ptr(),
                used_hosts_per_remote_dc,
                if allow_remote_dcs_for_local_cl {1} else {0}
            )
        }).wrap(self)
        }
    }

    pub unsafe fn set_token_aware_routing(&mut self, enabled: bool) -> &Self {
        cass_cluster_set_token_aware_routing(self.0,
                                             if enabled {
                1
            } else {
                0
            });
        self
    }

    pub fn set_tcp_nodelay(&mut self, enable: bool) -> &Self {
        unsafe {
            cass_cluster_set_tcp_nodelay(self.0,
                                         if enable {
                    1
                } else {
                    0
                });
            self
        }
    }

    pub unsafe fn set_tcp_keepalive(&mut self, enable: bool, delay_secs: u32) -> &Self {
        cass_cluster_set_tcp_keepalive(self.0,
                                       if enable {
                1
            } else {
                0
            },
                                       delay_secs);
        self
    }

    pub fn set_latency_aware_routing(&mut self, enabled: bool) -> &Self {
        unsafe {
            cass_cluster_set_latency_aware_routing(self.0,
                                                   if enabled {
                    1
                } else {
                    0
                });
            self
        }
    }

    pub fn cass_cluster_set_latency_aware_routing_settings(&mut self,
                                                           exclusion_threshold: f64,
                                                           scale_ms: u64,
                                                           retry_period_ms: u64,
                                                           update_rate_ms: u64,
                                                           min_measured: u64)
                                                           -> &Self {
        unsafe {
            cass_cluster_set_latency_aware_routing_settings(self.0,
                                                            exclusion_threshold,
                                                            scale_ms,
                                                            retry_period_ms,
                                                            update_rate_ms,
                                                            min_measured);
            self
        }
    }

}
