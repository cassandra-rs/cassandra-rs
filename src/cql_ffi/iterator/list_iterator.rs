use cql_bindgen::CassIterator as _CassIterator;
use cql_ffi::value::CassValue;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_value;

pub struct ListIterator(pub *mut _CassIterator);

impl Drop for ListIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
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

impl ListIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}

    pub fn _next(&mut self) -> bool {unsafe{
        if cass_iterator_next(self.0) > 0 {true} else {false}
    }}
    
    pub fn get_value(&mut self)-> CassValue {unsafe{
        CassValue(cass_iterator_get_value(self.0))
    }}
}

