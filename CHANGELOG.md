# Changelog

All notable changes to this project will be documented in this file.

This file's format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/). The
version number is tracked in the file `VERSION`.

## [Unreleased]
### Added
### Changed
### Fixed

## [0.14.1] - 2019-11-07
### Changed
- Add code example for SSL.
- Updated error-chain to 0.12.1 to avoid `Error` deprecation warnings.

### Fixed
- Provide missing doc comment, fix unused doc comment warnings.
- Fix type signature on `set_load_balance_dc_aware` so it can be used.

## [0.14.0] - 2019-01-22
### Added
- `Clone`, `Copy` and `PartialEq` traits in `Inet`
- Fields name and value support for `UserTypeIterator` (support for UDT)
- Extra bindings for `Row::get_by_name` and `Statement::bind_by_name`

### Changed
- `Debug` implementation for `Inet` now uses its `ToString` implementation

### Fixed
- `Value::get_inet` which would always return a zeroed `Inet`
- Dropping futures early could cause a segfault when using the system
  allocator (in Rust 1.32.0 or later).

## [0.13.2] - 2019-01-15
- Avoid possible segfaults, by returning `None` where possible, otherwise
  panicking. In particular, a collection field set to NULL now returns `None`
  rather than faulting.
- Make `SchemaMeta::get_keyspace_by_name` work (fix string handling bug).
- Allow using the `SetIterator` for lists and tuples. Previously these
  could not be enumerated at all!
- For convenience, support `bind()` for `List`s.

## [0.13.1] - 2019-01-08
- Fix `stmt!()` not working if `Statement` was not imported.

## [0.13.0] - 2018-12-04
- Added new set_local_address function using the function Datastax added in
  cassandra-cpp-driver version 2.10.0

## [0.12.0] - 2018-12-04
### Fixed
- No longer leaks all `CassResult`s.
### Changed
- Updated cassandra-cpp-sys to 0.12.
- Updated cassandra-cpp-driver to 2.10.0
- cql protocol version 2 is no longer supported.
- Breaking changes: The Cassandra WriteType UKNOWN is now called UNKNOWN
                    There is a new Cassandra error code LIB_EXECUTION_PROFILE_INVALID
                    There is a new Cassandra value type: DURATION
- `ResultIterator` now has a lifetime parameter. The underlying `CassResult` must live for at
  least as long as the iterator.
- `CassResult` is no longer `IntoIterator`; instead `&CassResult` is. You must change code
  like `for row in result` to `for row in &result` and ensure `result` lives long enough.

## [0.11.0] - 2018-04-26
- Remove the `AsInet` and `FromInet` traits, replacing them with suitable implementations of `From`.
- Fixed buggy IPv6 conversions.
- `Inet::cass_inet_init_v4` and `Inet::cass_inet_init_v6` no longer consume their arguments.
- `Tuple::set_inet()` now takes an `IpAddr` rather than a `SocketAddr`.
- Added wrapper for `cass_statement_set_request_timeout`.

## [0.10.2] - 2017-09-11
- Move to latest (0.11) version of `cassandra-cpp-sys` crate.

  There should be no external impact.

## [0.10.1] - 2017-08-30
- Remove unnecessary dependency on `ip` crate.
- Add `Copy`, `Clone`, `Hash` impls for all nullary enums.
- Specify correct `Send` and `Sync` markers for all C* types.

## [0.10.0] - 2017-08-03
The API changed significantly in version 0.10. Here is a summary of the main changes.
(Version 0.9 was skipped, for consistency with the `cassandra-cpp-sys` version number.)

Errors:

- The internal module `errors` and the underlying `cassandra-cpp-sys` crate are
  no longer exposed in the API.
  All necessary types are defined in this crate's root module.
- All errors are now reported consistently using a single newly-defined `Error` type.
  - The crate makes every effort to return an error rather than panicking.
  - `CassError`, `CassErrorResult`, and others are replaced by `Error` and
  `CassErrorCode`.
  - Several return types have changed from `T` to `Result<T, Error>`.

Futures:

- There is only a single future type, `CassFuture`, and it implements the
  Rust/Tokio [futures](https://docs.rs/futures) API. It interoperates smoothly
  with existing futures code.
  - `Future`, `CloseFuture`, `ResultFuture`, `PreparedFuture`, `SessionFuture`
    are all subsumed.
  - `wait` is replaced with `Future::wait`; other methods have standard analogues
    as well. See the [futures]((https://docs.rs/futures)) documentation for details.
  - Callbacks can no longer be set explicitly on a future; instead the normal
    futures mechanisms (e.g., `and_then`) should be used.

Values:

- The `Column` type is retired; instead use `Value`.
- Some `Value` getters have new names for consistency, e.g.,
  `get_flt` and `get_dbl` are now `get_f32` and `get_f64` respectively.
- `Value::get_string` now gets a `String`, not a `&str`; you can get a `&str` with `get_str`.
- The "magic" auto-converting `Row` getters `get_col` and `get_col_by_name` are renamed
  to `get` and `get_by_name` respectively. This is to avoid confusion with `get_column`, which is
  something else entirely (it gets a `Value` from a `Row`).
- Values have a new `is_null` method to allow retrieving null values.
- UUIDs now support `Eq` and `Ord`.

Miscellaneous types:

- Several types which wrapped `cassandra-cpp-sys` types now have enums of their
  own, complete with implementations of `Debug`, `Eq`, `PartialEq`, `Display`,
  and `FromStr`. This includes `BatchType`, `CassErrorCode`, `Consistency`,
  `LogLevel`, `SslVerifyFlag`, and `ValueType`.
- `CqlProtocol` is now simply an alias for an integer.
- Contact points are now expressed as a simple string in the driver's preferred format.
  `ContactPoints` is retired.

Other:

- Logging uses the `slog` crate. It is no longer possible to set your own logging
  callback, but you can set the `slog` logger.
- Internally, the code is cleaner and smaller and some tests have been added.

## [0.8.2] - 2017-06-29
First release of https://github.com/Metaswitch/cassandra-rs

- Fork package.
- Move examples to examples directory, then make several into Rust tests.
- Resolve all warnings.
- Fix various panics.
- Add `Eq` and `Ord` for `Uuid`.
- Add `is_null` support.

## [0.8.1] - 2016-12-13
Last release of https://github.com/tupshin/cassandra-rs

[Unreleased]: https://github.com/Metaswitch/cassandra-rs/compare/0.14.0...HEAD
[0.14.0]: https://github.com/Metaswitch/cassandra-rs/compare/0.13.2...0.14.0
[0.13.2]: https://github.com/Metaswitch/cassandra-rs/compare/0.13.1...0.13.2
[0.13.1]: https://github.com/Metaswitch/cassandra-rs/compare/0.13.0...0.13.1
[0.13.0]: https://github.com/Metaswitch/cassandra-rs/compare/0.12.0...0.13.0
[0.12.0]: https://github.com/Metaswitch/cassandra-rs/compare/0.11.0...0.12.0
[0.11.0]: https://github.com/Metaswitch/cassandra-rs/compare/0.10.2...0.11.0
[0.10.2]: https://github.com/Metaswitch/cassandra-rs/compare/0.10.1...0.10.2
[0.10.1]: https://github.com/Metaswitch/cassandra-rs/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/Metaswitch/cassandra-rs/compare/0.8.2...0.10.0
[0.8.2]: https://github.com/Metaswitch/cassandra-rs/compare/0.8.1...0.8.2
[0.8.1]: https://github.com/Metaswitch/cassandra-rs/tree/0.8.1
