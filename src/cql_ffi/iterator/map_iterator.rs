use cql_bindgen::CassIterator as _CassIterator;
use cql_ffi::value::CassValue;
use cql_ffi::error::CassError;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_value;
use cql_bindgen::cass_iterator_get_map_key;

pub struct MapIterator(pub *mut _CassIterator);

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

impl Iterator for MapIterator {
    type Item = CassValue;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self._next() {
            true => Some(self.get_value()),
            false => None
        }
    }
}
