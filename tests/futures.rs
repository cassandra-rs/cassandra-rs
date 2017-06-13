//! Test use of Rust futures (not ResultFuture etc).
//! Based on `async`.

#[macro_use(stmt)]
extern crate cassandra;
extern crate num;
extern crate futures;

use cassandra::*;
use errors::*;
use futures::Future;

mod help;

static NUM_CONCURRENT_REQUESTS: usize = 1000;

fn insert_into_async(session: &Session, key: String, count: usize) -> Result<Vec<ResultFuture>> {
    let mut futures = Vec::<ResultFuture>::new();
    for i in 0..count {
        let key: &str = &(key.clone() + &i.to_string());
        let mut statement = stmt!("INSERT INTO examples.async (key, bln, flt, dbl, i32, i64)
        	VALUES (?, ?, \
                                   ?, ?, ?, ?);");

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

/// Smoke-test the implementation of Rust futures.
#[test]
pub fn test_rust_futures() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);

    let create_table = session.execute(
        &stmt!("CREATE TABLE IF NOT EXISTS examples.async(key text, bln boolean, flt float, dbl \
                         double, i32 int, i64 bigint, PRIMARY KEY (key));"));
    let use_examples = session.execute(&stmt!("USE examples"));
    let inserts = insert_into_async(&session, "test".to_owned(), NUM_CONCURRENT_REQUESTS).unwrap();

    create_table
        .and_then(move |_| use_examples)
        .and_then(move |_| futures::future::join_all(inserts))
        .wait()
        .expect("Should succeed");
}

/// Test early drops of Rust futures (a buggy implementation would lead to SIGSEGV, SIGILL,
/// SIGABRT and friends).
#[test]
pub fn test_early_drop_rust_futures() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);

    let big_future = {
        let create_table = session.execute(
            &stmt!("CREATE TABLE IF NOT EXISTS examples.async(key text, bln boolean, flt float, dbl \
                         double, i32 int, i64 bigint, PRIMARY KEY (key));"));
        let use_examples = session.execute(&stmt!("USE examples"));
        let mut inserts = insert_into_async(&session, "test".to_owned(), NUM_CONCURRENT_REQUESTS).unwrap();
        // Put in reverse, so we poll the later ones (which won't be ready) before the earlier ones
        // (which will be immediately ready)
        inserts.reverse();

        create_table
            .and_then(move |_| use_examples)
            .and_then(move |_| futures::future::select_all(inserts).map_err(|(e, _, _)| e))
            // Wait for one of them to complete, and drop all the other in-flight ones.
    };
    big_future.wait().expect("Should succeed");

    let more_inserts = insert_into_async(&session, "test".to_owned(), NUM_CONCURRENT_REQUESTS).unwrap();

    futures::future::join_all(more_inserts)
        .wait()
        .expect("Should succeed");
}
