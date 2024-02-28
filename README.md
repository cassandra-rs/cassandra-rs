[![Build Status](https://github.com/Metaswitch/cassandra-rs/actions/workflows/build.yml/badge.svg)](https://github.com/Metaswitch/cassandra-rs/actions)
[![Current Version](https://img.shields.io/crates/v/cassandra-cpp.svg)](https://crates.io/crates/cassandra-cpp)
[![License](https://img.shields.io/github/license/Metaswitch/cassandra-rs.svg)](#License)

# cassandra-cpp

This is a maintained Rust project that
exposes the DataStax cpp driver at https://github.com/datastax/cpp-driver/
in a somewhat-sane crate.
It was originally a fork of https://github.com/tupshin/cassandra-rs but that is no longer maintained.

It is a wrapper around the raw driver binding crate [cassandra-cpp-sys](https://github.com/Metaswitch/cassandra-sys-rs).

[Documentation (crates.io)](https://docs.rs/cassandra-cpp).


## Getting started

### Local environment

For this crate to work, you must first have installed a sufficiently-recent version of the datastax-cpp driver (at least 2.16).
Follow the steps in the
[cpp driver docs](https://github.com/datastax/cpp-driver/tree/master/topics#installation)
to do so. Pre-built packages are available for most platforms.

Make sure that the driver (specifically `libcassandra_static.a` and `libcassandra.so`) are in your `/usr/local/lib64/` directory

### Floki

Alternatively you can use the [Floki](https://github.com/Metaswitch/floki) utility to create you a Dockerized compilation environment. After installing Floki, just type

```
floki
```

in the root of this project. You will be dropped into a Rust compilation environment; type `cargo build` as normal to build the driver.

## Documentation

See the [API documentation](https://docs.rs/cassandra-cpp).

The [Cassandra Query Language (CQL) documentation](http://docs.datastax.com/en/cql/3.3/cql/cql_reference/cqlCommandsTOC.html)
is likely to be useful.

Since this crate provides a relatively
thin wrapper around the DataStax driver, you may also find the DataStax
[documentation](http://datastax.github.io/cpp-driver/topics/) and
[API docs](http://datastax.github.io/cpp-driver/api/) useful.


## Example

For a straightforward example see [`simple.rs`](examples/simple.rs).
    
There are additional examples included with the project in [`tests`](tests/) and
[`examples`](examples/).

## Lending iterator API (version 3.0)

Version 3.0 fixes a soundness issue with the previous API. The iterators in the
underlying Cassandra driver invalidate the current item when `next()` is called,
and this was not reflected in the Rust binding prior to version 3.

To deal with this, the various iterators (`ResultIterator`, `RowIterator`,
`MapIterator`, `SetIterator`, `FieldIterator`, `UserTypeIterator`,
`KeyspaceIterator`, `FunctionIterator`, `AggregateIterator`, `TableIterator`,
`ColumnIterator`) no longer implement `std::iter::Iterator`. Instead, since this
is a [lending
iterator,](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#generic-associated-types-gats)
these types all implement a new `LendingIterator` trait. We define this
ourselves because there is currently no widely-used crate that implements it.

To upgrade, change

```rust
for row in result {
  // ... do something with row ...
}
```

to

```rust
let mut iter = result.iter();
while let Some(row) = iter.next() {
  // ... do something with row ...
}
```

The intermediate variable `iter` is necessary, otherwise you will infinitely
visit the first row of the result!

Other changes:

* Many types now take a lifetime argument, e.g., `Value` is now `Value<'a>`,
  `ResultIterator` is now `ResultIterator<'a>`. In almost all cases you can omit
  this and it will be inferred for you. If not, you can usually write
  `Value<'_>` to let Rust worry about it for you.
* `RowIterator` no longer implements `Display` (since it would consume the
  iterator); however `Row` does.
* `TupleIterator` is removed - it was never used, since you use the set iterator
  (Value::get_set()) for lists, sets, and tuples.
* `ConstDataType::sub_data_by_name` and `ConstDataType::sub_type_name` now take
  `&self` rather than an explicit argument.
* `FunctionMeta::argument` now returns the name and type, rather than just `()`.


## New session API (version 2.0)

Version 2.0 introduces a new and safer API. `Statement`s (and
`PreparedStatement` and `Batch`) are now associated with a specific `Session`.
In addition, the legacy `.wait()` API is removed in favour of the now-ubiquitous
`.await`.

* This crate's functions have became `async`, meaning they can only be called as
  part of an asynchronous workflow. To use these functions, you can either call
  them from within an asynchronous function using the `.await` operator, or you
  can call them from a synchronous context using the `block_on` method from
  [tokio
  runtime](https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on).

* The `stmt!` macro and `Statement::new` method have been replaced with the
  `Session::statement()` method, which records the association with the session.
  Simply update your code to use the new method instead of the macro to continue
  using its functionality.

* Statements are executed with `.execute()`, which consumes
  the statement: you cannot execute the same statement twice; if you need this,
  recreate the statement.

* `Batch::new` is removed in favour of `Session::batch`.
  
* There is a new error, `BatchSessionMismatch`, which occurs if you try to add
  statements from different `Session`s into the same `Batch`.

* Connection methods are tidied up. `Cluster::connect_async` is removed since
 `Cluster::connect` is now async. `Session::connect` and
 `Session::connect_keyspace` are removed - use `Cluster::connect` and
 `Cluster::connect_keyspace` instead.

* `Session::close` (which allowed waiting until in-flight requests on the
  session were complete) is removed because it is non-trivial to implement
  safely. This functionality is no longer supported.

* `Cluster::set_ssl` now consumes its argument, for improved safety.


## Futures (version 0.15)

Since version 0.15, this crate uses `std::future`, allowing your code to
use `futures:0.3`, `async/await`, etc.

Previous versions (up to 0.14) used `futures:0.1`. You can either remain on
the 0.14 stream, update your code to use `std::future`, or use a compatibility
shim (e.g., `futures::compat`).


## Migrating from version 0.8

The API changed significantly in version 0.10.
(Version 0.9 was skipped, for consistency with the `cassandra-cpp-sys` version number.)
For a summary of the main changes, see [`CHANGELOG`](CHANGELOG.md#0100).

## Feature flags

This crate includes the feature flag `early_access_min_tls_version`, which allows you to build against a version of the DataStax driver including the `cass_ssl_set_min_protocol_version` method, as defined in [this PR](https://github.com/datastax/cpp-driver/pull/525). You must have a version of the driver supporting this installed locally to be able to compile (and run) with this feature flag.

When this this feature is available in the mainline driver this flag will be set to do nothing and deprecated, and the functions will be added to the main library. The flag will then be retired in the next breaking change.

## License

This code is open source, licensed under the Apache License Version 2.0 as
described in [`LICENSE`](LICENSE).


## Contributing

Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for details on how to contribute
to this project.


## Development

This crate is regularly built by Travis; to see details of the most recent builds
click on the "build" badge at the top of this page.

You must have the DataStax driver installed on your system in order to build
this crate.

The unit tests assume Cassandra is running on the local host accessible on the
standard port. The easiest way to achieve this is using Docker and the standard
Cassandra image, with
```
docker pull cassandra
docker run -d --net=host --name=cassandra cassandra
```

You should run them single-threaded to avoid the dreaded
`org.apache.cassandra.exceptions.ConfigurationException: Column family ID mismatch`
error. The tests share a keyspace and tables, so if run in parallel they
interfere with each other.
```
cargo test -- --test-threads 1
```

Remember to destroy the container when you're done:
```
docker stop cassandra
docker rm cassandra
```

## History

This project was forked from [cassandra](https://github.com/tupshin/cassandra-rs), which was no longer being maintained.
