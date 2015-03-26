use cql_bindgen::CassIterator as _CassIterator;
use cql_ffi::value::CassValue;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_type;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_value;
use cql_bindgen::cass_iterator_fields_from_schema_meta;
use cql_bindgen::cass_iterator_get_schema_meta;
use cql_bindgen::cass_iterator_get_schema_meta_field;
use cql_ffi::iterator::cass_iterator::CassIteratorType;
use cql_ffi::schema::CassSchemaMetaField;
use cql_ffi::schema::CassSchemaMeta;

pub struct SetIterator(pub *mut _CassIterator);


impl Drop for SetIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
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


impl SetIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}
    pub unsafe fn get_type(&mut self) -> CassIteratorType {CassIteratorType(cass_iterator_type(self.0))}

    fn _next(&mut self) -> bool {unsafe{
        if cass_iterator_next(self.0) > 0 {true} else {false}
    }}
    
    //~ unsafe fn get_column(&mut self) -> CassColumn {CassColumn(cass_iterator_get_column(self.0))}
    pub unsafe fn get_value(&mut self)-> CassValue {CassValue(cass_iterator_get_value(self.0))}
    pub fn get_schema_meta(&mut self) -> CassSchemaMeta {unsafe{CassSchemaMeta(cass_iterator_get_schema_meta(self.0))}}
    pub fn get_schema_meta_field(&mut self) -> CassSchemaMetaField {unsafe{
        CassSchemaMetaField(cass_iterator_get_schema_meta_field(&mut *self.0))
    }}
}
