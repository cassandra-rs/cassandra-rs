use cql_bindgen::cass_time_from_epoch;
use cql_bindgen::cass_timestamp_gen_free;
use cql_bindgen::cass_timestamp_gen_monotonic_new;
use cql_bindgen::cass_timestamp_gen_server_side_new;
use cql_bindgen::cass_date_from_epoch;
use cql_bindgen::cass_date_time_to_epoch;

use cql_bindgen::CassTimestampGen as _CassTimestampGen;

// FIXME add chrono support

pub struct TimestampGen(pub *mut _CassTimestampGen);

impl TimestampGen {
    pub fn from_epoch(epoch_seconds: i64) -> i64 { unsafe { cass_time_from_epoch(epoch_seconds) } }

    pub fn gen_monotonic_new() -> Self { unsafe { TimestampGen(cass_timestamp_gen_monotonic_new()) } }

    pub fn gen_server_side_new() -> Self { unsafe { TimestampGen(cass_timestamp_gen_server_side_new()) } }

    //    pub fn from_epoch() -> Self {
    //        unsafe { TimestampGen(cass_timestamp_gen_monotonic_new()) }
    //    }
}

pub fn date_from_epoch(epoch_secs: i64) -> u32 { unsafe { cass_date_from_epoch(epoch_secs) } }
pub fn date_time_from_epoch(date: u32, time: i64) -> i64 { unsafe { cass_date_time_to_epoch(date, time) } }

impl Drop for TimestampGen {
    fn drop(&mut self) { unsafe { cass_timestamp_gen_free(self.0) } }
}
