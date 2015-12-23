use std::{mem, slice, str};

use cql_bindgen::CASS_OK;
// use cql_bindgen::CassIteratorType as _CassIteratorType;
use cql_bindgen::CassIterator as _CassIterator;
// use cql_bindgen::cass_iterator_type;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_get_function_meta;
use cql_bindgen::cass_iterator_get_keyspace_meta;
use cql_bindgen::cass_iterator_get_map_key;
use cql_bindgen::cass_iterator_get_map_value;
use cql_bindgen::cass_iterator_get_meta_field_name;
use cql_bindgen::cass_iterator_get_meta_field_value;
use cql_bindgen::cass_iterator_get_table_meta;
use cql_bindgen::cass_iterator_get_column_meta;
use cql_bindgen::cass_iterator_get_user_type;
use cql_bindgen::cass_iterator_get_value;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_user_type_field_name;
use cql_bindgen::cass_iterator_get_user_type_field_value;
use cql_ffi::error::CassError;
use cql_ffi::value::Value;
use cql_ffi::schema::keyspace_meta::KeyspaceMeta;
use cql_ffi::schema::table_meta::TableMeta;
use cql_ffi::schema::function_meta::FunctionMeta;
use cql_ffi::schema::column_meta::ColumnMeta;
use cql_ffi::schema::aggregate_meta::AggregateMeta;

use cql_bindgen::cass_iterator_get_aggregate_meta;

pub struct AggregateIterator(pub *mut _CassIterator);

impl Drop for AggregateIterator {
    fn drop(&mut self) { unsafe { cass_iterator_free(self.0) } }
}

impl Iterator for AggregateIterator {
    type Item = AggregateMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                // cass_iterator_get_user_type_field_name(fields, &field_name, &field_name_length);
                _ => {
                    //
                    let field_name = mem::zeroed();
                    let field_name_length = mem::zeroed();
                    cass_iterator_get_user_type_field_name(self.0, field_name, field_name_length);
                    let slice = slice::from_raw_parts(field_name as *const u8, field_name_length as usize);
                    let key = str::from_utf8(slice).unwrap().to_owned();

                    let field_value = cass_iterator_get_aggregate_meta(self.0);
                    Some(AggregateMeta(field_value))
                }
            }
        }
    }
}

pub struct UserTypeIterator(pub *mut _CassIterator);

impl Drop for UserTypeIterator {
    fn drop(&mut self) { unsafe { cass_iterator_free(self.0) } }
}

impl Iterator for UserTypeIterator {
    type Item = (String,Value);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                // cass_iterator_get_user_type_field_name(fields, &field_name, &field_name_length);
                _ => {
                    //
                    let field_name = mem::zeroed();
                    let field_name_length = mem::zeroed();
                    cass_iterator_get_user_type_field_name(self.0, field_name, field_name_length);
                    let slice = slice::from_raw_parts(field_name as *const u8, field_name_length as usize);
                    let key = str::from_utf8(slice).unwrap().to_owned();

                    let field_value = cass_iterator_get_user_type_field_value(self.0);
                    Some((key, Value::new(field_value)))
                }
            }
        }
    }
}

impl UserTypeIterator {
    //    pub fn get_field_name(&mut self)-> Value {unsafe{
    //
    //        Value::new(cass_iterator_get_user_type_field_name(self.0))
    //    }}
}


pub struct FunctionIterator(pub *mut _CassIterator);

impl Iterator for FunctionIterator {
    type Item = FunctionMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(FunctionMeta(cass_iterator_get_function_meta(self.0))),
            }
        }
    }
}

pub struct TableIterator(pub *mut _CassIterator);

impl Iterator for TableIterator {
    type Item = TableMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(TableMeta(cass_iterator_get_table_meta(self.0))),
            }
        }
    }
}

pub struct KeyspaceIterator(pub *mut _CassIterator);

impl Iterator for KeyspaceIterator {
    type Item = KeyspaceMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(KeyspaceMeta(cass_iterator_get_keyspace_meta(self.0))),
            }
        }
    }
}

pub struct ColumnIterator(pub *mut _CassIterator);

