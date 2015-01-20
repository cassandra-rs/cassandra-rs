#![allow(dead_code)]
#![allow(non_camel_case_types)]

use libc::types::os::arch::c95::c_uint;

pub enum CassConsistency {
    ANY = 0is,
    ONE = 1,
    TWO = 2,
    THREE = 3,
    QUORUM = 4,
    ALL = 5,
    LOCAL_QUORUM = 6,
    EACH_QUORUM = 7,
    SERIAL = 8,
    LOCAL_SERIAL = 9,
    LOCAL_ONE = 10
}
