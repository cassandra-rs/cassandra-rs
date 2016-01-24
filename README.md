# cassandra-rs

This is a (hopefully) maintained rust project that unsafely
exposes the cpp driver at https://github.com/datastax/cpp-driver/
in a somewhat-sane crate.

For the wrapper to work, you must first have installed the datastax-cpp driver.

Follow the steps on the cpp driver [docs](https://github.com/datastax/cpp-driver/blob/15215e170810433511c48c304b9e9ca51ff32b2f/topics/building/README.md)  to do so. 

Make sure that the driver (specifically `libcassandra_static.a` and `libcassandra.so`) are in your `/usr/local/lib64/` directory

You can use it from cargo with

    [dependencies.cassandra]
    git = "https://github.com/tupshin/cassandra-rs"

Or just

    [dependencies]
    cassandra="*"


Here's a straightforward example found in simple.rs:


    extern crate cassandra;
    use cassandra::*;

    static QUERY:&'static str = "SELECT keyspace_name FROM system.schema_keyspaces;";
    static COL_NAME:&'static str = "keyspace_name";
    static CONTACT_POINTS:&'static str = "127.0.0.1";

    fn main() {
        let mut cluster = Cluster::new();
        cluster
            .set_contact_points(CONTACT_POINTS).unwrap()
            .set_load_balance_round_robin().unwrap();
        let session = cluster.connect().unwrap();
        let result = session.execute(QUERY, 0).wait().unwrap();
        println!("{}",result);
        for row in result.iter() {
            println!("ks name = {}", row.get_column_by_name(COL_NAME));
        }
        session.close().wait().unwrap();
    }

There's additional examples included with the project in src/examples.
