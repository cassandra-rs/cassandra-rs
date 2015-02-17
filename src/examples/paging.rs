extern crate cql_ffi;

use std::mem;

use cql_ffi::*;

static NUM_CONCURRENT_REQUESTS:isize=1000;

fn create_cluster() -> Result<CassCluster,CassError> {unsafe{
    let cluster = CassCluster::new();
    cluster.set_contact_points("127.0.0.1")
}}




unsafe fn insert_into_paging(session:&mut CassSession, uuid_gen:&mut CassUuidGen) -> Result<(),CassError> {
    let query = "INSERT INTO paging (key, value) VALUES (?, ?);";
    let mut futures:Vec<CassFuture> = Vec::with_capacity(NUM_CONCURRENT_REQUESTS as usize);

    for i in (0..NUM_CONCURRENT_REQUESTS) {
        let statement = CassStatement::new(query, 2);
        let key = mem::zeroed();
        uuid_gen.random(key);

        let _ = statement.bind_uuid(0, key);
        println!("{}", i);
        let _ = statement.bind_uuid(1, key);
        let future = session.execute_statement(&statement);
        futures.push(future);
    }
             
    while !futures.is_empty() {
        futures.pop().unwrap().wait().unwrap();
    }
    Ok(())
}

unsafe fn select_from_paging(session:&mut CassSession) {
   let mut has_more_pages = true;
    let query = "SELECT * FROM paging";
    let statement = CassStatement::new(query, 0);
    let _ = statement.set_paging_size(100);
    while has_more_pages {
        let future = session.execute_statement(&statement);
        let mut future = future.wait().unwrap();
        let result = future.get_result();
        let mut iterator = result.iter();
        while iterator.next() {
            let row = iterator.get_row();
            match cassvalue2cassuuid(&row.get_column(0)) {
                Ok(key) => {
                    let key_str = cassuuid2string(key);
                    let value = &row.get_column(1);
                    println!("key: '{:?}' value: '{:?}'\n", key_str, &value.get_string());
                },
                Err(err) => panic!(err)
            }
        }
        if result.has_more_pages() {
            let _ = statement.set_paging_state(&result);
        }
        has_more_pages = result.has_more_pages();
    }
}

fn main() {unsafe{
    let uuid_gen = &mut CassUuidGen::new();
    let  cluster = &mut create_cluster().unwrap();
    let session = &mut CassSession::new();
    let future = session.connect(cluster);
    future.wait().unwrap();
    let _ = session.execute_statement(&CassStatement::new("CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };",0));
    let _ = session.execute_statement(&CassStatement::new("CREATE TABLE examples.paging (key timeuuid, value text, PRIMARY KEY (key));",0));
    let _ = session.execute_statement(&CassStatement::new("USE examples",0));
    let _ = insert_into_paging(session, uuid_gen);
    select_from_paging(session);    
    session.close().wait().unwrap(); 
}}
