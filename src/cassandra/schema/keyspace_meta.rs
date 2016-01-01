use cassandra::schema::aggregate_meta::AggregateMeta;
use cassandra_sys::raw2utf8;
use cassandra_sys::CassValue as _CassValue;

use cassandra_sys::cass_iterator_tables_from_keyspace_meta;
use cassandra_sys::cass_keyspace_meta_aggregate_by_name;
use cassandra_sys::cass_keyspace_meta_field_by_name;
use cassandra_sys::cass_keyspace_meta_function_by_name;
use cassandra_sys::cass_keyspace_meta_name;
use cassandra_sys::cass_keyspace_meta_table_by_name;
use cassandra_sys::cass_keyspace_meta_user_type_by_name;
use cassandra_sys::cass_iterator_aggregates_from_keyspace_meta;
use cassandra_sys::cass_iterator_fields_from_keyspace_meta;
use cassandra_sys::cass_iterator_functions_from_keyspace_meta;
use cassandra_sys::cass_iterator_user_types_from_keyspace_meta;

use cassandra::schema::function_meta::FunctionMeta;
use cassandra::schema::table_meta::TableMeta;
use cassandra::data_type::ConstDataType;
use cassandra::iterator::TableIterator;
use cassandra::iterator::FieldIterator;
use cassandra::iterator::AggregateIterator;
use cassandra::iterator::FunctionIterator;
use cassandra::iterator::UserTypeIterator;
use std::mem;
use std::ffi::CString;

use cassandra_sys::CassKeyspaceMeta as _CassKeyspaceMeta;

pub struct KeyspaceMeta(pub *const _CassKeyspaceMeta);

pub struct MetadataFieldValue(*const _CassValue);

impl KeyspaceMeta {
    pub fn aggregrates_iterator(&self) -> AggregateIterator {
        unsafe { AggregateIterator(cass_iterator_aggregates_from_keyspace_meta(self.0)) }
    }

    pub fn fields_iter(&self) -> FieldIterator {
        unsafe { FieldIterator(cass_iterator_fields_from_keyspace_meta(self.0)) }
    }

    ///Gets the table metadata for the provided table name.
    pub fn table_by_name(&self, name: &str) -> Option<TableMeta> {
        unsafe {
            let value = cass_keyspace_meta_table_by_name(self.0, CString::new(name).unwrap().as_ptr());
            if value.is_null() { None } else { Some(TableMeta(value)) }
        }
    }

    ///Gets the data type for the provided type name.
    pub fn user_type_by_name(&self, name: &str) -> Option<ConstDataType> {
        unsafe {
            let value = cass_keyspace_meta_user_type_by_name(self.0, CString::new(name).unwrap().as_ptr());
            if value.is_null() { None } else { Some(ConstDataType(value)) }
        }
    }

    ///Gets the function metadata for the provided function name.
    pub fn get_function_by_name(&self, name: &str, arguments: Vec<&str>) -> Option<FunctionMeta> {
        unsafe {
            let value = cass_keyspace_meta_function_by_name(self.0,
                                                            CString::new(name).unwrap().as_ptr(),
                                                            CString::new(arguments.join(","))
                                                                .unwrap()
                                                                .as_ptr());
            if value.is_null() { None } else { Some(FunctionMeta(value)) }
        }
    }

    ///Gets the aggregate metadata for the provided aggregate name.
    pub fn aggregate_by_name(&self, name: &str, arguments: Vec<&str>) -> Option<AggregateMeta> {
        unsafe {
            let agg = cass_keyspace_meta_aggregate_by_name(self.0,
                                                           CString::new(name).unwrap().as_ptr(),
                                                           CString::new(arguments.join(","))
                                                               .unwrap()
                                                               .as_ptr());
            if agg.is_null() { None } else { Some(AggregateMeta(agg)) }
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
            raw2utf8(name, name_length).unwrap()
        }
    }

    ///Gets a metadata field for the provided name. Metadata fields allow direct
    ///access to the column data found in the underlying "keyspaces" metadata table.
    pub fn field_by_name(&self, name: &str) -> Option<MetadataFieldValue> {
        unsafe {
            let value = cass_keyspace_meta_field_by_name(self.0, CString::new(name).unwrap().as_ptr());
            if value.is_null() { None } else { Some(MetadataFieldValue(value)) }
        }
    }
}
