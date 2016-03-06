//! This is a wrapper around the DataStax C++ driver for Cassandra. It aims to be 100% safe with minimal overhead added
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![warn(missing_docs)]

extern crate libc;
#[macro_use]
extern crate log;
extern crate decimal;
extern crate chrono;
extern crate time;
extern crate ip;
extern crate uuid;


pub use cassandra::consistency::Consistency;
// pub use cassandra::inet::{Inet};
pub use cassandra_sys::CASS_BATCH_TYPE_LOGGED;
pub use cassandra::batch::{Batch, BatchType, CustomPayload};
pub use cassandra::uuid::{Uuid, UuidGen};
pub use cassandra::cluster::{Cluster, ContactPoints, CqlProtocol};
pub use cassandra::session::Session;
pub use cassandra::statement::Statement;
pub use cassandra_sys::CassBatchType;
pub use cassandra::future::{CloseFuture, Future, FutureCallback, PreparedFuture, ResultFuture, SessionFuture};
pub use cassandra::prepared::PreparedStatement;
pub use cassandra::result::CassResult;
pub use cassandra::row::Row;
pub use cassandra::value::{Value, ValueType}; //FIXME this should not be exported
pub use cassandra::collection::{CassCollection, List, Map, Set};
pub use cassandra::ssl::Ssl;
pub use cassandra::schema::keyspace_meta::KeyspaceMeta;
pub use cassandra::schema::column_meta::ColumnMeta;
pub use cassandra::schema::schema_meta::SchemaMeta;
pub use cassandra::schema::table_meta::TableMeta;
pub use cassandra::schema::function_meta::FunctionMeta;
pub use cassandra::schema::aggregate_meta::AggregateMeta;
pub use cassandra::error::{CassError, CassErrorResult};
pub use cassandra::log::{LogLevel, set_callback, set_level};
pub use cassandra::row::AsRustType;
pub use cassandra::statement::BindRustType;
pub use cassandra::column::Column;
pub use cassandra::tuple::Tuple;
pub use cassandra::inet::Inet;
pub use cassandra::user_type::UserType;
pub use cassandra::data_type::DataType;
pub use cassandra::policy::retry::RetryPolicy;
// pub use cassandra::custom_payload::CustomPayload;
pub use cassandra::time::TimestampGen;
// pub use cassandra::util::*;
// pub use cassandra::metrics::*;
pub use cassandra::iterator::{AggregateIterator, ColumnIterator, FieldIterator, FunctionIterator, KeyspaceIterator,
                              MapIterator, SetIterator, TableIterator, UserTypeIterator};
// pub use cassandra::write_type::*;
pub use cassandra::field::Field;

extern crate cassandra_sys;


mod cassandra {
    pub mod consistency;
    pub mod field;
    pub mod inet;
    pub mod uuid;
    pub mod cluster;
    pub mod session;
    pub mod statement;
    pub mod batch;
    pub mod future;
    pub mod prepared;
    pub mod result;
    pub mod iterator;
    pub mod row;
    pub mod value;
    pub mod collection;
    pub mod ssl;
    pub mod schema;
    pub mod log;
    pub mod error;
    pub mod helpers;
    pub mod column;
    pub mod user_type;
    pub mod data_type;
    pub mod tuple;
    pub mod policy;
    pub mod custom_payload;
    pub mod time;
    pub mod util;
    pub mod metrics;
    pub mod write_type;
}


// #[phase(plugin)] extern crate bindgen;
// #[allow(dead_code, uppercase_variables, non_camel_case_types)]
// mod mysql_bindings {
//    bindgen!("/usr/include/mysql/mysql.h", match="mysql.h", link="mysql");
// }
