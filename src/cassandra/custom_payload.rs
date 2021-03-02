use crate::cassandra::error::*;
use crate::cassandra::util::Protected;

use crate::cassandra_sys::cass_custom_payload_free;
use crate::cassandra_sys::cass_custom_payload_new;
use crate::cassandra_sys::cass_custom_payload_set;
use crate::cassandra_sys::cass_custom_payload_set_n;
use crate::cassandra_sys::CassCustomPayload as _CassCustomPayload;
use std::collections::HashMap;
use std::os::raw::c_char;

pub type CustomPayloadResponse = HashMap<String, Vec<u8>>;

/// Custom payloads not fully supported yet
#[derive(Debug)]
pub struct CustomPayload(*mut _CassCustomPayload);

// The underlying C type has no thread-local state, but does not support access
// from multiple threads: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for CustomPayload {}

impl Protected<*mut _CassCustomPayload> for CustomPayload {
    fn inner(&self) -> *mut _CassCustomPayload {
        self.0
    }
    fn build(inner: *mut _CassCustomPayload) -> Self {
        if inner.is_null() {
            panic!("Unexpected null pointer")
        };
        CustomPayload(inner)
    }
}

impl Default for CustomPayload {
    /// creates a new custom payload
    fn default() -> Self {
        unsafe { CustomPayload(cass_custom_payload_new()) }
    }
}
impl CustomPayload {
    /// Sets an item to the custom payload.
    pub fn set(&self, name: String, value: &[u8]) -> Result<()> {
        unsafe {
            let name_ptr = name.as_ptr() as *const c_char;
            Ok(cass_custom_payload_set_n(
                self.0,
                name_ptr,
                name.len(),
                value.as_ptr(),
                value.len(),
            ))
        }
    }
}

impl Drop for CustomPayload {
    fn drop(&mut self) {
        unsafe { cass_custom_payload_free(self.0) }
    }
}
