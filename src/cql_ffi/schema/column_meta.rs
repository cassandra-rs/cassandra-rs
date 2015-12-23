use cql_bindgen::CassColumnMeta as _CassColumnMeta;
use cql_bindgen::cass_column_meta_name;
use cql_bindgen::cass_column_meta_data_type;
use cql_bindgen::cass_column_meta_field_by_name;
use cql_bindgen::cass_column_meta_type;
use cql_bindgen::cass_iterator_fields_from_column_meta;

use cql_ffi::column::ColumnType;
use cql_ffi::iterator::FieldIterator;
use cql_ffi::data_type::ConstDataType;
use cql_ffi::value::Value;

pub struct ColumnMeta(pub *const _CassColumnMeta);

use std::mem;
use std::str;
use std::slice;
use std::ffi::CString;

impl ColumnMeta {
    ///returns an iterator over the fields of this column
    pub fn field_iter(&mut self) -> FieldIterator {
        unsafe { FieldIterator(cass_iterator_fields_from_column_meta(self.0)) }
    }

    ///Gets the name of the column.
    pub fn name(&self) -> String {
        unsafe {
            let name = mem::zeroed();
            let name_length = mem::zeroed();
            cass_column_meta_name(self.0, name, name_length);
            let slice = slice::from_raw_parts(name as *const u8, name_length as usize);
            str::from_utf8(slice).unwrap().to_string()
        }
    }

    ///Gets the type of the column.
    pub fn get_type(&self) -> ColumnType { unsafe { ColumnType::build(cass_column_meta_type(self.0)).unwrap() } }

    ///Gets the data type of the column.
    pub fn data_type(&self) -> ConstDataType { unsafe { ConstDataType(cass_column_meta_data_type(self.0)) } }

    ///Gets a metadata field for the provided name. Metadata fields allow direct
    ///access to the column data found in the underlying "columns" metadata table.
    pub fn field_by_name(&self, name: &str) -> Option<Value> {
        unsafe {
            let field = cass_column_meta_field_by_name(self.0, CString::new(name).unwrap().as_ptr());
            match field.is_null() {
                true => None,
                false => Some(Value(field)),
            }
        }
    }
}
