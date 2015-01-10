type Enum_CassColumnType_ = c_uint;
pub const CASS_COLUMN_TYPE_PARTITION_KEY: c_uint = 0;
pub const CASS_COLUMN_TYPE_CLUSTERING_KEY: c_uint = 1;
pub const CASS_COLUMN_TYPE_REGULAR: c_uint = 2;
pub const CASS_COLUMN_TYPE_COMPACT_VALUE: c_uint = 3;
pub const CASS_COLUMN_TYPE_STATIC: c_uint = 4;
pub const CASS_COLUMN_TYPE_UNKNOWN: c_uint = 5;
pub type CassColumnType = Enum_CassColumnType_;
