#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;
use libc::types::os::arch::c95::c_uint;
use libc::types::os::arch::c95::c_long;
use libc::types::os::arch::c95::c_ushort;
use libc::types::os::arch::c95::c_uchar;
use libc::types::os::arch::c95::c_short;
use libc::types::os::arch::c95::c_ulong;
use libc::types::os::arch::c95::c_double;
use libc::types::os::arch::c95::c_float;

pub type cass_byte_t = cass_uint8_t;
pub type ptrdiff_t = c_long;
pub type size_t = c_ulong;
pub type wchar_t = c_int;
pub type Enum_Unnamed1 = c_uint;
pub const cass_false: c_uint = 0;
pub const cass_true: c_uint = 1;
pub type cass_bool_t = Enum_Unnamed1;
pub type cass_float_t = c_float;
pub type cass_double_t = c_double;
pub type cass_int8_t = c_char;
pub type cass_uint8_t = c_uchar;
pub type cass_int16_t = c_short;
pub type cass_uint16_t = c_ushort;
pub type cass_int32_t = c_int;
pub type cass_uint32_t = c_uint;
pub type cass_int64_t = c_long;
pub type cass_uint64_t = c_ulong;
pub type cass_size_t = size_t;
pub type cass_duration_t = cass_uint64_t;
