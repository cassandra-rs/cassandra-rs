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
        let session = Session::new().connect(&cluster).wait().unwrap();
        let result = session.execute(QUERY, 0).wait().unwrap();
        println!("{}",result);
        for row in result.iter() {
            println!("ks name = {}", row.get_column_by_name(COL_NAME));
        }
        session.close().wait().unwrap();
    }
