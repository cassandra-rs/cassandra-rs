use cql_bindgen::CassIterator as _CassIterator;
use cql_ffi::row::CassRow;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_row;

pub struct ResultIterator(pub *mut _CassIterator);

impl Drop for ResultIterator {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
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

impl ResultIterator {
    unsafe fn free(&mut self) {cass_iterator_free(self.0)}
    pub unsafe fn get_row(&mut self) -> CassRow {CassRow(cass_iterator_get_row(self.0))}
    pub unsafe fn next(&mut self) -> bool {if cass_iterator_next(self.0) > 0 {true} else {false}}
}
