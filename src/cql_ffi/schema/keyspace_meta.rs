use cql_ffi::aggregate::CassAggregateMeta;
use cql_bindgen::{raw2utf8, str2ref};
use cql_bindgen::CassValue as _CassValue;

use cql_bindgen::cass_iterator_tables_from_keyspace_meta;
use cql_bindgen::cass_keyspace_meta_aggregate_by_name;
use cql_bindgen::cass_keyspace_meta_field_by_name;
use cql_bindgen::cass_keyspace_meta_function_by_name;
use cql_bindgen::cass_keyspace_meta_name;
use cql_bindgen::cass_keyspace_meta_table_by_name;
use cql_bindgen::cass_keyspace_meta_user_type_by_name;
use cql_bindgen::cass_iterator_aggregates_from_keyspace_meta;
use cql_bindgen::cass_iterator_fields_from_keyspace_meta;
use cql_bindgen::cass_iterator_functions_from_keyspace_meta;
use cql_bindgen::cass_iterator_user_types_from_keyspace_meta;

use cql_ffi::schema::function_meta::FunctionMeta;
use cql_ffi::schema::table_meta::TableMeta;
use cql_ffi::data_type::ConstDataType;
use cql_ffi::iterator::TableIterator;
use cql_ffi::iterator::KeyspaceIterator;
use cql_ffi::iterator::FieldIterator;
use cql_ffi::iterator::FunctionIterator;
use cql_ffi::iterator::UserTypeIterator;
use std::mem;

use cql_bindgen::CassKeyspaceMeta as _CassKeyspaceMeta;

pub struct KeyspaceMeta(pub *const _CassKeyspaceMeta);

pub struct MetadataFieldValue(*const _CassValue);

impl KeyspaceMeta {
    pub fn aggregrates_iterator(&self) -> FieldIterator {
        unsafe { FieldIterator(cass_iterator_aggregates_from_keyspace_meta(self.0)) }
    }

    pub fn fields_iter(&self) -> KeyspaceIterator {
        unsafe { KeyspaceIterator(cass_iterator_fields_from_keyspace_meta(self.0)) }
    }

    ///Gets the table metadata for the provided table name.
    pub fn table_by_name(&self, name: &str) -> Option<TableMeta> {
        unsafe {
            let value = cass_keyspace_meta_table_by_name(self.0, str2ref(name));
            if value.is_null() { None } else { Some(TableMeta(value)) }
        }
    }

    ///Gets the data type for the provided type name.
    pub fn user_type_by_name(&self, name: &str) -> Option<ConstDataType> {
        unsafe {
            let value = cass_keyspace_meta_user_type_by_name(self.0, str2ref(name));
            if value.is_null() { None } else { Some(ConstDataType(value)) }
        }
    }

    ///Gets the function metadata for the provided function name.
    pub fn function_by_name(&self, name: &str, arguments: Vec<&str>) -> Option<FunctionMeta> {
        unsafe {
            let value = cass_keyspace_meta_function_by_name(self.0, str2ref(name), str2ref(&arguments.join(",")));
            if value.is_null() { None } else { Some(FunctionMeta(value)) }
        }
    }

    ///Gets the aggregate metadata for the provided aggregate name.
    pub fn aggregate_by_name(&self, name: &str, arguments: Vec<&str>) -> Option<CassAggregateMeta> {
        unsafe {
            let agg = cass_keyspace_meta_aggregate_by_name(self.0, str2ref(name), str2ref(&arguments.join(",")));
            if agg.is_null() { None } else { Some(CassAggregateMeta(agg)) }
        }
    }

    pub fn table_iter(&mut self) -> TableIterator {
        unsafe { TableIterator(cass_iterator_tables_from_keyspace_meta(self.0)) }
    }

    pub fn function_iter(&mut self) -> FunctionIterator {
        unsafe { FunctionIterator(cass_iterator_functions_from_keyspace_meta(self.0)) }
    }

    pub fn user_type_iter(&mut self) -> UserTypeIterator {
        unsafe { UserTypeIterator(cass_iterator_user_types_from_keyspace_meta(self.0)) }
    }

    /// Gets the name of the keyspace.
    pub fn name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_keyspace_meta_name(self.0, &mut name, &mut name_length);
            raw2utf8(name, name_length)
        }
    }

    ///Gets a metadata field for the provided name. Metadata fields allow direct
    ///access to the column data found in the underlying "keyspaces" metadata table.
    pub fn field_by_name(&self, name: &str) -> Option<MetadataFieldValue> {
        unsafe {
            let value = cass_keyspace_meta_field_by_name(self.0, str2ref(name));
            if value.is_null() { None } else { Some(MetadataFieldValue(value)) }
        }
    }
}
