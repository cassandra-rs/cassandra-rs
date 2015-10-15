#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use std::ffi::CString;

use cql_ffi::batch::Batch;
use cql_ffi::future::Future;
use cql_ffi::future::ResultFuture;
use cql_ffi::future::PreparedFuture;
use cql_ffi::error::CassandraError;
use cql_ffi::statement::Statement;
use cql_ffi::schema::Schema;
use cql_ffi::cluster::Cluster;
use cql_bindgen::CassFuture as _Future;
use cql_bindgen::cass_future_free;
use cql_bindgen::cass_future_wait;
use cql_bindgen::cass_future_error_code;

use cql_bindgen::CassSession as _Session;
use cql_bindgen::cass_session_new;
use cql_bindgen::cass_session_free;
use cql_bindgen::cass_session_close;
use cql_bindgen::cass_session_connect;
use cql_bindgen::cass_session_prepare;
use cql_bindgen::cass_session_execute;
use cql_bindgen::cass_session_execute_batch;
use cql_bindgen::cass_session_get_schema;
use cql_bindgen::cass_session_connect_keyspace;

pub struct Session(pub *mut _Session);

unsafe impl Sync for Session{}
unsafe impl Send for Session{}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            cass_session_free(self.0)
        }
    }
}

impl Session {
    /// Create a new Cassanda session.
    /// It's recommended to use Cluster.connect() instead
    pub fn new() -> Session {
        unsafe {
            Session(cass_session_new())
        }
    }

    pub fn close(self) -> Future {
        unsafe {
            Future(cass_session_close(self.0))
        }
    }

    pub fn connect(self, cluster: &Cluster) -> SessionFuture {
        unsafe {
            SessionFuture(cass_session_connect(self.0, cluster.0), self)
        }
    }

    pub fn prepare(&self, query: &str) -> Result<PreparedFuture, CassandraError> {
        unsafe {
            let query = CString::new(query).unwrap();
            Ok(PreparedFuture(cass_session_prepare(self.0, query.as_ptr())))
        }
    }

    pub fn execute(&self, statement: &str, parameter_count: u64) -> ResultFuture {
        unsafe {
            ResultFuture(cass_session_execute(self.0,
                                              Statement::new(statement,parameter_count).0))
        }
    }

    pub fn execute_statement(&self, statement: &Statement) -> ResultFuture {
        unsafe {
            ResultFuture(cass_session_execute(self.0, statement.0))
        }
    }

    pub fn execute_batch(&self, batch: Batch) -> ResultFuture {
        ResultFuture(unsafe {
                cass_session_execute_batch(self.0, batch.0)
            })
    }

    pub fn get_schema(&self) -> Schema {
        unsafe {
            Schema(cass_session_get_schema(self.0))
        }
    }

    pub unsafe fn connect_keyspace(&self,
                                   cluster: Cluster,
                                   keyspace: *const ::libc::c_char)
                                   -> Future {
        Future(cass_session_connect_keyspace(self.0, cluster.0, keyspace))
    }
}

pub struct SessionFuture(pub *mut _Future, pub Session);

impl SessionFuture {
    pub fn wait(self) -> Result<Session, CassandraError> {
        unsafe {
            cass_future_wait(self.0);
            self.error_code()
        }
    }

    fn error_code(self) -> Result<Session, CassandraError> {
        unsafe {
            let code = cass_future_error_code(self.0);
            cass_future_free(self.0);
            CassandraError::build(code).wrap(self.1)
        }
    }
}