impl Iterator for ColumnIterator {
    type Item = ColumnMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(ColumnMeta(cass_iterator_get_column_meta(self.0))),
            }
        }
    }
}

pub struct FieldIterator(pub *mut _CassIterator);

impl Iterator for FieldIterator {
    type Item = (String, Value);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => {
                    let mut name = mem::zeroed();
                    let mut name_length = mem::zeroed();
                    match cass_iterator_get_meta_field_name(self.0, &mut name, &mut name_length) {
                        CASS_OK => {
                            let slice = slice::from_raw_parts(name as *const u8, name_length as usize);
                            let name = str::from_utf8(slice).unwrap();
                            let value = cass_iterator_get_meta_field_value(self.0);
                            Some((name.to_owned(), Value(value)))
                        }
                        err => panic!("FIXME: no error handling. Err {}", err),
                    }
                }
            }
        }
    }
}

// pub struct CassIteratorType(_CassIteratorType);

// impl CassIteratorType {
//    pub fn new(_type: _CassIteratorType) -> Self { CassIteratorType(_type) }
// }

pub trait CassIterator {

    fn inner(&self) -> *mut _CassIterator;

    fn get_value(&mut self) -> Value { unsafe { Value::new(cass_iterator_get_value(self.inner())) } }

}

// impl Iterator for CassIterator {
//    type Item = Value;
//
//    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//        unsafe {
//            match cass_iterator_next(self.inner()) {
//                0 => None,
//                _ => Some(self.get_value()),
//            }
//        }
//    }
// }

pub struct SetIterator(pub *mut _CassIterator);

// impl<'a> Display for &'a SetIterator {
//    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
//        for item in self {
//            try!(write!(f, "{}\t", item));
//        }
//        Ok(())
//    }
// }

impl Drop for SetIterator {
    fn drop(&mut self) { unsafe { cass_iterator_free(self.0) } }
}

impl CassIterator for SetIterator {
    fn inner(&self) -> *mut _CassIterator { self.0 }
}

impl Iterator for SetIterator {
    type Item = Value;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.inner()) {
                0 => None,
                _ => Some(self.get_value()),
            }
        }
    }
}

impl SetIterator {
    //    pub fn get_type(&mut self) -> CassIteratorType {
    //        unsafe { CassIteratorType::new(cass_iterator_type(self.0)) }
    //    }

    // ~ unsafe fn get_column(&mut self) -> Column
    // {Column(cass_iterator_get_column(self.0))}

    pub fn get_value(&mut self) -> Value { unsafe { Value::new(cass_iterator_get_value(self.0)) } }

    //    pub fn get_schema_meta(&mut self) -> SchemaMeta {
    //        unsafe { SchemaMeta(cass_iterator_get_schema_meta(self.0)) }
    //    }

    //    pub fn get_schema_meta_field(&mut self) -> SchemaMetaField {
    //        unsafe { SchemaMetaField(cass_iterator_get_schema_meta_field(&mut *self.0)) }
    //    }
}

pub struct MapIterator(pub *mut _CassIterator);

impl MapIterator {
    pub fn get_key(&mut self) -> Value { unsafe { Value::new(cass_iterator_get_map_key(self.0)) } }
    pub fn get_value(&mut self) -> Value { unsafe { Value::new(cass_iterator_get_map_value(self.0)) } }

    pub fn get_pair(&mut self) -> Result<(Value, Value), CassError> { Ok((self.get_key(), self.get_value())) }
}


impl Drop for MapIterator {
    fn drop(&mut self) { unsafe { cass_iterator_free(self.0) } }
}

impl Iterator for MapIterator {
    type Item = (Value,Value);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(self.get_pair().unwrap()),
            }
        }
    }
}

pub struct ListIterator(pub *mut _CassIterator);

impl Drop for ListIterator {
    fn drop(&mut self) { unsafe { cass_iterator_free(self.0) } }
}

impl Iterator for ListIterator {
    type Item = Value;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(self.get_value()),
            }
        }
    }
}

impl ListIterator {
    pub fn get_value(&mut self) -> Value { unsafe { Value::new(cass_iterator_get_value(self.0)) } }
}
