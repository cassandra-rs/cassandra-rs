extern crate cql_ffi;

use cql_ffi::*;

static CONTACT_POINTS:&'static str = "127.0.0.1";
static NUM_CONCURRENT_REQUESTS:isize=10;
static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.paging (key ascii, value text, PRIMARY KEY (key));";
static SELECT_QUERY:&'static str = "SELECT * FROM paging";
static INSERT_QUERY:&'static str = "INSERT INTO paging (key, value) VALUES (?, ?);";

//FIXME uuids not yet working
fn insert_into_paging(session:&mut CassSession, uuid_gen:&mut CassUuidGen) -> Result<Vec<Option<ResultFuture>>,CassError> {
    let mut futures:Vec<ResultFuture> = Vec::with_capacity(NUM_CONCURRENT_REQUESTS as usize);
    let mut results:Vec<Option<ResultFuture>> = Vec::with_capacity(NUM_CONCURRENT_REQUESTS as usize);

    for i in 0..NUM_CONCURRENT_REQUESTS {
        let key = i.to_string();
        println!("key ={:?}", key);
        let statement = &CassStatement::new(INSERT_QUERY, 2);
        try!(statement.bind_string(0, &key[]));
        try!(statement.bind_string(1, &key[]));
        let future = session.execute_statement(&statement);
        futures.push(future);
    }
             
    while !futures.is_empty() {
        results.push(futures.pop());
    }
    Ok(results)
}

fn select_from_paging(session:&mut CassSession) -> Result<(), CassError> {
   let mut has_more_pages = true;
    let statement = CassStatement::new(SELECT_QUERY, 0);
        statement.set_paging_size(100).unwrap();

    while has_more_pages {
        let result = try!(session.execute_statement(&statement).wait());
        for row in result.iter() {
            match row.get_column(0).get_string() {
                Ok(key) => {
                    let key_str = key.to_string();
                    let value = &row.get_column(1);
                    println!("key: '{:?}' value: '{:?}'\n", key_str, &value.get_string());
                },
                Err(err) => panic!(err)
            }
        }
        if result.has_more_pages() {
            try!(statement.set_paging_state(&result));
        }
        has_more_pages = result.has_more_pages();
    }
    Ok(())
}

fn main() {unsafe{
    let uuid_gen = &mut CassUuidGen::new();

    let cluster = &CassCluster::new()
                        .set_contact_points(CONTACT_POINTS).unwrap()
                        .set_load_balance_round_robin().unwrap();

    let mut session = CassSession::new().connect(&cluster).wait().unwrap();
    
    session.execute(CREATE_KEYSPACE,0).wait().unwrap();
    session.execute(CREATE_TABLE,0).wait().unwrap();
    session.execute("USE examples",0).wait().unwrap();
    let results = insert_into_paging(&mut session, uuid_gen).unwrap();
    for result in results {
        println!("result: {:?}", result.unwrap().wait());
    }
    select_from_paging(&mut session).unwrap();    
    session.close().wait().unwrap(); 
}}
