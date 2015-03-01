#![feature(libc,core,std_misc,alloc)]
extern crate libc;

pub use cql_ffi::consistency::*;
pub use cql_ffi::bytes::*;
pub use cql_ffi::types::*;
pub use cql_ffi::string::*;
pub use cql_ffi::inet::*;
pub use cql_ffi::decimal::*;
pub use cql_ffi::uuid::*;
pub use cql_ffi::cluster::*;
pub use cql_ffi::session::*;
pub use cql_ffi::statement::*;
pub use cql_ffi::batch::*;
pub use cql_ffi::future::*;
pub use cql_ffi::prepared::*;
pub use cql_ffi::result::*;
pub use cql_ffi::iterator::*;
pub use cql_ffi::row::*;
pub use cql_ffi::value::*;
pub use cql_ffi::collection::*;
pub use cql_ffi::ssl::*;
pub use cql_ffi::schema::*;
pub use cql_ffi::error::*;
pub use cql_ffi::helpers::*;
pub use cql_ffi::log::*;
pub use cql_ffi::column::*;
pub use cql_ffi::iterator::set_iterator::*;
pub use cql_ffi::iterator::map_iterator::*;

extern crate cql_bindgen;

mod cql_ffi {
    pub mod consistency;
    pub mod bytes;
    pub mod types;
    pub mod string;
    pub mod inet;
    pub mod decimal;
    pub mod uuid;
    pub mod cluster;
    pub mod session;
    pub mod statement;
    pub mod batch;
    pub mod future;
    pub mod prepared;
    pub mod result;
    pub mod iterator {
        pub mod map_iterator;
        pub mod set_iterator;
        pub mod result_iterator;
        pub mod row_iterator;
        pub mod cass_iterator;
    }
    pub mod row;
    pub mod value;
    pub mod collection;
    pub mod ssl;
    pub mod schema;
    pub mod log;
    pub mod error;
    pub mod helpers;
    pub mod column;
}
