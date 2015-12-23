#![feature(plugin)]
#![plugin(cheddar)]

extern crate libc;
#[macro_use]
extern crate log;
extern crate decimal;
extern crate chrono;

pub use cql_ffi::consistency::*;
pub use cql_ffi::inet::*;
pub use cql_ffi::uuid::*;
pub use cql_ffi::cluster::*;
pub use cql_ffi::session::*;
pub use cql_ffi::statement::*;
pub use cql_ffi::batch::*;
pub use cql_ffi::future::*;
pub use cql_ffi::prepared::*;
pub use cql_ffi::result::*;
pub use cql_ffi::row::*;
pub use cql_ffi::value::*;
pub use cql_ffi::collection::*;
pub use cql_ffi::ssl::*;
pub use cql_ffi::schema::keyspace_meta::*;
pub use cql_ffi::schema::column_meta::*;
pub use cql_ffi::schema::schema_meta::*;
pub use cql_ffi::schema::table_meta::*;
pub use cql_ffi::schema::function_meta::*;
pub use cql_ffi::schema::aggregate_meta::*;
pub use cql_ffi::error::*;
pub use cql_ffi::helpers::*;
pub use cql_ffi::log::*;
pub use cql_ffi::column::*;
pub use cql_ffi::tuple::*;
pub use cql_ffi::user_type::*;
pub use cql_ffi::data_type::*;
pub use cql_ffi::policy::retry::*;
pub use cql_ffi::aggregate::*;
pub use cql_ffi::custom_payload::*;
pub use cql_ffi::time::*;
pub use cql_ffi::util::*;
pub use cql_ffi::metrics::*;
pub use cql_ffi::iterator::*;
pub use cql_ffi::write_type::*;
pub use cql_ffi::field::*;

extern crate cql_bindgen;


mod cql_ffi {
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
    pub mod aggregate;
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
