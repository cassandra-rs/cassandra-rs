use cql_bindgen::cass_table_meta_clustering_key;
use cql_bindgen::cass_table_meta_clustering_key_count;
use cql_bindgen::cass_table_meta_column;
use cql_bindgen::cass_table_meta_column_by_name;
use cql_bindgen::cass_table_meta_column_count;
use cql_bindgen::cass_table_meta_field_by_name;
use cql_bindgen::cass_table_meta_name;
use cql_bindgen::cass_table_meta_partition_key;
use cql_bindgen::cass_table_meta_partition_key_count;
use cql_bindgen::CassTableMeta as _CassTableMeta;
use cql_bindgen::cass_iterator_columns_from_table_meta;
use cql_bindgen::cass_iterator_fields_from_table_meta;

use cql_ffi::iterator::FieldIterator;
use cql_ffi::iterator::ColumnIterator;

use std::str;
use std::slice;
use std::mem;

use cql_ffi::schema::column_meta::ColumnMeta;
use cql_ffi::value::Value;

pub struct TableMeta(pub *const _CassTableMeta);

impl TableMeta {
    ///returns an iterator over the fields of this table
    pub fn field_iter(&mut self) -> FieldIterator {
        unsafe { FieldIterator(cass_iterator_fields_from_table_meta(self.0)) }
    }

    pub fn columns_iter(&self) -> ColumnIterator {
        unsafe { ColumnIterator(cass_iterator_columns_from_table_meta(self.0)) }
    }

    ///Gets the column metadata for the provided column name.
    pub fn column_by_name(&self, name: &str) -> ColumnMeta {
        unsafe { ColumnMeta(cass_table_meta_column_by_name(self.0, name.as_ptr() as *const i8)) }
    }

    ///Gets the name of the table.
    pub fn name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_table_meta_name(self.0, &mut name, &mut name_length);
            let slice = slice::from_raw_parts(name as *const u8, name_length as usize);
            str::from_utf8(slice).unwrap().to_string()
        }
    }

    ///Gets the total number of columns for the table.
    pub fn column_count(&self) -> u64 { unsafe { cass_table_meta_column_count(self.0) } }

    ///Gets the column metadata for the provided index.
    pub fn column(&self, index: u64) -> ColumnMeta { unsafe { ColumnMeta(cass_table_meta_column(self.0, index)) } }

    ///Gets the number of columns for the table's partition key.
    pub fn partition_key_count(&self) -> u64 { unsafe { cass_table_meta_partition_key_count(self.0) } }

    ///Gets the partition key column metadata for the provided index.
    pub fn partition_key(&self, index: u64) -> Option<ColumnMeta> {
        unsafe {
            let key = cass_table_meta_partition_key(self.0, index);
            match key.is_null() {
                true => None,
                false => Some(ColumnMeta(key)),
            }
        }
    }

    ///Gets the number of columns for the table's clustering key
    pub fn clustering_key_count(&self) -> u64 { unsafe { cass_table_meta_clustering_key_count(self.0) } }

    ///Gets the clustering key column metadata for the provided index.
    pub fn cluster_key(&self, index: u64) -> Option<ColumnMeta> {
        unsafe {
            let key = cass_table_meta_clustering_key(self.0, index);
            match key.is_null() {
                true => None,
                false => Some(ColumnMeta(key)),
            }
        }
    }


    pub fn field_by_name(&self, name: &str) -> Option<Value> {
        // fixme replace CassValule with a custom type
        unsafe {
            let value = cass_table_meta_field_by_name(self.0, name.as_ptr() as *const i8);
            if value.is_null() { None } else { Some(Value(value)) }

        }
    }
}
