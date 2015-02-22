extern crate cql_ffi;

use cql_ffi::*;

struct Pair<'a> {
    key:&'a str,
    value:i32
}
const CONTACT_POINTS:&'static str = "127.0.0.1";

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.maps (key text, items map<text, int>, PRIMARY KEY (key))";
static SELECT_QUERY:&'static str = "SELECT items FROM examples.maps WHERE key = ?";
static INSERT_QUERY:&'static str ="INSERT INTO examples.maps (key, items) VALUES (?, ?);";

fn insert_into_maps(mut session:CassSession, key:&str, items:Vec<Pair>) -> Result<CassSession,CassError> {
    let statement = CassStatement::new(INSERT_QUERY, 2);
    statement.bind_string(0, key).unwrap();

    
    let mut map = CassMap::new(5);
    for item in items.iter() {
        map.append_string(item.key).unwrap();
        map.append_int32(item.value).unwrap();
    }
    try!(statement.bind_map(1, map));
    try!(session.execute_statement(&statement).wait());
    Ok(session)
}

fn select_from_maps(mut session: CassSession, key:&str) -> Result<CassSession,CassError> {
    let statement = CassStatement::new(SELECT_QUERY, 1);
    let statement = statement.bind_string(0, key).unwrap();
    match session.execute_statement(&statement).wait() {
        Ok(result) => {
            for row in result.iter() {
                let iterator = row.get_column(0).map_iter().unwrap();
                for pair in iterator {
                    println!("item: '{:?}'", try!(pair.get_int32()));
                }
            }
            Ok(session)    
        },
        Err(err) => Err(err),
    }
}

fn main() {
    let cluster = &CassCluster::new()
                        .set_contact_points(CONTACT_POINTS.as_contact_points()).unwrap()
                        .set_load_balance_round_robin().unwrap();
    let items:Vec<Pair> = vec!(
        Pair{key:"apple", value:1 },
        Pair{key:"orange", value:2 },
        Pair{key:"banana", value:3 },
        Pair{key:"mango", value:4 }
    );
    let session_future = CassSession::new().connect(cluster).wait();
    match session_future {
        Ok(mut session) => {
            session.execute(CREATE_KEYSPACE,0).wait().unwrap();
            session.execute(CREATE_TABLE,0).wait().unwrap();
            let session = insert_into_maps(session, "test", items).unwrap();
            let mut session = select_from_maps(session, "test").unwrap();
            session.close();
        },
        Err(err) => panic!("agh:{:?}",err)
    }
}
