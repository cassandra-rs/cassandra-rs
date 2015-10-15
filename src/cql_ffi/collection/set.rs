use std::ffi::CString;

use cql_bindgen::CassCollection as _CassCollection;
use cql_bindgen::cass_collection_new;
use cql_bindgen::cass_collection_free;
use cql_bindgen::cass_collection_append_int32;
use cql_bindgen::cass_collection_append_int64;
use cql_bindgen::cass_collection_append_float;
use cql_bindgen::cass_collection_append_double;
use cql_bindgen::cass_collection_append_bool;
use cql_bindgen::cass_collection_append_bytes;
use cql_bindgen::cass_collection_append_uuid;
use cql_bindgen::cass_collection_append_string;
use cql_bindgen::cass_collection_append_inet;
use cql_bindgen::CassIterator as _CassIterator;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_type;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_value;
//use cql_bindgen::cass_iterator_fields_from_schema_meta;
use cql_bindgen::cass_iterator_get_schema_meta;
use cql_bindgen::cass_iterator_get_schema_meta_field;

use cql_ffi::udt::UserType;
use cql_ffi::value::CassValue;
use cql_ffi::cass_iterator::CassIteratorType;
use cql_ffi::schema::SchemaMetaField;
use cql_ffi::schema::SchemaMeta;//use cql_bindgen::cass_collection_append_decimal;
use cql_ffi::collection::collection::CassCollectionType;
use cql_ffi::error::CassError;
use cql_ffi::uuid::Uuid;
use cql_ffi::inet::Inet;
use cql_bindgen::cass_collection_append_user_type;


pub struct Set(pub *mut _CassCollection);

impl Drop for Set {
    fn drop(&mut self) {
        self.free()
    }
}

impl Set {
    pub fn new(item_count: u64) -> Set {
        unsafe {
            Set(cass_collection_new(CassCollectionType::SET as u32, item_count))
        }
    }

    fn free(&mut self) {
        unsafe {
            cass_collection_free(self.0)
        }
    }

    pub fn append_int32(&mut self, value: i32) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_int32(self.0,value)).wrap(self)
        }
    }

    pub fn append_int64(&mut self, value: i64) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_int64(self.0,value)).wrap(self)
        }
    }

    pub fn append_float(&mut self, value: f32) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_float(self.0,value)).wrap(self)
        }
    }

    pub fn append_double(&mut self, value: f64) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_double(self.0,value)).wrap(self)
        }
    }

    pub fn append_bool(&mut self, value: bool) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_bool(self.0,if value {1} else {0})).wrap(self)
        }
    }

    pub fn append_string(&mut self, value: &str) -> Result<&Self, CassError> {
        unsafe {
            let cstr = CString::new(value).unwrap();
            let result = cass_collection_append_string(self.0, cstr.as_ptr());
            CassError::build(result).wrap(self)
        }
    }

    pub fn append_bytes(&mut self, value: Vec<u8>) -> Result<&Self, CassError> {
        unsafe {
            let bytes = cass_collection_append_bytes(self.0,
                                                     value[..].as_ptr(),
                                                     value.len() as u64);
            CassError::build(bytes).wrap(self)
        }
    }

    pub fn append_uuid(&mut self, value: Uuid) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_uuid(self.0,value.0)).wrap(self)
        }
    }

    pub fn append_inet(&mut self, value: Inet) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_inet(self.0,value.0)).wrap(self)
        }
    }

    pub fn append_user_type(&mut self, value: UserType) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_user_type(self.0,value.0)).wrap(self)
        }
    }

//    pub fn append_decimal(&mut self, value: String) -> Result<&Self,CassError> {unsafe{
//        CassError::build(cass_collection_append_decimal(self.0,value)).wrap(self)
//    }}
}


pub struct SetIterator(pub *mut _CassIterator);

//impl<'a> Display for &'a SetIterator {
//    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
//        for item in self {
//            try!(write!(f, "{}\t", item));
//        }
//        Ok(())
//    }
//}

impl Drop for SetIterator {
    fn drop(&mut self) {
        unsafe {
            cass_iterator_free(self.0)
        }
    }
}

impl Iterator for SetIterator {
    type Item = CassValue;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(self.get_value()),
            }
        }
    }
}

impl SetIterator {
    pub unsafe fn get_type(&mut self) -> CassIteratorType {
        CassIteratorType::new(cass_iterator_type(self.0))
    }

    //~ unsafe fn get_column(&mut self) -> Column {Column(cass_iterator_get_column(self.0))}

    pub fn get_value(&mut self) -> CassValue {
        unsafe {
            CassValue::new(cass_iterator_get_value(self.0))
        }
    }

    pub fn get_schema_meta(&mut self) -> SchemaMeta {
        unsafe {
            SchemaMeta(cass_iterator_get_schema_meta(self.0))
        }
    }

    pub fn get_schema_meta_field(&mut self) -> SchemaMetaField {
        unsafe {
            SchemaMetaField(cass_iterator_get_schema_meta_field(&mut *self.0))
        }
    }
}
