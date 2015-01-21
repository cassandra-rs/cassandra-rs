#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cass_batch::CassBatch;
use cass_future::CassFuture;
use cass_string::CassString;
use cass_statement::CassStatement;
use cass_schema::CassSchema;
use cass_cluster::CassCluster;

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
