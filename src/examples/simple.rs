extern crate cassandra;
use cassandra::*;
use std::str::FromStr;

static QUERY: &'static str = "SELECT keyspace_name FROM system_schema.keyspaces;";
static COL_NAME: &'static str = "keyspace_name";

fn main() {
    let mut cluster = Cluster::new();
    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();
    cluster.set_load_balance_round_robin().unwrap();
    let session = cluster.connect().unwrap();
    let result = session.execute(QUERY, 0).wait().unwrap();
    println!("{}", result);
    for row in result.iter() {
        println!("ks name = {}", row.get_column_by_name(COL_NAME));
    }
    session.close().wait().unwrap();
}
