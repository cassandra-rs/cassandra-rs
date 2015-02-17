extern crate cql_ffi;

use cql_ffi::*;

struct Pair<'a> {
    key:&'a str,
    value:i32
}

fn create_cluster() -> Result<CassCluster,CassError> {unsafe{
    let cluster = CassCluster::new();
    cluster.set_contact_points("127.0.0.1")
}}

unsafe fn insert_into_maps(session:&mut CassSession, key:&str, items:Vec<Pair>) -> Result<CassFuture,CassError> {
    let query = "INSERT INTO examples.maps (key, items) VALUES (?, ?);";
    let statement = CassStatement::new(query, 2);
    let _ = statement.bind_string(0, str2cass_string(key));
    let collection = &mut CassCollection::new(CassCollectionType::MAP, 5);
    for item in items.iter() {
        let _ = collection.append_string(str2cass_string(item.key));
        let _ = collection.append_int32(item.value);
    }
    let _ = statement.bind_collection(1, collection);
    session.execute_statement(&statement).wait()
}

unsafe fn select_from_maps(session: &mut CassSession, key:&str) {
    let query = "SELECT items FROM examples.maps WHERE key = ?";
    let statement = CassStatement::new(query, 1);
    let _ = statement.bind_string(0, str2cass_string(key));
    match session.execute_statement(&statement).wait() {
        Err(err) => panic!("{:?}",err.desc()),
        Ok(mut future) => {
            let result = future.get_result();
            if result.row_count() > 0 {
            let row = result.first_row();
            let mut iterator = row.get_column(0).map_iterator();
                let mut value = 0;
                while iterator.next() {
                    match iterator.get_map_key().get_string() {
                        Ok(key) => {
                            let _ = iterator.get_map_value().get_int32(&mut value);
                            println!("item: '{:?}' : {:?}", ToString::to_string(&key), value);
                        }
                        Err(err) => println!("{:?}",err)
                    }
                }
            }
        }
    }
}

fn main() {unsafe{
    let cluster = &mut create_cluster().unwrap();
    let session = &mut CassSession::new();
    let items:Vec<Pair> = vec!(
        Pair{key:"apple", value:1 },
        Pair{key:"orange", value:2 },
        Pair{key:"banana", value:3 },
        Pair{key:"mango", value:4 }
    );
    let _ = session.connect(cluster);    

    let _ = session.execute_statement(&CassStatement::new("CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };",0));
    let _ = session.execute_statement(&CassStatement::new("CREATE TABLE examples.maps (key text, items map<text, int>, PRIMARY KEY (key))",0));
    let _ = insert_into_maps(session, "test", items);
    select_from_maps(session, "test");
    let close_future = session.close();
    close_future.wait().unwrap();
}}
