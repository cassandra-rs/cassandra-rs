use cassandra::util::Protected;
use cassandra_sys::CassConsistency_ as CassConsistency;

use cassandra_sys::cass_consistency_string;

use std::ffi::CStr;
use std::fmt::{self, Display};
use std::str::FromStr;

/// Enhance a nullary enum as follows:
///
/// * `Display` / `to_string`
/// * `FromStr` / `parse` (with just a simple `String` as error type)
/// * Interconvert with another type (via `Protected`).
/// * `variants` yields an array of all variants.
///
/// Only works for nullary enums, i.e., ones where no variants have any arguments.
///
/// # Syntax
///
/// ```ignore
/// enhance_nullary_enum(ThisEnum, ThatEnum, {
///     (ThisVariant1, ThatVariant1, "StringName1"),
///     (ThisVariant2, ThatVariant2, "StringName2"),
///     ...
/// });
/// ```
/// where
///
/// * `ThisEnum` is the name of the type being enhanced.
/// * `ThatEnum` is the name of the inner type wrapped by `ThisEnum`.
/// * Then all variants of `ThisEnum` are listed:
///   * `ThisVariant`i is the name of the variant.
///   * `ThatVariant`i is the name of the corresponding variant of `ThatEnum`.
///   * `StringName`i is the desired string representation of the enum for parsing and printing.
///
//
// We attempted to use the `macro-attr` crate to achieve this more naturally, but sadly
// identifier-oncatenation functionality is not yet available in stable Rust
// per https://github.com/rust-lang/rust/issues/29599. We really don't want to use the verbose
// identifiers (e.g., `CASS_CONSISTENCY_ANY`), so we have to provide an explicit list
// of identifiers; however with `macro-attr` there's nowhere to hang them; we can't add
// them in a custom attribute because custom attributes are unstable.
// In the end the best approach is just the direct one, as shown here.
macro_rules! enhance_nullary_enum {
    ( $this_name:ident, $that_name: ident, { $( ($this:ident, $that:ident, $name:expr), )* } ) => {
        impl Display for $this_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                write!(f, "{}", match *self {
                    $( $this_name::$this => $name, )*
                })
            }
        }

        impl FromStr for $this_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( $name => Ok($this_name::$this), )*
                    _ => Err(format!("Unrecognized {}: {}", stringify!($this_name), s)),
                }
            }
        }

        impl Protected<$that_name> for $this_name {
            fn build(inner: $that_name) -> Self {
                match inner {
                    $( $that_name::$that=> $this_name::$this ),*
                }
            }
            fn inner(&self) -> $that_name {
                match *self {
                    $( $this_name::$this => $that_name::$that ),*
                }
            }
        }

        impl $this_name {
            /// List all the possible values of this enumeration.
            pub fn variants() -> &'static [$this_name] {
                // Nasty trick to calculate the length of the iteration - we must mention a
                // variable inside the layer, even though we never actually use it.
                static VARIANTS: [ $this_name; 0 $( + ($this_name::$this, 1).1 )* ] = [ $( $this_name::$this ),* ];
                &VARIANTS
            }
        }
    }
}

/// A Cassandra consistency level.
#[derive(Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Meanings are defined in CQL documentation.
#[allow(non_camel_case_types)] // Names are traditional.
pub enum Consistency {
    UNKNOWN,
    ANY,
    ONE,
    TWO,
    THREE,
    QUORUM,
    ALL,
    LOCAL_QUORUM,
    EACH_QUORUM,
    SERIAL,
    LOCAL_SERIAL,
    LOCAL_ONE,
}

enhance_nullary_enum!(Consistency, CassConsistency, {
    (UNKNOWN, CASS_CONSISTENCY_UNKNOWN, "UNKNOWN"),
    (ANY, CASS_CONSISTENCY_ANY, "ANY"),
    (ONE, CASS_CONSISTENCY_ONE, "ONE"),
    (TWO, CASS_CONSISTENCY_TWO, "TWO"),
    (THREE, CASS_CONSISTENCY_THREE, "THREE"),
    (QUORUM, CASS_CONSISTENCY_QUORUM, "QUORUM"),
    (ALL, CASS_CONSISTENCY_ALL, "ALL"),
    (LOCAL_QUORUM, CASS_CONSISTENCY_LOCAL_QUORUM, "LOCAL_QUORUM"),
    (EACH_QUORUM, CASS_CONSISTENCY_EACH_QUORUM, "EACH_QUORUM"),
    (SERIAL, CASS_CONSISTENCY_SERIAL, "SERIAL"),
    (LOCAL_SERIAL, CASS_CONSISTENCY_LOCAL_SERIAL, "LOCAL_SERIAL"),
    (LOCAL_ONE, CASS_CONSISTENCY_LOCAL_ONE, "LOCAL_ONE"),
});
