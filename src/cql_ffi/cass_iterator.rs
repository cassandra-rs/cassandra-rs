use cql_bindgen::CassIteratorType as _CassIteratorType;


pub struct CassIteratorType(_CassIteratorType);

impl CassIteratorType {
    pub fn new(_type: _CassIteratorType) -> Self {
        CassIteratorType(_type)
    }
}
