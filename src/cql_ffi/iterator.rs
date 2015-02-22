#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::value::CassValue;
use cql_ffi::column::CassColumn;
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

//pub struct CassIterator(pub *mut _CassIterator);

pub struct CassIteratorType(_CassIteratorType);

pub struct ResultIterator(pub *mut _CassIterator);
pub struct RowIterator(pub *mut _CassIterator);

pub struct MapIterator(pub *mut _CassIterator);
pub struct SetIterator(pub *mut _CassIterator);
pub struct ListIterator(pub *mut _CassIterator);

impl Drop for ResultIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl Drop for RowIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl Drop for SetIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl Drop for ListIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}
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

//~ impl Debug for MapIterator{
    //~ fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        //~ while self._next() {
           //~ write!(f, "MAP {:?}:{:?}", "a","b")
        //~ }
        //~ Ok(())
    //~ }
//~ }

//~ impl Debug for SetIterator{
    //~ fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        //~ while self._next() {
          //~ // write!(f, "SET {:?}:{:?}", self.get_key(), self.get_value())
        //~ }
        //~ Ok(())
    //~ }
//~ }

impl MapIterator {
    fn get_key(&mut self) -> CassValue {unsafe{
        CassValue(cass_iterator_get_map_key(self.0))
    }}
    fn get_value(&mut self) -> CassValue {unsafe{
        CassValue(cass_iterator_get_value(self.0))
    }}
    
    pub fn get_pair(&mut self) -> Result<(CassValue,CassValue),CassError> {
        Ok((self.get_key(),self.get_value()))
    }
    
    fn _next(&mut self) -> bool {unsafe{
        if cass_iterator_next(self.0) > 0 {true} else {false}
    }}

    unsafe fn free(&mut self) {
        cass_iterator_free(self.0)
    }

}

impl Drop for MapIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl RowIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}

    pub fn _next(&mut self) -> bool {unsafe{
        if cass_iterator_next(self.0) > 0 {true} else {false}
    }}

    pub fn get_column(&mut self) -> CassColumn {unsafe{CassColumn(cass_iterator_get_column(self.0))}}


}

impl Iterator for RowIterator {
    type Item = CassColumn;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self._next() {
            true => Some(self.get_column()),
            false => None
        }
    }
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

impl Iterator for ListIterator {
    type Item = CassValue;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self._next() {
            true => Some(self.get_value()),
            false => None
        }
    }
}

impl Iterator for MapIterator {
    type Item = (CassValue,CassValue);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let k = match self._next() {
            true => self.get_value(),
            false => return None
        };
        let v = match self._next() {
            true => self.get_value(),
            false => return None
        };
        Some((k,v))
    }
}

impl Iterator for SetIterator {
    type Item = CassValue;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {unsafe{
        match self._next() {
            true => Some(self.get_value()),
            false => None
        }}
    }
}

impl ListIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}

    pub fn _next(&mut self) -> bool {unsafe{
        if cass_iterator_next(self.0) > 0 {true} else {false}
    }}
    
    pub fn get_value(&mut self)-> CassValue {unsafe{
        CassValue(cass_iterator_get_value(self.0))
    }}
}

impl ResultIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}
    pub unsafe fn get_row(&mut self) -> CassRow {CassRow(cass_iterator_get_row(self.0))}
    pub unsafe fn next(&mut self) -> bool {if cass_iterator_next(self.0) > 0 {true} else {false}}
}

impl SetIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}
    pub unsafe fn get_type(&mut self) -> CassIteratorType {CassIteratorType(cass_iterator_type(self.0))}

    fn _next(&mut self) -> bool {unsafe{
        if cass_iterator_next(self.0) > 0 {true} else {false}
    }}
    
    pub unsafe fn get_column(&mut self) -> CassValue {CassValue(cass_iterator_get_column(self.0))}
    pub unsafe fn get_value(&mut self)-> CassValue {CassValue(cass_iterator_get_value(self.0))}
    pub unsafe fn get_schema_meta(&mut self) -> CassSchemaMeta {CassSchemaMeta(cass_iterator_get_schema_meta(self.0))}
    pub unsafe fn get_schema_meta_field(&mut self) -> CassSchemaMetaField {CassSchemaMetaField(cass_iterator_get_schema_meta_field(self.0))}
}
