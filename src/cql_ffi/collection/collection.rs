// use cql_bindgen::cass_collection_new_from_data_type;
// use cql_bindgen::cass_collection_data_type;
// use cql_bindgen::cass_collection_append_collection;
// use cql_bindgen::cass_collection_append_tuple;
// use cql_bindgen::cass_collection_append_user_type;



#[repr(C)]

use std::convert::Into;

#[derive(Debug,Copy,Clone)]
pub enum CassCollectionType {
    LIST = 32,
    MAP = 33,
    SET = 34,
}

impl Into<i64> for CassCollectionType {
    fn into(self) -> i64 {
        self as i64
    }
}
