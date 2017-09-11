## 0.10.2 (2017-09-11)

* Move to latest (0.11) version of `cassandra-cpp-sys` crate.
  There should be no external impact.


## 0.10.1 (2017-08-30)

* Remove unnecessary dependency on `ip` crate.
* Add `Copy`, `Clone`, `Hash` impls for all nullary enums.
* Specify correct `Send` and `Sync` markers for all C* types.


## 0.10.0 (2017-08-03)

The API changed significantly in version 0.10. Here is a summary of the main changes.
(Version 0.9 was skipped, for consistency with the `cassandra-cpp-sys` version number.)

Errors:

* The internal module `errors` and the underlying `cassandra-cpp-sys` crate are
  no longer exposed in the API.
  All necessary types are defined in this crate's root module.
* All errors are now reported consistently using a single newly-defined `Error` type.
  * The crate makes every effort to return an error rather than panicking.
  * `CassError`, `CassErrorResult`, and others are replaced by `Error` and
  `CassErrorCode`.
  * Several return types have changed from `T` to `Result<T, Error>`.

Futures:

* There is only a single future type, `CassFuture`, and it implements the
  Rust/Tokio [futures](https://docs.rs/futures) API. It interoperates smoothly
  with existing futures code.
  * `Future`, `CloseFuture`, `ResultFuture`, `PreparedFuture`, `SessionFuture`
    are all subsumed.
  * `wait` is replaced with `Future::wait`; other methods have standard analogues
    as well. See the [futures]((https://docs.rs/futures)) documentation for details.
  * Callbacks can no longer be set explicitly on a future; instead the normal
    futures mechanisms (e.g., `and_then`) should be used.

Values:

* The `Column` type is retired; instead use `Value`.
* Some `Value` getters have new names for consistency, e.g.,
  `get_flt` and `get_dbl` are now `get_f32` and `get_f64` respectively.
* `Value::get_string` now gets a `String`, not a `&str`; you can get a `&str` with `get_str`.
* The "magic" auto-converting `Row` getters `get_col` and `get_col_by_name` are renamed
  to `get` and `get_by_name` respectively. This is to avoid confusion with `get_column`, which is
  something else entirely (it gets a `Value` from a `Row`).
* Values have a new `is_null` method to allow retrieving null values.
* UUIDs now support `Eq` and `Ord`.

Miscellaneous types:

* Several types which wrapped `cassandra-cpp-sys` types now have enums of their
  own, complete with implementations of `Debug`, `Eq`, `PartialEq`, `Display`,
  and `FromStr`. This includes `BatchType`, `CassErrorCode`, `Consistency`,
  `LogLevel`, `SslVerifyFlag`, and `ValueType`.
* `CqlProtocol` is now simply an alias for an integer.
* Contact points are now expressed as a simple string in the driver's preferred format.
  `ContactPoints` is retired.

Other:

* Logging uses the `slog` crate. It is no longer possible to set your own logging
  callback, but you can set the `slog` logger.
* Internally, the code is cleaner and smaller and some tests have been added.


## 0.8.2 (2017-06-29)

First release of https://github.com/Metaswitch/cassandra-rs

* Fork package.
* Move examples to examples directory, then make several into Rust tests.
* Resolve all warnings.
* Fix various panics.
* Add `Eq` and `Ord` for `Uuid`.
* Add `is_null` support.


## 0.8.1 (2016-12-13)

Last release of https://github.com/tupshin/cassandra-rs
