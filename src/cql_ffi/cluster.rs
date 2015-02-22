#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;
use libc::types::os::arch::c95::c_uint;

use cql_ffi::ssl::CassSsl;
use cql_bindgen::CassCluster as _CassCluster;
use cql_ffi::helpers::str_to_ref;
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

use cql_ffi::error::CassError;

pub struct CassCluster(pub *mut _CassCluster);

impl Drop for CassCluster {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

pub struct ContactPoints(*const c_char);

pub trait AsContactPoints {
    fn as_contact_points(&self) -> ContactPoints;
}

impl AsContactPoints for str {
    fn as_contact_points(&self) -> ContactPoints {
        let cstr = CString::new(self).unwrap();
        let bytes = cstr.as_bytes_with_nul();
        let ptr = bytes.as_ptr();
        ContactPoints(ptr as *const i8)
    }
}

impl CassCluster {

    pub fn new() -> CassCluster {unsafe{CassCluster(cass_cluster_new())}}

    unsafe fn free(&mut self){cass_cluster_free(self.0)}

    
    pub fn set_contact_points(self, contact_points: ContactPoints) -> Result<Self,CassError> {unsafe{
        let err:CassError = CassError::build(cass_cluster_set_contact_points(self.0,contact_points.0));
        err.wrap(self)
    }}

    pub unsafe fn set_port<'a>(&'a mut self, port: c_int) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_port(self.0,port)).wrap(self)}

    pub unsafe fn set_ssl(&mut self, ssl: &mut CassSsl) {cass_cluster_set_ssl(self.0,ssl.0)}

    pub unsafe fn set_protocol_version<'a>(&'a mut self, protocol_version: c_int) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_protocol_version(self.0,protocol_version)).wrap(self)}

    pub unsafe fn set_num_threads_io(&mut self, num_threads: c_uint) {cass_cluster_set_num_threads_io(self.0,num_threads);}

    pub unsafe fn set_queue_size_io<'a>(&'a mut self, queue_size: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_queue_size_io(self.0,queue_size)).wrap(self)}

    pub unsafe fn set_queue_size_event<'a>(&'a mut self, queue_size: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_queue_size_event(self.0,queue_size)).wrap(self)}

    pub unsafe fn set_queue_size_log<'a>(&'a mut self, queue_size: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_queue_size_log(self.0,queue_size)).wrap(self)}

    pub unsafe fn set_core_connections_per_host<'a>(&'a mut self, num_connections: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_core_connections_per_host(self.0,num_connections)).wrap(self)}

    pub unsafe fn set_max_connections_per_host<'a>(&'a mut self, num_connections: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_max_connections_per_host(self.0,num_connections)).wrap(self)}

    pub unsafe fn set_reconnect_wait_time(&mut self, wait_time: c_uint) {cass_cluster_set_reconnect_wait_time(self.0,wait_time)}

    pub unsafe fn set_max_concurrent_creation<'a>(&'a mut self, num_connections: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_max_concurrent_creation(self.0,num_connections)).wrap(self)}

    pub unsafe fn set_max_concurrent_requests_threshold<'a>(&'a mut self, num_requests: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_max_concurrent_requests_threshold(self.0,num_requests)).wrap(self)}

    pub unsafe fn set_max_requests_per_flush<'a>(&'a mut self, num_requests: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_max_requests_per_flush(self.0,num_requests)).wrap(self)}

    pub unsafe fn set_write_bytes_high_water_mark<'a>(&'a mut self, num_bytes: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_write_bytes_high_water_mark(self.0,num_bytes)).wrap(self)}

    pub unsafe fn set_write_bytes_low_water_mark<'a>(&'a mut self, num_bytes: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_write_bytes_low_water_mark(self.0,num_bytes)).wrap(self)}

    pub unsafe fn set_pending_requests_high_water_mark<'a>(&'a mut self, num_requests: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_pending_requests_high_water_mark(self.0,num_requests)).wrap(self)}

    pub unsafe fn set_pending_requests_low_water_mark<'a>(&'a mut self, num_requests: c_uint) -> Result<&'a mut Self,CassError> {CassError::build(cass_cluster_set_pending_requests_low_water_mark(self.0,num_requests)).wrap(self)}

    pub unsafe fn set_connect_timeout(&mut self, timeout_ms: c_uint) {cass_cluster_set_connect_timeout(self.0,timeout_ms)}

    pub unsafe fn set_request_timeout(&mut self, timeout_ms: c_uint) {cass_cluster_set_request_timeout(self.0,timeout_ms)}

    pub unsafe fn set_credentials(&mut self, username: *const c_char, password: *const c_char) {cass_cluster_set_credentials(self.0,username,password)}

    pub fn set_load_balance_round_robin(self) -> Result<Self,CassError> {unsafe{
        cass_cluster_set_load_balance_round_robin(self.0);
        CassError::build(0).wrap(self)
    }}

    pub fn set_load_balance_dc_aware(self, local_dc: &str,used_hosts_per_remote_dc: u32,allow_remote_dcs_for_local_cl: bool) -> Result<Self,CassError> {unsafe{
        CassError::build(
            cass_cluster_set_load_balance_dc_aware(self.0,str_to_ref(local_dc),used_hosts_per_remote_dc,if allow_remote_dcs_for_local_cl {1} else {0})
        ).wrap(self)
    }}

    pub unsafe fn set_token_aware_routing<'a>(&'a mut self, enabled: bool) {cass_cluster_set_token_aware_routing(self.0,if enabled {1} else {0})}

    pub unsafe fn set_tcp_nodelay(&mut self, enable: bool) {cass_cluster_set_tcp_nodelay(self.0,if enable {1} else {0})}

    pub unsafe fn set_tcp_keepalive(&mut self, enable: bool, delay_secs: ::libc::c_uint) {cass_cluster_set_tcp_keepalive(self.0,if enable {1} else {0},delay_secs)}
}
