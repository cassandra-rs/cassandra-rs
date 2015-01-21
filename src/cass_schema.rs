#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_uint;

use cass_string::CassString;
use cass_value::CassValue;

pub enum CassSchema { }

pub enum CassSchemaMeta { }

pub enum CassSchemaMetaField { }

pub enum CassSchemaMetaType {
    KEYSPACE = 0is,
    TABLE = 1,
    COLUMN = 2
}

extern "C" {
    pub fn cass_schema_free(schema: *const CassSchema);
    pub fn cass_schema_get_keyspace(schema: *const CassSchema, keyspace_name: *const c_char) -> *const CassSchemaMeta;
    pub fn cass_schema_meta_type(meta: *const CassSchemaMeta) -> CassSchemaMetaType;
    pub fn cass_schema_meta_get_entry(meta: *const CassSchemaMeta, name: *const c_char) -> *const CassSchemaMeta;
    pub fn cass_schema_meta_get_field(meta: *const CassSchemaMeta, name: *const c_char) -> *const CassSchemaMetaField;
    pub fn cass_schema_meta_field_name(field: *const CassSchemaMetaField) -> CassString;
    pub fn cass_schema_meta_field_value(field: *const CassSchemaMetaField) -> *const CassValue;
}

