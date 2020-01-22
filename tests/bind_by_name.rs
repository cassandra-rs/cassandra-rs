mod help;

use cassandra_cpp::*;

#[test]
fn test_bind_by_name() {
    let keyspace = "system_schema";
    let table = "tables";

    let session = help::create_test_session();

    let query = format!(
        "select column_name, type from system_schema.columns where keyspace_name = '{}' and \
         table_name = '{}'",
        keyspace, table
    );
    let schema_query = Statement::new(&query, 0);
    for _ in 0..1000 {
        let result = session.execute(&schema_query).wait().unwrap();
        for row in &result {
            let name: String = row.get_by_name("column_name").unwrap();
            let ftype: String = row.get_by_name("type").unwrap();
            // Actual values are not important; we're checking it doesn't panic or fail to return info.

            println!("{} {}", name, ftype);
        }
    }
}
