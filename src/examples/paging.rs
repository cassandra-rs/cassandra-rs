#[macro_use(stmt)]
extern crate cassandra;

use cassandra::*;
use errors::*;
use std::str::FromStr;

static NUM_CONCURRENT_REQUESTS: isize = 100;
static CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'1\' };";
static CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.paging (key ascii, value text, PRIMARY KEY \
                                     (key));";
static SELECT_QUERY: &'static str = "SELECT * FROM paging";
static INSERT_QUERY: &'static str = "INSERT INTO paging (key, value) VALUES (?, ?);";

// FIXME uuids not yet working
fn insert_into_paging(session: &mut Session /* , uuid_gen:&mut UuidGen */) -> Result<Vec<Option<ResultFuture>>> {
    let mut futures = Vec::with_capacity(NUM_CONCURRENT_REQUESTS as usize);
    let mut results = Vec::with_capacity(NUM_CONCURRENT_REQUESTS as usize);

    for i in 0..NUM_CONCURRENT_REQUESTS {
        let key: &str = &(i.to_string());
        println!("key ={:?}", key);
        let mut statement = Statement::new(INSERT_QUERY, 2);
        statement.bind(0, key)?;
        statement.bind(1, key)?;
        let future = session.execute(&statement);
        futures.push(future);
    }

    while !futures.is_empty() {
        results.push(futures.pop());
    }
    Ok(results)
}

fn select_from_paging(session: &mut Session) -> Result<()> {
    let has_more_pages = true;
    let mut statement = Statement::new(SELECT_QUERY, 0);
    statement.set_paging_size(100).unwrap();

    // FIXME must understand statement lifetime better for paging
    while has_more_pages {
        let result = session.execute(&statement).wait()?;
        // println!("{:?}", result);
        for row in result.iter() {
            match row.get_column(0)?.get_string() {
                Ok(key) => {
                    let key_str = key.to_string();
                    let value = row.get_column(1)?;
                    println!("key: '{:?}' value: '{:?}'",
                             key_str,
                             &value.get_string().unwrap());
                }
                Err(err) => panic!(err),
            }
        }
        //        if result.has_more_pages() {
        //            try!(statement.set_paging_state(&result));
        //        }
        //        has_more_pages = result.has_more_pages();
    }
    Ok(())
}

fn main() {
    // let uuid_gen = &mut UuidGen::new();

    let mut cluster = Cluster::default();
    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();
    cluster.set_load_balance_round_robin();

    match cluster.connect() {
        Ok(ref mut session) => {
            session.execute(&stmt!(CREATE_KEYSPACE)).wait().unwrap();
            session.execute(&stmt!(CREATE_TABLE)).wait().unwrap();
            session.execute(&stmt!("USE examples")).wait().unwrap();
            let results = insert_into_paging(session /* , uuid_gen */).unwrap();
            for result in results {
                print!("{:?}", result.unwrap().wait().unwrap());
            }
            select_from_paging(session).unwrap();
        }
        err => println!("{:?}", err),
    }
}
