#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::batch::CassBatch;
use cql_ffi::future::CassFuture;
use cql_ffi::string::CassString;
use cql_ffi::statement::CassStatement;
use cql_ffi::schema::CassSchema;
use cql_ffi::cluster::CassCluster;
use cql_bindgen::CassSession as _CassSession;
use cql_bindgen::cass_session_new;
use cql_bindgen::cass_session_free;
use cql_bindgen::cass_session_close;
use cql_bindgen::cass_session_connect;
use cql_bindgen::cass_session_prepare;
use cql_bindgen::cass_session_execute;
use cql_bindgen::cass_session_execute_batch;
use cql_bindgen::cass_session_get_schema;
use cql_bindgen::cass_session_connect_keyspace;

pub struct CassSession(pub *mut _CassSession);

impl CassSession {
    pub unsafe fn new() -> CassSession {CassSession(cass_session_new())}
    pub unsafe fn free(&mut self) {cass_session_free(self.0)}
    pub unsafe fn close(&mut self) -> CassFuture {CassFuture(cass_session_close(self.0))}
    pub unsafe fn connect(&mut self, cluster: &mut CassCluster) -> CassFuture {CassFuture(cass_session_connect(self.0, cluster.0))}
    pub unsafe fn prepare(&mut self, query: CassString) -> CassFuture {CassFuture(cass_session_prepare(self.0, query.0))}
    pub unsafe fn execute(&mut self, statement: CassStatement) -> CassFuture {CassFuture(cass_session_execute(self.0, statement.0))}
    pub unsafe fn execute_batch(&mut self, batch: &CassBatch) -> CassFuture {CassFuture(cass_session_execute_batch(self.0, batch.0))}
    pub unsafe fn get_schema(&mut self) -> CassSchema {CassSchema(cass_session_get_schema(self.0))}
    pub unsafe fn connect_keyspace(&mut self, cluster: CassCluster, keyspace: *const ::libc::c_char) -> CassFuture {CassFuture(cass_session_connect_keyspace(self.0, cluster.0, keyspace))}
}
