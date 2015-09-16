# cassandra-rs

This is a (hopefully) maintained rust project that unsafely 
exposes the cpp driver at https://github.com/datastax/cpp-driver/
in a somewhat-sane crate. 

You can use it from cargo with

    [dependencies.cassandra]
    git = "https://github.com/tupshin/cassandra-rs"
    
Or just 

    [dependencies]
    cassandra="*"
