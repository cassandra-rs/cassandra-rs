type Enum_CassCompression_ = c_uint;
pub const CASS_COMPRESSION_NONE: c_uint = 0;
pub const CASS_COMPRESSION_SNAPPY: c_uint = 1;
pub const CASS_COMPRESSION_LZ4: c_uint = 2;
pub type CassCompression = Enum_CassCompression_;
