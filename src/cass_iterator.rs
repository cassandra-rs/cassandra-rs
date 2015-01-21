#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_uint;

use cass_result::CassResult;
use cass_value::CassValue;
use cass_row::CassRow;
use cass_schema::CassSchemaMeta;
use cass_schema::CassSchema;
use cass_schema::CassSchemaMetaField;

use cass_types::cass_bool_t;

pub enum CassIterator { }

pub enum CassIteratorType {
    RESULT = 0,
    ROW = 1,
    COLLECTION = 2,
    MAP = 3,
    SCHEMA_META = 4,
    SCHEMA_META_FIELD = 5
}

extern "C" {
    pub fn cass_iterator_free(iterator: *mut CassIterator);
    pub fn cass_iterator_type(iterator: *mut CassIterator) -> CassIteratorType;
    pub fn cass_iterator_from_result(result: *const CassResult) -> *mut CassIterator;
    pub fn cass_iterator_from_row(row: *const CassRow) -> *mut CassIterator;
    pub fn cass_iterator_from_collection(value: *const CassValue) -> *mut CassIterator;
    pub fn cass_iterator_from_map(value: *const CassValue) -> *mut CassIterator;
    pub fn cass_iterator_from_schema(schema: *const CassSchema) -> *mut CassIterator;
    pub fn cass_iterator_from_schema_meta(meta: *const CassSchemaMeta) -> *mut CassIterator;
    pub fn cass_iterator_fields_from_schema_meta(meta: *const CassSchemaMeta) -> *mut CassIterator;
    pub fn cass_iterator_next(iterator: *mut CassIterator) -> cass_bool_t;
    pub fn cass_iterator_get_row(iterator: *mut CassIterator) -> *const CassRow;
    pub fn cass_iterator_get_column(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_value(iterator: *mut CassIterator)-> *const CassValue;
    pub fn cass_iterator_get_map_key(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_map_value(iterator: *mut CassIterator) -> *const CassValue;
    pub fn cass_iterator_get_schema_meta(iterator: *mut CassIterator) -> *const CassSchemaMeta;
    pub fn cass_iterator_get_schema_meta_field(iterator: *mut CassIterator) -> *const CassSchemaMetaField;
}
