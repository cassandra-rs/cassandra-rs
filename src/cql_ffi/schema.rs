#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use std::ffi::CString;
use std::mem;
use std::slice;
use std::str;

use cql_ffi::value::CassValue;
use cql_ffi::error::CassError;
use cql_ffi::collection::map::MapIterator;
use cql_ffi::collection::set::SetIterator;

//use cql_ffi::udt::CassDataType;
use cql_ffi::udt::CassConstDataType;
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
use cql_bindgen::cass_iterator_from_schema;
use cql_bindgen::cass_iterator_from_schema_meta;
use cql_bindgen::cass_iterator_fields_from_schema_meta;
use cql_bindgen::cass_schema_get_udt;
//use cql_bindgen::cass_schema_get_udt_n;



pub struct CassSchema(pub *const _CassSchema);
pub struct CassSchemaMeta(pub *const _CassSchemaMeta);
pub struct CassSchemaMetaField(pub *const _CassSchemaMetaField);
//pub struct CassSchemaMetaType(pub _CassSchemaMetaType);
pub struct CassSchemaMetaFieldIterator(pub *mut SetIterator);

//~ #[repr(C)]
#[derive(Debug,Copy,Clone)]
pub enum CassSchemaMetaType {
    KEYSPACE = 0isize,
    TABLE = 1,
    COLUMN = 2,
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
    pub fn build(val: isize) -> Result<Self, CassError> {
        match val {
            0 => Ok(CassSchemaMetaType::KEYSPACE),
            1 => Ok(CassSchemaMetaType::TABLE),
            2 => Ok(CassSchemaMetaType::COLUMN),
            _ => panic!("impossible schema meta type"),
        }
    }
}

impl CassSchema {
    pub fn get_keyspace(&self, keyspace_name: &str) -> CassSchemaMeta {
        unsafe {
            let keyspace_name = CString::new(keyspace_name).unwrap();
            CassSchemaMeta(cass_schema_get_keyspace(self.0, keyspace_name.as_ptr()))
        }
    }

    pub fn get_udt<S>(&self, keyspace: S, type_name: S) -> CassConstDataType
        where S: Into<String>
    {
        unsafe {
            let keyspace = CString::new(keyspace.into()).unwrap();
            let type_name = CString::new(type_name.into()).unwrap();
            CassConstDataType(cass_schema_get_udt(self.0, keyspace.as_ptr(), type_name.as_ptr()))
        }
    }



    pub unsafe fn iterator(&self) -> SetIterator {
        SetIterator(cass_iterator_from_schema(self.0))
    }
}

impl Drop for CassSchema {
    fn drop(&mut self) {
        unsafe {
            cass_schema_free(self.0)
        }
    }
}


impl CassSchemaMeta {
    pub fn get_type(&self) -> Result<CassSchemaMetaType, CassError> {
        unsafe {
            CassSchemaMetaType::build(cass_schema_meta_type(self.0) as isize)
        }
    }

    pub fn get_entry(&self, name: &str) -> CassSchemaMeta {
        unsafe {
            let name = CString::new(name).unwrap();
            CassSchemaMeta(cass_schema_meta_get_entry(self.0, name.as_ptr()))
        }
    }

    pub fn get_field(&self, name: &str) -> CassSchemaMetaField {
        unsafe {
            let name = CString::new(name).unwrap();
            CassSchemaMetaField(cass_schema_meta_get_field(self.0, name.as_ptr()))
        }
    }

    pub fn fields_from_schema_meta(&self) -> SetIterator {
        unsafe {
            SetIterator(cass_iterator_fields_from_schema_meta(self.0))
        }
    }

    //~ fn is_null(&self) -> bool {unsafe{
        //~ if cass_value_is_null(self.0) > 0 {true} else {false}
    //~ }}

    pub fn iterator(&self) -> SetIterator {
        unsafe {
            SetIterator(cass_iterator_from_schema_meta(self.0))
        }
    }

    pub fn fields_iterator(&self) -> MapIterator {
        unsafe {
            MapIterator(cass_iterator_fields_from_schema_meta(self.0))
        }
    }

}

impl CassSchemaMetaField {
    pub fn get_name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_schema_meta_field_name(self.0, &mut name, &mut name_length);

            let slice = slice::from_raw_parts(name as *const u8, name_length as usize);
            str::from_utf8(slice).unwrap().to_owned()
        }
    }

    pub fn get_value(&self) -> CassValue {
        unsafe {
            CassValue::new(cass_schema_meta_field_value(self.0))
        }
    }
}
