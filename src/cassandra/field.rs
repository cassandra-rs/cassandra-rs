// use decimal::d128;

use crate::cassandra::value::{Value, ValueType};

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

// #[repr(C)]
// #[derive(Copy,Debug,Clone)]
// #[allow(missing_docs)]
// pub enum FieldType {
//    PARTITION_KEY = 0,
//    CLUSTERING_KEY = 1,
//    REGULAR = 2,
//    COMPACT_VALUE = 3,
//    STATIC = 4,
//    UNKNOWN = 5,
// }

// impl FieldType {
//    //    pub fn build(type_num: u32) -> Result<FieldType, u32> {
//    //        match type_num {
//    //            //            0 => Ok(PARTITION_KEY),
//    //            //            1 => Ok(CLUSTERING_KEY),
//    //            //            2 => Ok(REGULAR),
//    //            //            3 => Ok(COMPACT_VALUE),
//    //            //            4 => Ok(STATIC),
//    //            //            5 => Ok(UNKNOWN),
//    //            err => Err(err),
//    //        }
//    //    }
// }

/// A field's metadata
//
// Borrowed from wherever the value is borrowed from.
pub struct Field<'a> {
    /// The field's name
    pub name: String,
    /// The field's value
    pub value: Value<'a>,
}

impl Debug for Field<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} Cassandra type", self.get_type())
    }
}

impl Display for Field<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} Cassandra type", self.get_type())
    }
}

impl<'a> Field<'a> {
    /// Gets the name of this field
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Gets the type of this field
    pub fn get_type(&self) -> ValueType {
        self.value.get_type()
    }

    /// Gets the value of this field
    pub fn get_value(&self) -> &Value<'a> {
        &self.value
    }
}
