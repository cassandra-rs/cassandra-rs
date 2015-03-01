use cql_bindgen::CassIterator as _CassIterator;
use cql_ffi::column::CassColumn;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_column;

pub struct RowIterator(pub *mut _CassIterator);


impl Drop for RowIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl RowIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}

    fn _next(&mut self) -> bool {unsafe{
        if cass_iterator_next(self.0) > 0 {true} else {false}
    }}

    fn get_column(&mut self) -> CassColumn {unsafe{CassColumn(cass_iterator_get_column(self.0))}}


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
