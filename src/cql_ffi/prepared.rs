use cql_ffi::statement::CassStatement;

use cql_bindgen::CassPrepared as _PreparedStatement;
use cql_bindgen::cass_prepared_free;
use cql_bindgen::cass_prepared_bind;
//use cql_bindgen::cass_prepared_parameter_name;
//use cql_bindgen::cass_prepared_parameter_data_type;
//use cql_bindgen::cass_prepared_parameter_data_type_by_name;
//use cql_bindgen::cass_prepared_parameter_data_type_by_name_n;

/// A statement that has been prepared against at least one Cassandra node.
/// Instances of this class should not be created directly, but through Session.prepare().
pub struct PreparedStatement(pub *const _PreparedStatement);

unsafe impl Sync for PreparedStatement{}
unsafe impl Send for PreparedStatement{}

impl Drop for PreparedStatement {
    fn drop(&mut self) {
        unsafe {
            cass_prepared_free(self.0)
        }
    }
}

impl PreparedStatement {
    pub fn bind(&self) -> CassStatement {
        unsafe {
            CassStatement(cass_prepared_bind(self.0))
        }
    }
}
