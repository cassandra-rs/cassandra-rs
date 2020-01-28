use crate::cassandra::util::Protected;
use crate::cassandra_sys::CassWriteType_;

/// A Cassandra write type level.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[allow(missing_docs)] // Meanings are defined in CQL documentation.
#[allow(non_camel_case_types)] // Names are traditional.
pub enum WriteType {
    UNKNOWN,
    SIMPLE,
    BATCH,
    UNLOGGED_BATCH,
    COUNTER,
    BATCH_LOG,
    CAS,
    VIEW,
    CDC,
}

enhance_nullary_enum!(WriteType, CassWriteType_, {
(UNKNOWN, CASS_WRITE_TYPE_UNKNOWN, "UNKNOWN"),
(SIMPLE, CASS_WRITE_TYPE_SIMPLE, "SIMPLE"),
(BATCH, CASS_WRITE_TYPE_BATCH, "BATCH"),
(UNLOGGED_BATCH, CASS_WRITE_TYPE_UNLOGGED_BATCH, "UNLOGGED_BATCH"),
(COUNTER, CASS_WRITE_TYPE_COUNTER, "COUNTER"),
(BATCH_LOG, CASS_WRITE_TYPE_BATCH_LOG, "BATCH_LOG"),
(CAS, CASS_WRITE_TYPE_CAS, "CAS"),
(VIEW, CASS_WRITE_TYPE_VIEW, "VIEW"),
(CDC, CASS_WRITE_TYPE_CDC, "CDC"),
});
