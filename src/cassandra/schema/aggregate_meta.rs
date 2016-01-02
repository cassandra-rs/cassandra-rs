use std::mem;
use std::ffi::CString;

use cassandra_sys::CassAggregateMeta as _CassAggregateMeta;
use cassandra_sys::raw2utf8;
use cassandra_sys::cass_iterator_fields_from_aggregate_meta;
use cassandra_sys::cass_aggregate_meta_name;
use cassandra_sys::cass_aggregate_meta_full_name;
use cassandra_sys::cass_aggregate_meta_argument_count;
use cassandra_sys::cass_aggregate_meta_argument_type;
use cassandra_sys::cass_aggregate_meta_return_type;
use cassandra_sys::cass_aggregate_meta_state_type;
use cassandra_sys::cass_aggregate_meta_state_func;
use cassandra_sys::cass_aggregate_meta_final_func;
use cassandra_sys::cass_aggregate_meta_init_cond;
use cassandra_sys::cass_aggregate_meta_field_by_name;

use cassandra::schema::function_meta::FunctionMeta;
use cassandra::data_type::ConstDataType;
use cassandra::value::Value;
use cassandra::iterator::FieldIterator;

pub struct AggregateMeta(pub *const _CassAggregateMeta);

impl AggregateMeta {
    pub fn fields_iter(&self) -> FieldIterator {
        unsafe { FieldIterator(cass_iterator_fields_from_aggregate_meta(self.0)) }
    }


    /// Gets the name of the aggregate.
    pub fn get_name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_aggregate_meta_name(self.0, &mut name, &mut name_length);
            raw2utf8(name, name_length).unwrap()
        }
    }

    /// Gets the full name of the aggregate.
    pub fn full_name(&self) -> String {
        unsafe {
            let mut name = mem::zeroed();
            let mut name_length = mem::zeroed();
            cass_aggregate_meta_full_name(self.0, &mut name, &mut name_length);
            raw2utf8(name, name_length).unwrap()
        }
    }

    /// Gets the number of arguments this aggregate takes.
    pub fn argument_count(&self) -> u64 {
        unsafe { cass_aggregate_meta_argument_count(self.0) }
    }

    /// Gets the aggregate's argument type for the provided index.
    pub fn argument_type(&self, index: u64) -> ConstDataType {
        unsafe { ConstDataType(cass_aggregate_meta_argument_type(self.0, index)) }
    }

    /// Gets the aggregate's argument return type.
    pub fn return_type(&self) -> ConstDataType {
        unsafe { ConstDataType(cass_aggregate_meta_return_type(self.0)) }
    }

    /// Gets the aggregate's argument state type.
    pub fn state_type(&self) -> ConstDataType {
        unsafe { ConstDataType(cass_aggregate_meta_state_type(self.0)) }
    }

    /// Gets the function metadata for the aggregate's state function.
    pub fn state_func(&self) -> FunctionMeta {
        unsafe { FunctionMeta(cass_aggregate_meta_state_func(self.0)) }
    }

    /// Gets the function metadata for the aggregates's final function.
    pub fn final_func(&self) -> FunctionMeta {
        unsafe { FunctionMeta(cass_aggregate_meta_final_func(self.0)) }
    }

    ///  Gets the initial condition value for the aggregate.
    pub fn init_cond(&self) -> Value {
        unsafe { Value(cass_aggregate_meta_init_cond(self.0)) }
    }

    ///  Gets a metadata field for the provided name. Metadata fields allow direct
    ///access to the column data found in the underlying "aggregates" metadata table.
    pub fn field_by_name(&self, name: &str) -> Option<Value> {
        unsafe {
            let agg = cass_aggregate_meta_field_by_name(self.0, CString::new(name).unwrap().as_ptr());
            if agg.is_null() { None } else { Some(Value(agg)) }
        }
    }
}
