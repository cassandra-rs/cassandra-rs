extern crate cassandra;
use cassandra::*;

static QUERY:&'static str = "SELECT keyspace_name FROM system.schema_keyspaces;";
static COL_NAME:&'static str = "keyspace_name";
static CONTACT_POINTS:&'static str = "127.0.0.1";

fn main() {
    let mut cluster = CassCluster::new();
    cluster
        .set_contact_points(CONTACT_POINTS).unwrap()
        .set_load_balance_round_robin().unwrap();
    let session = CassSession::new().connect(&cluster).wait().unwrap();
    let result = session.execute(QUERY, 0).wait().unwrap();
    println!("{}",result);
    for row in result.iter() {
        println!("ks name = {}", row.get_column_by_name(COL_NAME));
    }
    session.close().wait().unwrap();
}
