#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::value::CassValue;
use cql_ffi::row::CassRow;
use cql_ffi::error::CassError;
use cql_ffi::schema::CassSchemaMeta;
use cql_ffi::schema::CassSchemaMetaField;
use cql_bindgen::CassIterator as _CassIterator;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_type;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_row;
use cql_bindgen::cass_iterator_get_column;
use cql_bindgen::cass_iterator_get_value;
use cql_bindgen::cass_iterator_get_map_key;
use cql_bindgen::cass_iterator_get_schema_meta;
use cql_bindgen::cass_iterator_get_schema_meta_field;
use cql_bindgen::CassIteratorType as _CassIteratorType;

pub struct CassIterator(pub *mut _CassIterator);

pub struct CassIteratorType(_CassIteratorType);

pub struct ResultIterator(pub *mut _CassIterator);

pub struct MapIterator(pub *mut _CassIterator);

//~ #[repr(C)]
//~ #[derive(Debug,Copy)]
//~ pub enum CassIteratorType {
    //~ RESULT = 0,
    //~ ROW = 1,
    //~ COLLECTION = 2,
    //~ MAP = 3,
    //~ SCHEMA_META = 4,
    //~ SCHEMA_META_FIELD = 5
//~ }

impl MapIterator {
    pub unsafe fn get_key(&mut self) -> CassValue {CassValue(cass_iterator_get_map_key(self.0))}
    pub unsafe fn get_value(&mut self) -> CassValue {CassValue(cass_iterator_get_value(self.0))}
    pub unsafe fn get_pair(&mut self) -> Result<(CassValue,CassValue),CassError> {Ok((self.get_key(),self.get_value()))}
    pub unsafe fn next(&mut self) -> bool {if cass_iterator_next(self.0) > 0 {true} else {false}}
}

impl Drop for CassIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl Iterator for ResultIterator {
    type Item = CassRow;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {unsafe{
        match self.next() {
            true => Some(self.get_row()),
            false => None
        }}
    }
}

impl ResultIterator {
    pub unsafe fn get_row(&mut self) -> CassRow {CassRow(cass_iterator_get_row(self.0))}
    pub unsafe fn next(&mut self) -> bool {if cass_iterator_next(self.0) > 0 {true} else {false}}
}

impl CassIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}
    pub unsafe fn get_type(&mut self) -> CassIteratorType {CassIteratorType(cass_iterator_type(self.0))}
    pub unsafe fn next(&mut self) -> bool {if cass_iterator_next(self.0) > 0 {true} else {false}}
    pub unsafe fn get_column(&mut self) -> CassValue {CassValue(cass_iterator_get_column(self.0))}
    pub unsafe fn get_value(&mut self)-> CassValue {CassValue(cass_iterator_get_value(self.0))}
    pub unsafe fn get_schema_meta(&mut self) -> CassSchemaMeta {CassSchemaMeta(cass_iterator_get_schema_meta(self.0))}
    pub unsafe fn get_schema_meta_field(&mut self) -> CassSchemaMetaField {CassSchemaMetaField(cass_iterator_get_schema_meta_field(self.0))}
}
