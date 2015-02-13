#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::batch::CassBatch;
use cql_ffi::future::CassFuture;
use cql_ffi::string::CassString;
use cql_ffi::statement::CassStatement;
use cql_ffi::schema::CassSchema;
use cql_ffi::cluster::CassCluster;

pub enum CassSession { }

extern "C" {
    pub fn cass_session_new() -> *mut CassSession;
    pub fn cass_session_free(session: *mut CassSession);
    pub fn cass_session_close(session: *mut CassSession) -> *mut CassFuture;
    pub fn cass_session_connect(session: *mut CassSession, cluster: *mut CassCluster) -> *mut CassFuture;
    pub fn cass_session_prepare(session: *mut CassSession, query: CassString) -> *mut CassFuture;
    pub fn cass_session_execute(session: *mut CassSession, statement: *const CassStatement) -> *mut CassFuture;
    pub fn cass_session_execute_batch(session: *mut CassSession, batch: *const CassBatch) -> *mut CassFuture;
    pub fn cass_session_get_schema(session: *mut CassSession) -> *const CassSchema;
    pub fn cass_session_connect_keyspace(session: *mut CassSession, cluster: *const CassCluster, keyspace: *const ::libc::c_char) -> *mut CassFuture;
}
