use cql_bindgen::CassIteratorType as _CassIteratorType;

use cql_bindgen::cass_iterator_aggregates_from_keyspace_meta;
use cql_bindgen::cass_iterator_columns_from_table_meta;
use cql_bindgen::cass_iterator_fields_from_aggregate_meta;
use cql_bindgen::cass_iterator_fields_from_column_meta;
use cql_bindgen::cass_iterator_fields_from_function_meta;
use cql_bindgen::cass_iterator_fields_from_keyspace_meta;
use cql_bindgen::cass_iterator_fields_from_table_meta;
use cql_bindgen::cass_iterator_fields_from_user_type;
use cql_bindgen::cass_iterator_from_tuple;
use cql_bindgen::cass_iterator_functions_from_keyspace_meta;
use cql_bindgen::cass_iterator_get_aggregate_meta;
use cql_bindgen::cass_iterator_get_column_meta;
use cql_bindgen::cass_iterator_get_function_meta;
use cql_bindgen::cass_iterator_get_keyspace_meta;
use cql_bindgen::cass_iterator_get_map_key;
use cql_bindgen::cass_iterator_get_map_value;
use cql_bindgen::cass_iterator_get_meta_field_name;
use cql_bindgen::cass_iterator_get_meta_field_value;
use cql_bindgen::cass_iterator_get_table_meta;
use cql_bindgen::cass_iterator_get_user_type;
use cql_bindgen::cass_iterator_get_value;
use cql_bindgen::cass_iterator_keyspaces_from_schema_meta;
use cql_bindgen::cass_iterator_tables_from_keyspace_meta;
use cql_bindgen::cass_iterator_type;
use cql_bindgen::cass_iterator_user_types_from_keyspace_meta;


pub struct CassIteratorType(_CassIteratorType);

impl CassIteratorType {
    pub fn new(_type: _CassIteratorType) -> Self {
        CassIteratorType(_type)
    }
}
