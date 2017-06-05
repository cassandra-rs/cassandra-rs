extern crate cassandra;
use cassandra::*;
use std::str::FromStr;

fn main() {

    let keyspace = "system_schema";
    let table = "tables";

    let contact_points = ContactPoints::from_str("127.0.0.1").unwrap();

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points).unwrap();
    cluster.set_load_balance_round_robin();
    match cluster.connect() {
        Ok(ref mut session) => {
            let query = format!("select column_name, type from system_schema.columns where keyspace_name = '{}' and \
                                 table_name = '{}'",
                                keyspace,
                                table);
            let schema_query = Statement::new(&query, 0);
            for _ in 0..1000 {
                let result = session.execute(&schema_query).wait().unwrap();
                for row in result {
                    let name: String = row.get_col_by_name("column_name").unwrap();
                    let ftype: String = row.get_col_by_name("type").unwrap();

                    println!("{} {}", name, ftype);
                }
            }
        }
        Err(_) => unimplemented!(),
    }
}
