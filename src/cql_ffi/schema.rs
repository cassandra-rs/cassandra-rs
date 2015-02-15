#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;

use cql_ffi::string::CassString;
use cql_ffi::value::CassValue;
use cql_bindgen::CassSchema as _CassSchema;
use cql_bindgen::CassSchemaMeta as _CassSchemaMeta;
use cql_bindgen::CassSchemaMetaType as _CassSchemaMetaType;
use cql_bindgen::CassSchemaMetaField as _CassSchemaMetaField;
use cql_bindgen::cass_schema_meta_field_value;
use cql_bindgen::cass_schema_meta_field_name;
use cql_bindgen::cass_schema_meta_get_entry;
use cql_bindgen::cass_schema_meta_get_field;
use cql_bindgen::cass_schema_free;
use cql_bindgen::cass_schema_get_keyspace;
use cql_bindgen::cass_schema_meta_type;

pub struct CassSchema(pub *const _CassSchema);
pub struct CassSchemaMeta(pub *const _CassSchemaMeta);
pub struct CassSchemaMetaField(pub *const _CassSchemaMetaField);
pub struct CassSchemaMetaType(pub _CassSchemaMetaType);

//~ #[repr(C)]
//~ #[derive(Debug,Copy)]
//~ pub enum CassSchemaMetaType {
    //~ KEYSPACE = 0is,
    //~ TABLE = 1,
    //~ COLUMN = 2
//~ }

impl CassSchema {
    pub unsafe fn free(schema: CassSchema) {cass_schema_free(schema.0)}
    pub unsafe fn get_keyspace(schema: CassSchema, keyspace_name: *const c_char) -> CassSchemaMeta {CassSchemaMeta(cass_schema_get_keyspace(schema.0,keyspace_name))}
}

impl CassSchemaMeta {
    pub unsafe fn meta_type(meta: CassSchemaMeta) -> CassSchemaMetaType {CassSchemaMetaType(cass_schema_meta_type(meta.0))}
    pub unsafe fn meta_get_entry(meta: CassSchemaMeta, name: *const c_char) -> CassSchemaMeta {CassSchemaMeta(cass_schema_meta_get_entry(meta.0,name))}
    pub unsafe fn meta_get_field(meta: CassSchemaMeta, name: *const c_char) -> CassSchemaMetaField {CassSchemaMetaField(cass_schema_meta_get_field(meta.0,name))}
}

impl CassSchemaMetaField {
    pub unsafe fn meta_field_name(field: CassSchemaMetaField) -> CassString {CassString(cass_schema_meta_field_name(field.0))}
    pub unsafe fn meta_field_value(field: CassSchemaMetaField) -> CassValue {CassValue(cass_schema_meta_field_value(field.0))}
}

