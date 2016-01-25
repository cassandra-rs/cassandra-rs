use cassandra_sys::CassSchemaMeta as _CassSchemaMeta;
use cassandra_sys::cass_schema_meta_free;
use cassandra_sys::cass_schema_meta_keyspace_by_name;
use cassandra_sys::cass_schema_meta_snapshot_version;
use cassandra_sys::cass_iterator_keyspaces_from_schema_meta;

use cassandra::schema::keyspace_meta::KeyspaceMeta;
use cassandra::iterator::KeyspaceIterator;
use cassandra::util::Protected;

///A snapshot of the schema's metadata
pub struct SchemaMeta(*const _CassSchemaMeta);

impl Drop for SchemaMeta {
    fn drop(&mut self) {
        unsafe {
            cass_schema_meta_free(self.0);
        }
    }
}

impl Protected<*const _CassSchemaMeta> for SchemaMeta {
    fn inner(&self) -> *const _CassSchemaMeta {
        self.0
    }
    fn build(inner: *const _CassSchemaMeta) -> Self {
        SchemaMeta(inner)
    }
}

impl SchemaMeta {
    ///Gets the version of the schema metadata snapshot.
    pub fn snapshot_version(&self) -> u32 {
        unsafe { cass_schema_meta_snapshot_version(self.0) }
    }

    ///Gets the keyspace metadata for the provided keyspace name.
    pub fn get_keyspace_by_name(&self, keyspace: &str) -> KeyspaceMeta {
        unsafe {
            KeyspaceMeta::build(cass_schema_meta_keyspace_by_name(self.0, keyspace.as_ptr() as *const i8))
        }
    }

	///Returns an iterator over the keyspaces in this schema
    pub fn keyspace_iter(&mut self) -> KeyspaceIterator {
        unsafe { KeyspaceIterator::build(cass_iterator_keyspaces_from_schema_meta(self.0)) }
    }
}
