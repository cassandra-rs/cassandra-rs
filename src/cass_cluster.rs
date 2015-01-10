#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;
use libc::types::os::arch::c95::c_uint;
use libc::types::common::c95::c_void;

use cass_future::CassFuture;
use cass_error::CassError;
use cass_ssl::CassSsl;
use cass_log::CassLogLevel;
use cass_log::CassLogCallback;
use cass_types::*;

enum Struct_CassCluster_ { }
pub type CassCluster = Struct_CassCluster_;

extern "C" {
    pub fn cass_cluster_new() -> *mut CassCluster;
    pub fn cass_cluster_free(cluster: *mut CassCluster);
    pub fn cass_cluster_set_contact_points(cluster: *mut CassCluster, contact_points: *const c_char) -> CassError;
    pub fn cass_cluster_set_port(cluster: *mut CassCluster, port: c_int) -> CassError;
    pub fn cass_cluster_set_ssl(cluster: *mut CassCluster, ssl: *mut CassSsl) -> CassError;
    pub fn cass_cluster_set_protocol_version(cluster: *mut CassCluster, protocol_version: c_int) -> CassError;
    pub fn cass_cluster_set_num_threads_io(cluster: *mut CassCluster, num_threads: c_uint) -> CassError;
    pub fn cass_cluster_set_queue_size_io(cluster: *mut CassCluster, queue_size: c_uint) -> CassError;
    pub fn cass_cluster_set_queue_size_event(cluster: *mut CassCluster, queue_size: c_uint) -> CassError;
    pub fn cass_cluster_set_queue_size_log(cluster: *mut CassCluster, queue_size: c_uint) -> CassError;
    pub fn cass_cluster_set_core_connections_per_host(cluster: *mut CassCluster, num_connections: c_uint) -> CassError;
    pub fn cass_cluster_set_max_connections_per_host(cluster: *mut CassCluster, num_connections: c_uint) -> CassError;
    pub fn cass_cluster_set_reconnect_wait_time(cluster: *mut CassCluster, wait_time: c_uint) -> CassError;
    pub fn cass_cluster_set_max_concurrent_creation(cluster: *mut CassCluster, num_connections: c_uint) -> CassError;
    pub fn cass_cluster_set_max_concurrent_requests_threshold(cluster: *mut CassCluster, num_requests: c_uint) -> CassError;
    pub fn cass_cluster_set_max_requests_per_flush(cluster: *mut CassCluster, num_requests: c_uint) -> CassError;
    pub fn cass_cluster_set_write_bytes_high_water_mark(cluster: *mut CassCluster, num_bytes: c_uint) -> CassError;
    pub fn cass_cluster_set_write_bytes_low_water_mark(cluster: *mut CassCluster, num_bytes: c_uint) -> CassError;
    pub fn cass_cluster_set_pending_requests_high_water_mark(cluster: *mut CassCluster, num_requests: c_uint) -> CassError;
    pub fn cass_cluster_set_pending_requests_low_water_mark(cluster: *mut CassCluster, num_requests: c_uint) -> CassError;
    pub fn cass_cluster_set_connect_timeout(cluster: *mut CassCluster, timeout_ms: c_uint) -> CassError;
    pub fn cass_cluster_set_request_timeout(cluster: *mut CassCluster, timeout_ms: c_uint) -> CassError;
    pub fn cass_cluster_set_log_level(cluster: *mut CassCluster, level: CassLogLevel) -> CassError;
    pub fn cass_cluster_set_log_callback(cluster: *mut CassCluster, callback: CassLogCallback, data: *mut c_void) -> CassError;
    pub fn cass_cluster_set_credentials(cluster: *mut CassCluster, username: *const c_char, password: *const c_char) -> CassError;
    pub fn cass_cluster_set_load_balance_round_robin(cluster: *mut CassCluster) -> CassError;
    pub fn cass_cluster_set_load_balance_dc_aware(cluster: *mut CassCluster, local_dc: *const c_char)-> CassError;
    pub fn cass_cluster_set_token_aware_routing(cluster: *mut CassCluster, enabled: cass_bool_t);
    pub fn cass_cluster_connect(cluster: *mut CassCluster) -> *mut CassFuture;
    pub fn cass_cluster_connect_keyspace(cluster: *mut CassCluster, keyspace: *const c_char) -> *mut CassFuture;
}
