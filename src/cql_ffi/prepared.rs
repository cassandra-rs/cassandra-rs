use cql_ffi::statement::CassStatement;

use cql_bindgen::CassPrepared as _CassPrepared;
use cql_bindgen::cass_prepared_free;
use cql_bindgen::cass_prepared_bind;
//use cql_bindgen::cass_prepared_parameter_name;
//use cql_bindgen::cass_prepared_parameter_data_type;
//use cql_bindgen::cass_prepared_parameter_data_type_by_name;
//use cql_bindgen::cass_prepared_parameter_data_type_by_name_n;

pub struct CassPrepared(pub *const _CassPrepared);

unsafe impl Sync for CassPrepared{}
unsafe impl Send for CassPrepared{}

impl Drop for CassPrepared {
    fn drop(&mut self) {
        unsafe {
            cass_prepared_free(self.0)
        }
    }
}

impl CassPrepared {
    pub fn bind(&self) -> CassStatement {
        unsafe {
            CassStatement(cass_prepared_bind(self.0))
        }
    }
}
