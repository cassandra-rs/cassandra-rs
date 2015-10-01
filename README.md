# cassandra-rs

This is a (hopefully) maintained rust project that unsafely 
exposes the cpp driver at https://github.com/datastax/cpp-driver/
in a somewhat-sane crate. 

For the wrapper to work, you must first have installed the datastax-cpp driver.

Follow the steps on the cpp driver [docs](https://github.com/datastax/cpp-driver/blob/15215e170810433511c48c304b9e9ca51ff32b2f/topics/building/README.md)  to do so. 

Make sure that the driver (specifically `libcassandra_static.a`) is in your `/usr/local/lib64/` directory

You can use it from cargo with

    [dependencies.cassandra]
    git = "https://github.com/tupshin/cassandra-rs"
    
Or just 

    [dependencies]
    cassandra="*"
