// #![feature(plugin)]
// #![plugin(cheddar)]

extern crate libc;
#[macro_use]
extern crate log;
extern crate decimal;
extern crate chrono;

pub use cassandra::consistency::*;
pub use cassandra::inet::*;
pub use cassandra::uuid::*;
pub use cassandra::cluster::*;
pub use cassandra::session::*;
pub use cassandra::statement::*;
pub use cassandra::batch::*;
pub use cassandra::future::*;
pub use cassandra::prepared::*;
pub use cassandra::result::*;
pub use cassandra::row::*;
pub use cassandra::value::*;
pub use cassandra::collection::*;
pub use cassandra::ssl::*;
pub use cassandra::schema::keyspace_meta::*;
pub use cassandra::schema::column_meta::*;
pub use cassandra::schema::schema_meta::*;
pub use cassandra::schema::table_meta::*;
pub use cassandra::schema::function_meta::*;
pub use cassandra::schema::aggregate_meta::*;
pub use cassandra::error::*;
pub use cassandra::helpers::*;
pub use cassandra::log::*;
pub use cassandra::column::*;
pub use cassandra::tuple::*;
pub use cassandra::user_type::*;
pub use cassandra::data_type::*;
pub use cassandra::policy::retry::*;
pub use cassandra::custom_payload::*;
pub use cassandra::time::*;
pub use cassandra::util::*;
pub use cassandra::metrics::*;
pub use cassandra::iterator::*;
pub use cassandra::write_type::*;
pub use cassandra::field::*;

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
