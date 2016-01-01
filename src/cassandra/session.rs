#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use std::mem;

use std::ffi::CString;

use cassandra::batch::Batch;
use cassandra::future::Future;
use cassandra::future::ResultFuture;
use cassandra::future::PreparedFuture;
use cassandra::error::CassError;
use cassandra::statement::Statement;
use cassandra::schema::schema_meta::SchemaMeta;
use cassandra::metrics::SessionMetrics;
use cassandra::cluster::Cluster;
use cassandra_sys::CassFuture as _Future;
use cassandra_sys::cass_future_free;
use cassandra_sys::cass_future_wait;
use cassandra_sys::cass_future_error_code;

use cassandra_sys::CassSession as _Session;
use cassandra_sys::cass_session_new;
use cassandra_sys::cass_session_free;
use cassandra_sys::cass_session_close;
use cassandra_sys::cass_session_connect;
use cassandra_sys::cass_session_prepare;
use cassandra_sys::cass_session_execute;
use cassandra_sys::cass_session_execute_batch;
use cassandra_sys::cass_session_get_schema_meta;
use cassandra_sys::cass_session_connect_keyspace;
use cassandra_sys::cass_session_get_metrics;

pub struct Session(pub *mut _Session);

unsafe impl Sync for Session {}
unsafe impl Send for Session {}

impl Drop for Session {
    /// Frees a session instance. If the session is still connected it will be synchronously
    /// closed before being deallocated.
    fn drop(&mut self) {
        unsafe { cass_session_free(self.0) }
    }
}

impl Session {
    /// Create a new Cassanda session.
    /// It's recommended to use Cluster.connect() instead
    // FIXME find a way to hide this from external api
    pub fn new() -> Session {
        unsafe { Session(cass_session_new()) }
    }

    /// Connects a session.
    pub fn connect(self, cluster: &Cluster) -> SessionFuture {
        unsafe { SessionFuture(cass_session_connect(self.0, cluster.0), self) }
    }

    ///Connects a session and sets the keyspace.
    pub fn connect_keyspace(&self, cluster: Cluster, keyspace: *const ::libc::c_char) -> Future {
        unsafe { Future(cass_session_connect_keyspace(self.0, cluster.0, keyspace)) }
    }

    ///Closes the session instance, outputs a close future which can
    ///be used to determine when the session has been terminated. This allows
    ///in-flight requests to finish.
    pub fn close(self) -> Future {
        unsafe { Future(cass_session_close(self.0)) }
    }

    ///Create a prepared statement.
    pub fn prepare(&self, query: &str) -> Result<PreparedFuture, CassError> {
        unsafe {
            let query = CString::new(query).unwrap();
            Ok(PreparedFuture(cass_session_prepare(self.0, query.as_ptr())))
        }
    }

    ///Execute a query or bound statement.
    pub fn execute(&self, statement: &str, parameter_count: u64) -> ResultFuture {
        unsafe { ResultFuture(cass_session_execute(self.0, Statement::new(statement, parameter_count).0)) }
    }

    /// Execute a batch statement.
    pub fn execute_batch(&self, batch: Batch) -> ResultFuture {
        ResultFuture(unsafe { cass_session_execute_batch(self.0, batch.0) })
    }

    /// Execute a statement.
    pub fn execute_statement(&self, statement: &Statement) -> ResultFuture {
        unsafe { ResultFuture(cass_session_execute(self.0, statement.0)) }
    }

    ///Gets a snapshot of this session's schema metadata. The returned
    ///snapshot of the schema metadata is not updated. This function
    ///must be called again to retrieve any schema changes since the
    ///previous call.
    pub fn get_schema_meta(&self) -> SchemaMeta {
        unsafe { SchemaMeta(cass_session_get_schema_meta(self.0)) }
    }

    ///Gets a copy of this session's performance/diagnostic metrics.
    pub fn get_metrics(&self) -> SessionMetrics {
        unsafe {
            let mut metrics = mem::zeroed();
            cass_session_get_metrics(self.0, &mut metrics);
            SessionMetrics(&metrics)
        }
    }

    //    pub fn get_schema(&self) -> Schema {
    //        unsafe { Schema(cass_session_get_schema(self.0)) }
    //    }
}

pub struct SessionFuture(pub *mut _Future, pub Session);

impl SessionFuture {
    pub fn wait(self) -> Result<Session, CassError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    fn error_code(self) -> Result<Session, CassError> {
        unsafe {
            let code = cass_future_error_code(self.0);
            cass_future_free(self.0);
            CassError::build(code).wrap(self.1)
        }
    }
}
