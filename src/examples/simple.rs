#[macro_use(stmt)]
extern crate cassandra;
use cassandra::*;
use std::str::FromStr;


fn main() {
    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");
    let col_name = "keyspace_name";

    let contact_points = ContactPoints::from_str("127.0.0.1").unwrap();

    let mut cluster = Cluster::new();
    cluster.set_contact_points(contact_points).unwrap();
    cluster.set_load_balance_round_robin();

    match cluster.connect() {
        Ok(ref mut session) => {
            let result = session.execute(&query).wait().unwrap();
            println!("{}", result);
            for row in result.iter() {
                println!("ks name = {}", row.get_column_by_name(col_name));
            }
        }
        err => println!("{:?}", err),
    }
}
