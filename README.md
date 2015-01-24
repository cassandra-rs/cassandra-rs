# cql-ffi

This is a (hopefully) maintained rust project that unsafely 
exposes the cpp driver at https://github.com/datastax/cpp-driver/
in a somewhat-sane crate. 

You can use it from cargo with
    [dependencies.cql_ffi]
    git = "https://github.com/tupshin/cql-ffi"

There are still a few bugs, but you will definitely want to use https://github.com/tupshin/cql-ffi-safe instead
