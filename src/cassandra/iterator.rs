use std::{mem, slice, str};

use cassandra_sys::CASS_OK;
// use cassandra_sys::CassIteratorType as _CassIteratorType;
use cassandra_sys::CassIterator as _CassIterator;
// use cassandra_sys::cass_iterator_type;
use cassandra_sys::cass_iterator_free;
use cassandra_sys::cass_iterator_get_function_meta;
use cassandra_sys::cass_iterator_get_keyspace_meta;
use cassandra_sys::cass_iterator_get_map_key;
use cassandra_sys::cass_iterator_get_map_value;
use cassandra_sys::cass_iterator_get_meta_field_name;
use cassandra_sys::cass_iterator_get_meta_field_value;
use cassandra_sys::cass_iterator_get_table_meta;
use cassandra_sys::cass_iterator_get_column_meta;
use cassandra_sys::cass_iterator_get_user_type;
use cassandra_sys::cass_iterator_get_value;
use cassandra_sys::cass_iterator_next;
// use cassandra_sys::cass_iterator_get_user_type_field_name;
// use cassandra_sys::cass_iterator_get_user_type_field_value;
use cassandra::error::CassError;
use cassandra::schema::keyspace_meta;
use cassandra::value::Value;
use cassandra::field::Field;
use cassandra::data_type::ConstDataType;
use cassandra::schema::keyspace_meta::KeyspaceMeta;
use cassandra::schema::table_meta::TableMeta;
use cassandra::schema::function_meta::FunctionMeta;
use cassandra::schema::column_meta::ColumnMeta;
use cassandra::schema::aggregate_meta::AggregateMeta;
use cassandra::schema::{column_meta, table_meta};
use cassandra_sys::cass_iterator_get_aggregate_meta;
use cassandra::schema::function_meta;
use cassandra::schema::aggregate_meta;
use cassandra::util::Protected;

///Iterates over the  aggregate metadata entries(??)
pub struct AggregateIterator(*mut _CassIterator);

impl Drop for AggregateIterator {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl Iterator for AggregateIterator {
    type Item = AggregateMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => {
                    let field_value = cass_iterator_get_aggregate_meta(self.0);
                    Some(AggregateMeta::build(field_value))
                }
            }
        }
    }
}

///Iterater over the fields of a UDT
pub struct UserTypeIterator(*mut _CassIterator);

impl Drop for UserTypeIterator {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl Iterator for UserTypeIterator {
    type Item = ConstDataType;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(ConstDataType(cass_iterator_get_user_type(self.0))),
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


///Iterater over the  function metadata entries(??)
pub struct FunctionIterator(*mut _CassIterator);

impl Iterator for FunctionIterator {
    type Item = FunctionMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(FunctionMeta::build(cass_iterator_get_function_meta(self.0))),
            }
        }
    }
}


///Iterater over the table's metadata entries(??)
pub struct TableIterator(*mut _CassIterator);

impl Iterator for TableIterator {
    type Item = TableMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(TableMeta::build(cass_iterator_get_table_meta(self.0))),
            }
        }
    }
}

///Iterater over the keyspace's metadata entries(??)
pub struct KeyspaceIterator(*mut _CassIterator);

impl Iterator for KeyspaceIterator {
    type Item = KeyspaceMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(KeyspaceMeta::build(cass_iterator_get_keyspace_meta(self.0))),
            }
        }
    }
}

///Iterater over the columns's metadata entries(??)
pub struct ColumnIterator(*mut _CassIterator);

impl Iterator for ColumnIterator {
    type Item = ColumnMeta;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(ColumnMeta::build(cass_iterator_get_column_meta(self.0))),
            }
        }
    }
}

///Iterater over the field's metadata entries(??)
pub struct FieldIterator(*mut _CassIterator);

impl Iterator for FieldIterator {
    type Item = Field;
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
                            Some(Field {
                                name: name.to_owned(),
                                value: Value::build(value),
                            })
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

//impl Protected<*mut _Batch> for CassIterator {
//    fn inner(&self) -> *mut _CassIterator {
//        self.0
//    }
//    fn build(inner: *mut _CassIterator) -> Self {
//        CassIterator(inner)
//    }
//}

impl Protected<*mut _CassIterator> for UserTypeIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        UserTypeIterator(inner)
    }
}

impl Protected<*mut _CassIterator> for AggregateIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        AggregateIterator(inner)
    }
}

impl Protected<*mut _CassIterator> for FunctionIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        FunctionIterator(inner)
    }
}

impl Protected<*mut _CassIterator> for KeyspaceIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        KeyspaceIterator(inner)
    }
}

impl Protected<*mut _CassIterator> for FieldIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        FieldIterator(inner)
    }
}

impl Protected<*mut _CassIterator> for ColumnIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        ColumnIterator(inner)
    }
}

impl Protected<*mut _CassIterator> for TableIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        TableIterator(inner)
    }
}

impl Protected<*mut _CassIterator> for MapIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        MapIterator(inner)
    }
}

impl Protected<*mut _CassIterator> for SetIterator {
    fn inner(&self) -> *mut _CassIterator {
        self.0
    }
    fn build(inner: *mut _CassIterator) -> Self {
        SetIterator(inner)
    }
}


///Iterater over the set's metadata entries(??)
pub struct SetIterator(*mut _CassIterator);

// impl<'a> Display for &'a SetIterator {
//    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
//        for item in self {
//            try!(write!(f, "{}\t", item));
//        }
//        Ok(())
//    }
// }

impl Drop for SetIterator {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}


impl Iterator for SetIterator {
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

use cassandra::value;
impl SetIterator {
    fn get_value(&mut self) -> Value {
        unsafe { Value::build(cass_iterator_get_value(self.0)) }
    }
}

///An iterator over the k/v pair in the map
pub struct MapIterator(*mut _CassIterator);

impl MapIterator {
    fn get_key(&mut self) -> Value {
        unsafe { Value::build(cass_iterator_get_map_key(self.0)) }
    }
    fn get_value(&mut self) -> Value {
        unsafe { Value::build(cass_iterator_get_map_value(self.0)) }
    }

    ///Gets the next k/v pair in the map
    pub fn get_pair(&mut self) -> Result<(Value, Value), CassError> {
        Ok((self.get_key(), self.get_value()))
    }
}

///An iterator over the elements of a Cassandra tuple
pub struct TupleIterator(pub *mut _CassIterator);

impl Drop for TupleIterator {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl Iterator for TupleIterator {
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

impl TupleIterator {
    fn get_value(&mut self) -> Value {
        unsafe { Value::build(cass_iterator_get_value(self.0)) }
    }
}



impl Drop for MapIterator {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
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
