#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use crate::cassandra::cluster::Cluster;
use crate::cassandra::error::*;
use crate::cassandra::future::CassFuture;
use crate::cassandra::metrics::SessionMetrics;
use crate::cassandra::prepared::PreparedStatement;
use crate::cassandra::result::CassResult;
use crate::cassandra::schema::schema_meta::SchemaMeta;
use crate::cassandra::statement::Statement;
use crate::cassandra::util::{Protected, ProtectedInner};
use crate::{cassandra::batch::Batch, BatchType};

use crate::cassandra_sys::cass_session_close;
use crate::cassandra_sys::cass_session_connect;
use crate::cassandra_sys::cass_session_connect_keyspace_n;
use crate::cassandra_sys::cass_session_execute;
use crate::cassandra_sys::cass_session_execute_batch;
use crate::cassandra_sys::cass_session_free;
use crate::cassandra_sys::cass_session_get_metrics;
use crate::cassandra_sys::cass_session_get_schema_meta;
use crate::cassandra_sys::cass_session_new;
use crate::cassandra_sys::cass_session_prepare_n;
use crate::cassandra_sys::CassSession as _Session;

use std::ffi::NulError;
use std::mem;
use std::os::raw::c_char;
use std::sync::Arc;

#[derive(Debug)]
struct SessionInner(*mut _Session);

// The underlying C type has no thread-local state, and explicitly supports access
// from multiple threads: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for SessionInner {}
unsafe impl Sync for SessionInner {}

impl SessionInner {
    fn new(inner: *mut _Session) -> Arc<Self> {
        Arc::new(Self(inner))
    }
}

/// A session object is used to execute queries and maintains cluster state through
/// the control connection. The control connection is used to auto-discover nodes and
/// monitor cluster changes (topology and schema). Each session also maintains multiple
/// /pools of connections to cluster nodes which are used to query the cluster.
///
/// Instances of the session object are thread-safe to execute queries.
#[derive(Debug, Clone)]
pub struct Session(Arc<SessionInner>);

impl ProtectedInner<*mut _Session> for SessionInner {
    fn inner(&self) -> *mut _Session {
        self.0
    }
}

impl ProtectedInner<*mut _Session> for Session {
    fn inner(&self) -> *mut _Session {
        self.0.inner()
    }
}

impl Protected<*mut _Session> for Session {
    fn build(inner: *mut _Session) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        Session(SessionInner::new(inner))
    }
}

impl Drop for SessionInner {
    /// Frees a session instance. If the session is still connected it will be synchronously
    /// closed before being deallocated.
    fn drop(&mut self) {
        unsafe { cass_session_free(self.0) }
    }
}

impl Default for Session {
    fn default() -> Session {
        Session::new()
    }
}

impl Session {
    /// Create a new Cassanda session.
    /// It's recommended to use Cluster.connect() instead
    pub fn new() -> Session {
        unsafe { Session(SessionInner::new(cass_session_new())) }
    }

    /// Connects a session.
    pub fn connect(self, cluster: &Cluster) -> CassFuture<Self> {
        let connect = unsafe { cass_session_connect(self.inner(), cluster.inner()) };
        <CassFuture<Self>>::build(self, connect)
    }

    /// Connects a session and sets the keyspace.
    pub fn connect_keyspace(self, cluster: &Cluster, keyspace: &str) -> Result<CassFuture<Self>> {
        let connect_keyspace = unsafe {
            let keyspace_ptr = keyspace.as_ptr() as *const c_char;
            cass_session_connect_keyspace_n(
                self.inner(),
                cluster.inner(),
                keyspace_ptr,
                keyspace.len(),
            )
        };
        Ok(<CassFuture<Self>>::build(self, connect_keyspace))
    }

    /// Closes the session instance, outputs a close future which can
    /// be used to determine when the session has been terminated. This allows
    /// in-flight requests to finish.
    pub fn close(self) -> CassFuture<()> {
        unsafe {
            let close = cass_session_close(self.inner());
            <CassFuture<()>>::build(self, close)
        }
    }

    /// Create a prepared statement with the given query.
    pub async fn prepare(&self, query: &str) -> Result<PreparedStatement> {
        let future = unsafe {
            let query_ptr = query.as_ptr() as *const c_char;
            <CassFuture<PreparedStatement>>::build(
                self.clone(),
                cass_session_prepare_n(self.inner(), query_ptr, query.len()),
            )
        };
        future.await
    }

    /// Creates a statement with the given query.
    pub fn statement(&self, query: &str) -> Statement {
        let param_count = query.matches("?").count();
        Statement::new(self.clone(), query, param_count)
    }

    //    ///Execute a query or bound statement.
    //    pub fn execute(&self, statement: &str, parameter_count: u64) -> CassFuture {
    //        unsafe {
    //            CassFuture::build(cass_session_execute(self.0,
    //            Statement::new(statement, parameter_count).inner()))
    //        }
    //    }

    /// Creates a new batch that is bound to this session.
    pub fn batch(&self, batch_type: BatchType) -> Batch {
        Batch::new(batch_type, self.clone())
    }

    /// Gets a snapshot of this session's schema metadata. The returned
    /// snapshot of the schema metadata is not updated. This function
    /// must be called again to retrieve any schema changes since the
    /// previous call.
    pub fn get_schema_meta(&self) -> SchemaMeta {
        unsafe { SchemaMeta::build(cass_session_get_schema_meta(self.inner())) }
    }

    /// Gets a copy of this session's performance/diagnostic metrics.
    pub fn get_metrics(&self) -> SessionMetrics {
        unsafe {
            let mut metrics = mem::zeroed();
            cass_session_get_metrics(self.inner(), &mut metrics);
            SessionMetrics::build(&metrics)
        }
    }

    //    pub fn get_schema(&self) -> Schema {
    //        unsafe { Schema(cass_session_get_schema(self.0)) }
    //    }
}
