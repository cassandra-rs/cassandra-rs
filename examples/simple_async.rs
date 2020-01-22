use cassandra_cpp::*;
#[tokio::main]
async fn main() {
    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");
    let col_name = "keyspace_name";

    let contact_points = "127.0.0.1";

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points).unwrap();
    cluster.set_load_balance_round_robin();

    match cluster.connect_async().await {
        Ok(ref mut session) => {
            let result = session.execute(&query).await.unwrap();
            println!("{}", result);
            for row in result.iter() {
                let col: String = row.get_by_name(col_name).unwrap();
                println!("ks name = {}", col);
            }
        }
        err => println!("{:?}", err),
    }
}
