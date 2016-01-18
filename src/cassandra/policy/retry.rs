use cassandra_sys::cass_retry_policy_default_new;
use cassandra_sys::cass_retry_policy_downgrading_consistency_new;
use cassandra_sys::cass_retry_policy_fallthrough_new;
use cassandra_sys::cass_retry_policy_logging_new;
use cassandra_sys::cass_retry_policy_free;

use cassandra_sys::CassRetryPolicy as _RetryPolicy;

///The selected retry policy
pub struct RetryPolicy(*mut _RetryPolicy);

pub mod protected {
    use cassandra::policy::retry::RetryPolicy;
    use cassandra_sys::CassRetryPolicy as _RetryPolicy;
    pub fn inner(policy: RetryPolicy) -> *mut _RetryPolicy {
        policy.0
    }
}

impl RetryPolicy {
    pub fn default_new() -> Self {
        unsafe { RetryPolicy(cass_retry_policy_default_new()) }
    }

    pub fn downgrading_consistency_new() -> Self {
        unsafe { RetryPolicy(cass_retry_policy_downgrading_consistency_new()) }
    }

    pub fn fallthrough_new() -> Self {
        unsafe { RetryPolicy(cass_retry_policy_fallthrough_new()) }
    }

    pub fn logging_new(child_retry_policy: RetryPolicy) -> Self {
        unsafe { RetryPolicy(cass_retry_policy_logging_new(child_retry_policy.0)) }
    }
}

impl Drop for RetryPolicy {
    fn drop(&mut self) {
        unsafe {
            cass_retry_policy_free(self.0);
        }
    }
}
