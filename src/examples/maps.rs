extern crate cql_ffi;

use std::str::FromStr;

use cql_ffi::*;

struct Pair<'a> {
    key:&'a str,
    value:i32
}

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.maps (key text, items map<text, int>, PRIMARY KEY (key))";
static SELECT_QUERY:&'static str = "SELECT items FROM examples.maps WHERE key = ?";
static INSERT_QUERY:&'static str ="INSERT INTO examples.maps (key, items) VALUES (?, ?);";

unsafe fn insert_into_maps(session:&mut CassSession, key:&str, items:Vec<Pair>) -> Result<CassResult,CassError> {
    let query = INSERT_QUERY;
    let statement = CassStatement::new(query, 2);
    statement.bind_string(0, key).unwrap();
    let mut collection = CassCollection::new(CassCollectionType::MAP, 5);
    for item in items.iter() {
        collection.append_string(FromStr::from_str(item.key).unwrap()).unwrap();
        collection.append_int32(item.value).unwrap();
    }
    let statement = statement.bind_collection(1, collection);
    session.execute_statement(&statement.unwrap()).wait()
}

unsafe fn select_from_maps(session: &mut CassSession, key:&str) {
    let query = SELECT_QUERY;
    let statement = CassStatement::new(query, 1);
    statement.bind_string(0, key);
    match session.execute_statement(&statement).wait() {
        Err(err) => panic!("{:?}",err.desc()),
        Ok(result) => {
            if result.row_count() > 0 {
            let row = result.first_row();
            let mut iterator = row.get_column(0).map_iterator();
                while iterator.next() {
                    match iterator.get_pair() {
                        Ok(pair) => {
                            println!("item: '{:?}' : {:?}", pair.0.get_int32(), pair.1);
                        }
                        Err(err) => println!("{:?}",err)
                    }
                }
            }
        }
    }
}

fn main() {unsafe{
    let cluster = CassCluster::new().set_contact_points("127.0.0.1,127.0.0.2").unwrap();
    let items:Vec<Pair> = vec!(
        Pair{key:"apple", value:1 },
        Pair{key:"orange", value:2 },
        Pair{key:"banana", value:3 },
        Pair{key:"mango", value:4 }
    );
    let mut session = CassSession::new().connect(&cluster).wait().unwrap();    

    let _ = session.execute(CREATE_KEYSPACE,0).wait().unwrap();
    let _ = session.execute(CREATE_TABLE,0).wait().unwrap();
    insert_into_maps(&mut session, "test", items).unwrap();
    select_from_maps(&mut session, "test");
    session.close().wait().unwrap();
}}
