//! This is a wrapper around the DataStax C++ driver for Cassandra. It aims to be 100% safe with minimal overhead added
#![deny(missing_docs)]
#![allow(unknown_lints)]
#![allow(doc_markdown)]
#![allow(unused_imports)]  // TODO: remove
#![allow(dead_code)]  // TODO: remove
// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate slog;
extern crate decimal;
extern crate time;
extern crate uuid;
extern crate futures;
extern crate cassandra_cpp_sys;
use cassandra_cpp_sys as cassandra_sys;
#[macro_use]
extern crate error_chain;


pub use cassandra::batch::{Batch, BatchType, CustomPayload};
pub use cassandra::cluster::{Cluster, CqlProtocol};
pub use cassandra::collection::{CassCollection, List, Map, Set};
pub use cassandra::consistency::Consistency;
pub use cassandra::data_type::DataType;
// pub use cassandra::write_type::*;
pub use cassandra::field::Field;
pub use cassandra::future::CassFuture;
pub use cassandra::inet::Inet;
// pub use cassandra::util::*;
// pub use cassandra::metrics::*;
pub use cassandra::iterator::{AggregateIterator, ColumnIterator, FieldIterator, FunctionIterator, KeyspaceIterator,
                              MapIterator, SetIterator, TableIterator, UserTypeIterator};
pub use cassandra::log::{LogLevel, set_logger, set_level};
pub use cassandra::policy::retry::RetryPolicy;
pub use cassandra::prepared::PreparedStatement;
pub use cassandra::result::CassResult;
pub use cassandra::row::AsRustType;
pub use cassandra::row::Row;
pub use cassandra::schema::aggregate_meta::AggregateMeta;
pub use cassandra::schema::column_meta::ColumnMeta;
pub use cassandra::schema::function_meta::FunctionMeta;
pub use cassandra::schema::keyspace_meta::KeyspaceMeta;
pub use cassandra::schema::schema_meta::SchemaMeta;
pub use cassandra::schema::table_meta::TableMeta;
pub use cassandra::session::Session;
pub use cassandra::ssl::{Ssl, SslVerifyFlag};
pub use cassandra::statement::BindRustType;
pub use cassandra::statement::Statement;
// pub use cassandra::custom_payload::CustomPayload;
pub use cassandra::time::TimestampGen;
pub use cassandra::tuple::Tuple;
pub use cassandra::user_type::UserType;
pub use cassandra::uuid::{Uuid, UuidGen};
pub use cassandra::value::{Value, ValueType};

pub use cassandra::error::*;

// #[macro_use]
mod cassandra {
    #[macro_use]
    pub mod util;
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
    pub mod user_type;
    pub mod data_type;
    pub mod tuple;
    pub mod policy;
    pub mod custom_payload;
    pub mod time;
    pub mod metrics;
    pub mod write_type;
}


// #[phase(plugin)] extern crate bindgen;
// #[allow(dead_code, uppercase_variables, non_camel_case_types)]
// mod mysql_bindings {
//    bindgen!("/usr/include/mysql/mysql.h", match="mysql.h", link="mysql");
// }
