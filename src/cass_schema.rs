#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_uint;

use cass_string::CassString;
use cass_value::CassValue;

enum Struct_CassSchema_ { }
pub type CassSchema = Struct_CassSchema_;

enum Struct_CassSchemaMeta_ { }
pub type CassSchemaMeta = Struct_CassSchemaMeta_;

enum Struct_CassSchemaMetaField_ { }
pub type CassSchemaMetaField = Struct_CassSchemaMetaField_;

type Enum_CassSchemaMetaType_ = c_uint;
pub const CASS_SCHEMA_META_TYPE_KEYSPACE: c_uint = 0;
pub const CASS_SCHEMA_META_TYPE_TABLE: c_uint = 1;
pub const CASS_SCHEMA_META_TYPE_COLUMN: c_uint = 2;
pub type CassSchemaMetaType = Enum_CassSchemaMetaType_;

extern "C" {
    pub fn cass_schema_free(schema: *const CassSchema);
    pub fn cass_schema_get_keyspace(schema: *const CassSchema, keyspace_name: *const c_char) -> *const CassSchemaMeta;
    pub fn cass_schema_meta_type(meta: *const CassSchemaMeta) -> CassSchemaMetaType;
    pub fn cass_schema_meta_get_entry(meta: *const CassSchemaMeta, name: *const c_char) -> *const CassSchemaMeta;
    pub fn cass_schema_meta_get_field(meta: *const CassSchemaMeta, name: *const c_char) -> *const CassSchemaMetaField;
    pub fn cass_schema_meta_field_name(field: *const CassSchemaMetaField) -> CassString;
    pub fn cass_schema_meta_field_value(field: *const CassSchemaMetaField) -> *const CassValue;
}

