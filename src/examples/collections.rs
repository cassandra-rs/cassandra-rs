extern crate cassandra;

use cassandra::*;
use std::str::FromStr;

static INSERT_QUERY: &'static str = "INSERT INTO examples.collections (key, items) VALUES (?, ?);";
static SELECT_QUERY: &'static str = "SELECT items FROM examples.collections WHERE key = ?";
static CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'1\' };";
static CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.collections (key text, items set<text>, \
                                     PRIMARY KEY (key))";

fn insert_into_collections(session: &mut Session, key: &str, items: Vec<&str>) -> Result<CassResult, CassError> {
    let mut statement = Statement::new(INSERT_QUERY, 2);
    try!(statement.bind(0, key));
    let mut set = Set::new(2);
    for item in items {
        try!(set.append_string(&item));
    }
    try!(statement.bind_set(1, set));
    session.execute_statement(&statement).wait()
}

fn select_from_collections(session: &mut Session, key: &str) -> Result<(), CassError> {
    let mut statement = Statement::new(SELECT_QUERY, 1);
    try!(statement.bind(0, key));
    let result = try!(session.execute_statement(&statement).wait());
    println!("{:?}", result);
    for row in result.iter() {
        let column = row.get_column(0);
        let items_iterator: SetIterator = try!(try!(column).set_iter());
        for item in items_iterator {
            println!("item: {:?}", item);
        }
    }
    Ok(())
}

fn main() {
    let items = vec!["apple", "orange", "banana", "mango"];
    let contact_points = ContactPoints::from_str("127.0.0.1").unwrap();
    let mut cluster = Cluster::new();
    cluster.set_contact_points(contact_points).unwrap();

    match cluster.connect() {
        Ok(ref mut session) => {
            session.execute(CREATE_KEYSPACE, 0).wait().unwrap();
            session.execute(CREATE_TABLE, 0).wait().unwrap();
            insert_into_collections(session, "test", items).unwrap();
            select_from_collections(session, "test").unwrap();
        }
        err => println!("{:?}", err),
    }
}
