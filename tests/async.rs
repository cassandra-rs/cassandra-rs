mod help;

use cassandra_cpp::*;

static NUM_CONCURRENT_REQUESTS: usize = 1000;

fn insert_into_async(session: &Session, key: String) -> Result<Vec<CassFuture<CassResult>>> {
    let mut futures = Vec::<CassFuture<CassResult>>::new();
    for i in 0..NUM_CONCURRENT_REQUESTS {
        let key: &str = &(key.clone() + &i.to_string());
        let mut statement = stmt!(
            "INSERT INTO examples.async (key, bln, flt, dbl, i32, i64)
        	VALUES (?, ?, \
                                   ?, ?, ?, ?);"
        );

        statement.bind(0, key)?;
        statement.bind(1, i % 2 == 0)?;
        statement.bind(2, i as f32 / 2.0f32)?;
        statement.bind(3, i as f64 / 200.0)?;
        statement.bind(4, i as i32 * 10)?;
        statement.bind(5, i as i64 * 100)?;

        let future = session.execute(&statement);
        futures.push(future);
    }
    Ok(futures)
}

#[test]
pub fn test_async() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);

    session
        .execute(&stmt!(
            "CREATE TABLE IF NOT EXISTS examples.async(key text, bln boolean, flt float, dbl \
             double, i32 int, i64 bigint, PRIMARY KEY (key));"
        ))
        .wait()
        .unwrap();
    session.execute(&stmt!("USE examples")).wait().unwrap();

    let futures = insert_into_async(&session, "test".to_owned()).unwrap();
    for future in futures {
        let res = future.wait();
        res.expect("Should succeed");
    }
}
