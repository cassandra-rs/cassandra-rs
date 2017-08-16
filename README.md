[![Build Status](https://travis-ci.org/Metaswitch/cassandra-rs.svg?branch=master)](https://travis-ci.org/Metaswitch/cassandra-rs)
[![Current Version](https://img.shields.io/crates/v/cassandra-cpp.svg)](https://crates.io/crates/cassandra-cpp)
[![License](https://img.shields.io/github/license/Metaswitch/cassandra-rs.svg)](#License)

# cassandra-cpp

This is a maintained Rust project that
exposes the DataStax cpp driver at https://github.com/datastax/cpp-driver/
in a somewhat-sane crate.

It is a wrapper around the raw driver binding crate [cassandra-cpp-sys](https://github.com/Metaswitch/cassandra-sys-rs).

[Documentation (crates.io)](https://docs.rs/cassandra-cpp).


## Getting started

You can use this crate from cargo with

```toml
    [dependencies]
    cassandra-cpp = "0.10"
```

For this crate to work, you must first have installed the datastax-cpp driver.
Follow the steps in the
[cpp driver docs](https://github.com/datastax/cpp-driver/tree/master/topics#installation)
to do so. Pre-built packages are available for most platforms.

Make sure that the driver (specifically `libcassandra_static.a` and `libcassandra.so`) are in your `/usr/local/lib64/` directory


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


## Migrating from version 0.8

The API changed significantly in version 0.10.
(Version 0.9 was skipped, for consistency with the `cassandra-cpp-sys` version number.)
For a summary of the main changes, see [`CHANGELOG`](CHANGELOG.md#0100).


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
