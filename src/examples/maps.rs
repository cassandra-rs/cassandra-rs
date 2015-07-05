extern crate cassandra;

use cassandra::*;

struct Pair<'a> {
    key:&'a str,
    value:i32
}
const CONTACT_POINTS:&'static str = "127.0.0.1";

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.maps (key text, items map<text, int>, PRIMARY KEY (key))";
static SELECT_QUERY:&'static str = "SELECT items FROM examples.maps WHERE key = ?";
static INSERT_QUERY:&'static str ="INSERT INTO examples.maps (key, items) VALUES (?, ?);";

fn insert_into_maps(session:&mut CassSession, key:&str, items:Vec<Pair>) -> Result<(),CassError> {
    let mut statement = CassStatement::new(INSERT_QUERY, 2);
    statement.bind_string(0, key).unwrap();

    let mut map = CassMap::new(5);
    for item in items {
        map.append_string(item.key).unwrap();
        map.append_int32(item.value).unwrap();
    }
    try!(statement.bind_map(1, map));
    try!(session.execute_statement(&statement).wait());
    Ok(())
}

fn select_from_maps(session:&mut CassSession, key:&str) -> Result<(),CassError> {
    let mut statement = CassStatement::new(SELECT_QUERY, 1);
    try!(statement.bind_string(0, key));
    let result = try!(session.execute_statement(&statement).wait());
    //println!("{:?}", result);
    for row in result.iter() {
        let column = row.get_column(0).unwrap(); //FIXME
        let items_iterator:MapIterator = column.map_iter().unwrap();
        for item in items_iterator {
            println!("item: {:?}", item);
        }
    }
    Ok(())
}

fn main() {
    match foo() {
        Ok(()) => {},
        Err(err) => println!("Error: {:?}",err)
    }
}

fn foo() -> Result<(),CassError> {
    let mut cluster = CassCluster::new();
    cluster
        .set_contact_points(CONTACT_POINTS).unwrap()
        .set_load_balance_round_robin().unwrap();

    let items:Vec<Pair> = vec!(
        Pair{key:"apple", value:1 },
        Pair{key:"orange", value:2 },
        Pair{key:"banana", value:3 },
        Pair{key:"mango", value:4 }
    );
    let session_future = CassSession::new().connect(&cluster).wait();
    match session_future {
        Ok(mut session) => {
            try!(session.execute(CREATE_KEYSPACE,0).wait());
            try!(session.execute(CREATE_TABLE,0).wait());
            try!(insert_into_maps(&mut session, "test", items));
            try!(select_from_maps(&mut session, "test"));
            session.close();
            Ok(())
        },
        Err(err) => Err(err)
    }

}
