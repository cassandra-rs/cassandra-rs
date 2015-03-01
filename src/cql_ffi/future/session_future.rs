use cql_ffi::error::CassError;
use cql_ffi::session::CassSession;
use cql_bindgen::CassFuture as _CassFuture;
use cql_bindgen::cass_future_wait;
use cql_bindgen::cass_future_error_code;

pub struct SessionFuture(pub *mut _CassFuture, pub CassSession);

impl SessionFuture {
    pub fn wait(self) -> Result<CassSession,CassError> {unsafe{cass_future_wait(self.0);self.error_code()}}
    unsafe fn error_code(self) -> Result<CassSession,CassError> {CassError::build(cass_future_error_code(self.0)).wrap(self.1)}
}
