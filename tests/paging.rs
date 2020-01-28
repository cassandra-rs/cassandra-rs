mod help;

use cassandra_cpp::*;

static NUM_CONCURRENT_REQUESTS: usize = 100;
const PAGE_SIZE: i32 = 10;

static CREATE_TABLE: &'static str =
    "CREATE TABLE IF NOT EXISTS examples.paging (key ascii, value text, PRIMARY KEY \
     (key));";
static SELECT_QUERY: &'static str = "SELECT * FROM paging";
static INSERT_QUERY: &'static str = "INSERT INTO paging (key, value) VALUES (?, ?);";

// FIXME uuids not yet working
fn insert_into_paging(
    session: &Session, /* , uuid_gen:&mut UuidGen */
) -> Result<Vec<Option<CassFuture<CassResult>>>> {
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

fn select_from_paging(session: &Session) -> Result<Vec<(String, String)>> {
    let mut has_more_pages = true;
    let mut statement = Statement::new(SELECT_QUERY, 0);
    statement.set_paging_size(PAGE_SIZE).unwrap();
    let mut res = vec![];

    // FIXME must understand statement lifetime better for paging
    while has_more_pages {
        let result = session.execute(&statement).wait()?;
        println!("{:?}", result);
        for row in result.iter() {
            match row.get_column(0)?.get_string() {
                Ok(key) => {
                    let key_str = key.to_string();
                    let value = row.get_column(1)?.get_string()?;
                    println!("key: '{:?}' value: '{:?}'", &key_str, &value);
                    res.push((key_str, value));
                }
                Err(err) => panic!(err),
            }
        }
        has_more_pages = result.has_more_pages();
        if has_more_pages {
            statement.set_paging_state(result)?;
        }
    }
    Ok(res)
}

#[test]
fn test_paging() {
    // let uuid_gen = &mut UuidGen::new();

    let session = help::create_test_session();
    help::create_example_keyspace(&session);

    session.execute(&stmt!(CREATE_TABLE)).wait().unwrap();
    session.execute(&stmt!("USE examples")).wait().unwrap();
    let results = insert_into_paging(&session /* , uuid_gen */).unwrap();
    for result in results {
        print!("{:?}", result.unwrap().wait().unwrap());
    }
    let mut results: Vec<(String, String)> = select_from_paging(&session).unwrap();
    results.sort_by_key(|kv| kv.0.clone());
    results.dedup_by_key(|kv| kv.0.clone());
    assert_eq!(results.len(), NUM_CONCURRENT_REQUESTS);
}
