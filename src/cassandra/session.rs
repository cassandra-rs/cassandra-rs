#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use std::mem;

use std::ffi::CString;
use std::ffi::NulError;
use cassandra::batch::Batch;
use cassandra::future::{CloseFuture, Future, PreparedFuture, ResultFuture, SessionFuture};
use cassandra::error::CassError;
use cassandra::statement::Statement;
use cassandra::schema::schema_meta::SchemaMeta;
use cassandra::metrics::SessionMetrics;
use cassandra::cluster::Cluster;
use cassandra_sys::CassFuture as _Future;
use cassandra_sys::cass_future_free;
use cassandra_sys::cass_future_wait;
use cassandra_sys::cass_future_error_code;
use cassandra::cluster;
use cassandra::metrics;
use cassandra::schema;
use cassandra::future;
use cassandra::statement;

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

///A session object is used to execute queries and maintains cluster state through
///the control connection. The control connection is used to auto-discover nodes and
///monitor cluster changes (topology and schema). Each session also maintains multiple
/// /pools of connections to cluster nodes which are used to query the cluster.
///
///Instances of the session object are thread-safe to execute queries.
#[derive(Debug)]
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

pub mod protected {
    use cassandra::session::Session;
    use cassandra_sys::CassSession as _Session;
    pub fn build(session: *mut _Session) -> Session {
        Session(session)
    }
}

impl Session {
//    /// Create a new Cassanda session.
//    /// It's recommended to use Cluster.connect() instead
//    // FIXME find a way to hide this from external api
//    pub fn new() -> Session {
//        unsafe { Session(cass_session_new()) }
//    }
//
//    pub fn new2() -> *mut _Session {
//        unsafe { cass_session_new() }
//    }
    
    /// Connects a session.
    pub fn connect(self, cluster: &Cluster) -> SessionFuture {
        unsafe {
            future::protected::build_session_future(cass_session_connect(self.0, cluster::protected::inner(cluster)),
                                                    self.0)
        }
    }

    ///Connects a session and sets the keyspace.
    pub fn connect_keyspace(&self, cluster: &Cluster, keyspace: &str) -> Result<Future, NulError> {
        let keyspace = try!(CString::new(keyspace));
        unsafe {
            Ok(future::protected::build_future(cass_session_connect_keyspace(self.0,
                                                                             cluster::protected::inner(cluster),
                                                                             keyspace.as_ptr())))
        }
    }

    ///Closes the session instance, outputs a close future which can
    ///be used to determine when the session has been terminated. This allows
    ///in-flight requests to finish.
    pub fn close(self) -> CloseFuture {
        unsafe { future::protected::build_close(cass_session_close(self.0)) }
    }

    ///Create a prepared statement.
    pub fn prepare(&self, query: &str) -> Result<PreparedFuture, CassError> {
        unsafe {
            let query = CString::new(query).unwrap();
            Ok(future::protected::build_prepared_future(cass_session_prepare(self.0, query.as_ptr())))
        }
    }

    ///Execute a query or bound statement.
    pub fn execute(&self, statement: &str, parameter_count: u64) -> ResultFuture {
        unsafe {
            future::protected::build_result_future(cass_session_execute(self.0,
                                                                        statement::protected::inner(&Statement::new(statement, parameter_count))))
        }
    }

    /// Execute a batch statement.
    pub fn execute_batch(&self, batch: Batch) -> ResultFuture {
        use cassandra::batch;
        future::protected::build_result_future(unsafe {
            cass_session_execute_batch(self.0, batch::protected::inner(&batch))
        })
    }

    /// Execute a statement.
    pub fn execute_statement(&self, statement: &Statement) -> ResultFuture {
        unsafe { future::protected::build_result_future(cass_session_execute(self.0, statement::protected::inner(statement))) }
    }

    ///Gets a snapshot of this session's schema metadata. The returned
    ///snapshot of the schema metadata is not updated. This function
    ///must be called again to retrieve any schema changes since the
    ///previous call.
    pub fn get_schema_meta(&self) -> SchemaMeta {
        unsafe { schema::schema_meta::protected::build(cass_session_get_schema_meta(self.0)) }
    }

    ///Gets a copy of this session's performance/diagnostic metrics.
    pub fn get_metrics(&self) -> SessionMetrics {
        unsafe {
            let mut metrics = mem::zeroed();
            cass_session_get_metrics(self.0, &mut metrics);
            metrics::protected::build_session_metrics(&mut metrics)
        }
    }

    //    pub fn get_schema(&self) -> Schema {
    //        unsafe { Schema(cass_session_get_schema(self.0)) }
    //    }
}
