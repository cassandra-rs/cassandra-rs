#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

//pub use iterator::*;
//mod iterator {
    pub mod result_iterator;
    pub mod row_iterator;
    pub mod list_iterator;
    pub mod map_iterator;
    pub mod set_iterator;
    pub mod cass_iterator;
//}

//use result_iterator::*;

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






//~ impl Iterator for MapIterator {
    //~ type Item = (CassValue,());
    //~ fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        //~ let k = match self._next() {
            //~ true => self.get_value(),
            //~ false => return None
        //~ };
        //~ let v = match self._next() {
            //~ true => self.get_value(),
            //~ false => return None
        //~ };
        //~ Some((k,()))
    //~ }
//~ }


