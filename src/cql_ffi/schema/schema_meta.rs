use cql_bindgen::CassSchemaMeta as _CassSchemaMeta;
use cql_bindgen::cass_schema_meta_free;
use cql_bindgen::cass_schema_meta_keyspace_by_name;
use cql_bindgen::cass_schema_meta_snapshot_version;
use cql_bindgen::cass_iterator_keyspaces_from_schema_meta;

use cql_ffi::schema::keyspace_meta::KeyspaceMeta;
use cql_ffi::iterator::KeyspaceIterator;

pub struct SchemaMeta(pub *const _CassSchemaMeta);

impl Drop for SchemaMeta {
    fn drop(&mut self) {
        unsafe {
            cass_schema_meta_free(self.0);
        }
    }
}

impl SchemaMeta {
    ///Gets the version of the schema metadata snapshot.
    pub fn snapshot_version(&self) -> u32 { unsafe { cass_schema_meta_snapshot_version(self.0) } }

    ///Gets the keyspace metadata for the provided keyspace name.
    pub fn cass_schema_meta_keyspace_by_name(&self, keyspace: &str) -> KeyspaceMeta {
        unsafe { KeyspaceMeta(cass_schema_meta_keyspace_by_name(self.0, keyspace.as_ptr() as *const i8)) }
    }

    pub fn keyspace_iter(&mut self) -> KeyspaceIterator {
        unsafe { KeyspaceIterator(cass_iterator_keyspaces_from_schema_meta(self.0)) }
    }
}
