use cassandra::util::Protected;
use cassandra_sys::CassWriteType_;

/// A Cassandra write type level.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[allow(missing_docs)] // Meanings are defined in CQL documentation.
#[allow(non_camel_case_types)] // Names are traditional.
pub enum WriteType {
    UKNOWN /* [sic] */,
    SIMPLE,
    BATCH,
    UNLOGGED_BATCH,
    COUNTER,
    BATCH_LOG,
    CAS,
}

enhance_nullary_enum!(WriteType, CassWriteType_, {
(UKNOWN, CASS_WRITE_TYPE_UKNOWN, "UKNOWN"),
(SIMPLE, CASS_WRITE_TYPE_SIMPLE, "SIMPLE"),
(BATCH, CASS_WRITE_TYPE_BATCH, "BATCH"),
(UNLOGGED_BATCH, CASS_WRITE_TYPE_UNLOGGED_BATCH, "UNLOGGED_BATCH"),
(COUNTER, CASS_WRITE_TYPE_COUNTER, "COUNTER"),
(BATCH_LOG, CASS_WRITE_TYPE_BATCH_LOG, "BATCH_LOG"),
(CAS, CASS_WRITE_TYPE_CAS, "CAS"),
});
