#[macro_use(stmt)]
extern crate cassandra_cpp;
extern crate futures;

use cassandra_cpp::*;

use futures::Future;


fn main() {
    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");
    let col_name = "keyspace_name";

    let contact_points = "127.0.0.1";

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points).unwrap();
    cluster.set_load_balance_round_robin();

    match cluster.connect() {
        Ok(ref mut session) => {
            let result = session.execute(&query).wait().unwrap();
            println!("{}", result);
            for row in result.iter() {
                let col: String = row.get_by_name(col_name).unwrap();
                println!("ks name = {}", col);
            }
        }
        err => println!("{:?}", err),
    }
}
