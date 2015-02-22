#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::string::CassString;
use cql_ffi::value::CassValue;
use cql_ffi::error::CassError;
use cql_ffi::helpers::str_to_ref;
use cql_ffi::iterator::SetIterator;
use cql_bindgen::CassSchema as _CassSchema;
use cql_bindgen::CassSchemaMeta as _CassSchemaMeta;
use cql_bindgen::CassSchemaMetaField as _CassSchemaMetaField;
use cql_bindgen::cass_schema_meta_field_value;
use cql_bindgen::cass_schema_meta_field_name;
use cql_bindgen::cass_schema_meta_get_entry;
use cql_bindgen::cass_schema_meta_get_field;
use cql_bindgen::cass_schema_free;
use cql_bindgen::cass_schema_get_keyspace;
use cql_bindgen::cass_schema_meta_type;
//use cql_bindgen::cass_value_is_null;
use cql_bindgen::cass_iterator_from_schema;
use cql_bindgen::cass_iterator_from_schema_meta;
use cql_bindgen::cass_iterator_fields_from_schema_meta;

pub struct CassSchema(pub *const _CassSchema);
pub struct CassSchemaMeta(pub *const _CassSchemaMeta);
pub struct CassSchemaMetaField(pub *const _CassSchemaMetaField);
//pub struct CassSchemaMetaType(pub _CassSchemaMetaType);
pub struct CassSchemaMetaFieldIterator(pub *mut SetIterator);

//~ #[repr(C)]
#[derive(Debug,Copy)]
pub enum CassSchemaMetaType {
    KEYSPACE = 0isize,
    TABLE = 1,
    COLUMN = 2
}

//~ impl Iterator for CassSchemaMetaFieldIterator {
    //~ type Item = CassSchemaMeta;
    //~ fn next(&mut self) -> Option<<Self as Iterator>::Item> {unsafe{
        //~ match self.0.next() {
            //~ Some(field) => Some(CassSchemaMeta(self.0.get_schema_meta_field())),
            //~ None => None
        //~ }}
    //~ }
//~ }

impl CassSchemaMetaType {
    pub fn build(val:isize) -> Result<Self,CassError> {
        match val {
            0 => Ok(CassSchemaMetaType::KEYSPACE),
            1 => Ok(CassSchemaMetaType::TABLE),
            2 => Ok(CassSchemaMetaType::COLUMN),
            _ => panic!("impossible schema meta type")
        }
    }
}

impl CassSchema {
    unsafe fn free(&mut self) {cass_schema_free(self.0)}
    pub unsafe fn get_keyspace(&self, keyspace_name: &str) -> CassSchemaMeta {
        CassSchemaMeta(cass_schema_get_keyspace(self.0,str_to_ref(keyspace_name)))
    }
    
    pub unsafe fn iterator(&self) -> SetIterator {SetIterator(cass_iterator_from_schema(self.0))}
}

impl Drop for CassSchema {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}


impl CassSchemaMeta {
    pub unsafe fn get_type(&self) -> Result<CassSchemaMetaType,CassError> {CassSchemaMetaType::build(cass_schema_meta_type(self.0) as isize)}
    pub unsafe fn get_entry(&self, name: &str) -> CassSchemaMeta {CassSchemaMeta(cass_schema_meta_get_entry(self.0,str_to_ref(name)))}
    pub unsafe fn get_field(&self, name: &str) -> CassSchemaMetaField {CassSchemaMetaField(cass_schema_meta_get_field(self.0,str_to_ref(name)))}
//    pub unsafe fn is_null(&self) -> bool {if cass_value_is_null(self.0) > 0 {true} else {false}}
    pub unsafe fn iterator(&self) -> SetIterator {SetIterator(cass_iterator_from_schema_meta(self.0))}
    pub unsafe fn fields_iterator(&self) -> SetIterator {SetIterator(cass_iterator_fields_from_schema_meta(self.0))}

}

impl CassSchemaMetaField {
    pub unsafe fn get_name(&self) -> CassString {CassString(cass_schema_meta_field_name(self.0))}
    pub unsafe fn get_value(&self) -> CassValue {CassValue(cass_schema_meta_field_value(self.0))}
}

