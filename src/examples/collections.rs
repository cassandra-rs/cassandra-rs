extern crate cassandra;

use cassandra::*;

static INSERT_QUERY:&'static str = "INSERT INTO examples.collections (key, items) VALUES (?, ?);";
static SELECT_QUERY:&'static str = "SELECT items FROM examples.collections WHERE key = ?";
static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";
static CREATE_TABLE:&'static str = "CREATE TABLE examples.collections (key text, items set<text>, PRIMARY KEY (key))";

fn insert_into_collections(session:&mut CassSession, key:&str, items:Vec<String>) -> Result<CassResult,CassError> {
    let mut statement = CassStatement::new(INSERT_QUERY, 2);
    try!(statement.bind_string(0, key));
    let mut set = CassSet::new(2);
    for item in items.iter() {
        try!(set.append_string(item));
    }
    try!(statement.bind_set(1, set));
    session.execute_statement(&statement).wait()
}

fn select_from_collections(session:&mut CassSession, key:&str) -> Result<(),CassError> {
    let mut statement = CassStatement::new(SELECT_QUERY, 1);
    try!(statement.bind_string(0, key));
    let result = try!(session.execute_statement(&statement).wait());
    println!("{:?}", result);
    for row in result.iter() {
        let column = row.get_column(0);
        let items_iterator:SetIterator = try!(try!(column).set_iter());
        for item in items_iterator {
            println!("item: {:?}", item);
        }
    }
    Ok(())
}

fn main() {
  
    let cluster = &CassCluster::new().set_contact_points("127.0.0.1").unwrap();
    let session = &mut CassSession::new().connect(cluster).wait().unwrap();
    
    let items = vec!("apple".to_string(), "orange".to_string(), "banana".to_string(), "mango".to_string());
    session.execute(CREATE_KEYSPACE,0);
    session.execute(CREATE_TABLE,0);
    insert_into_collections(session, "test", items).unwrap();
    select_from_collections(session, "test").unwrap();
}

