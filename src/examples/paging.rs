extern crate cassandra;

use cassandra::*;

static CONTACT_POINTS: &'static str = "127.0.0.1";
static NUM_CONCURRENT_REQUESTS: isize = 100;
static CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'1\' };";
static CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.paging (key ascii, value text, PRIMARY KEY \
                                     (key));";
static SELECT_QUERY: &'static str = "SELECT * FROM paging";
static INSERT_QUERY: &'static str = "INSERT INTO paging (key, value) VALUES (?, ?);";

// FIXME uuids not yet working
fn insert_into_paging(session: &mut Session /* , uuid_gen:&mut UuidGen */)
                      -> Result<Vec<Option<ResultFuture>>, CassandraError> {
    let mut futures = Vec::with_capacity(NUM_CONCURRENT_REQUESTS as usize);
    let mut results = Vec::with_capacity(NUM_CONCURRENT_REQUESTS as usize);

    for i in 0..NUM_CONCURRENT_REQUESTS {
        let key = i.to_string();
        println!("key ={:?}", key);
        let mut statement = Statement::new(INSERT_QUERY, 2);
        try!(statement.bind_string(0, &key));
        try!(statement.bind_string(1, &key));
        let future = session.execute_statement(&statement);
        futures.push(future);
    }

    while !futures.is_empty() {
        results.push(futures.pop());
    }
    Ok(results)
}

fn select_from_paging(session: &mut Session) -> Result<(), CassandraError> {
    let has_more_pages = true;
    let mut statement = Statement::new(SELECT_QUERY, 0);
    statement.set_paging_size(100).unwrap();

    // FIXME must understaned statement lifetime better for paging
    while has_more_pages {
        let result = try!(session.execute_statement(&statement).wait());
        // println!("{:?}", result);
        for row in result.iter() {
            match try!(row.get_column(0)).get_string() {
                Ok(key) => {
                    let key_str = key.to_string();
                    let value = try!(row.get_column(1));
                    print!("key: '{:?}' value: '{:?}'\n",
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

    let mut cluster = Cluster::new();
    cluster.set_contact_points(CONTACT_POINTS)
           .unwrap()
           .set_load_balance_round_robin()
           .unwrap();

    let mut session = Session::new().connect(&cluster).wait().unwrap();

    session.execute(CREATE_KEYSPACE, 0).wait().unwrap();
    session.execute(CREATE_TABLE, 0).wait().unwrap();
    session.execute("USE examples", 0).wait().unwrap();
    let results = insert_into_paging(&mut session /* , uuid_gen */).unwrap();
    for result in results {
        print!("{:?}", result.unwrap().wait().unwrap());
    }
    select_from_paging(&mut session).unwrap();
    session.close().wait().unwrap();
}
