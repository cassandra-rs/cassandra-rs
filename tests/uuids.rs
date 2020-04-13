mod help;

use cassandra_cpp::*;
use std::str::FromStr;

static TRUNCATE_QUERY: &'static str = "TRUNCATE examples.log;";
static INSERT_QUERY: &'static str = "INSERT INTO examples.log (key, time, entry) VALUES (?, ?, ?);";
static SELECT_QUERY: &'static str = "SELECT * FROM examples.log WHERE key = ?";
static CREATE_TABLE: &'static str =
    "CREATE TABLE IF NOT EXISTS examples.log (key text, time timeuuid, entry text, \
     PRIMARY KEY (key, time));";

fn insert_into_log(session: &Session, key: &str, time: Uuid, entry: &str) -> Result<CassResult> {
    let mut statement = stmt!(INSERT_QUERY);
    statement.bind(0, key)?;
    statement.bind(1, time)?;
    statement.bind(2, entry)?;
    let future = session.execute(&statement);
    future.wait()
}

fn select_from_log(session: &Session, key: &str) -> Result<Vec<(Uuid, String)>> {
    let mut statement = stmt!(SELECT_QUERY);
    statement.bind(0, key)?;
    let future = session.execute(&statement);
    let results = future.wait();
    results.map(|r| {
        r.iter()
            .map(|r| {
                let t: Uuid = r.get_column(1).expect("time0").get_uuid().expect("time");
                let e: String = r.get(2).expect("entry");
                (t, e)
            })
            .collect()
    })
}

#[test]
fn test_uuids() {
    let uuid_gen = UuidGen::default();

    let session = help::create_test_session();
    help::create_example_keyspace(&session);

    session.execute(&stmt!(CREATE_TABLE)).wait().unwrap();
    session.execute(&stmt!(TRUNCATE_QUERY)).wait().unwrap();

    println!("uuid_gen = {}", uuid_gen.gen_time());
    insert_into_log(&session, "test", uuid_gen.gen_time(), "Log entry #1").unwrap();
    insert_into_log(&session, "test", uuid_gen.gen_time(), "Log entry #2").unwrap();
    insert_into_log(&session, "test", uuid_gen.gen_time(), "Log entry #3").unwrap();
    insert_into_log(&session, "test", uuid_gen.gen_time(), "Log entry #4").unwrap();
    let mut results = select_from_log(&session, "test").unwrap();
    println!("{:?}", results);

    // Check the resulting UUIDs are in order.
    results.sort_by_key(|ref kv| kv.0);
    assert_eq!(results[0].1, "Log entry #1");
    assert_eq!(results[1].1, "Log entry #2");
    assert_eq!(results[2].1, "Log entry #3");
    assert_eq!(results[3].1, "Log entry #4");

    let mut uniques = results.iter().map(|ref kv| kv.0).collect::<Vec<Uuid>>();
    uniques.dedup();
    assert_eq!(4, uniques.len());
}

#[test]
fn test_uuids_from() {
    const TEST_UUID: &'static str = "a0a1a2a3-a4a5-a6a7-a8a9-aaabacadaeaf";
    let uuid_id_from_str = uuid::Uuid::parse_str(TEST_UUID).unwrap();
    let cass_id_from_str = Uuid::from_str(TEST_UUID).unwrap();

    let cass_id_from_uuid = Uuid::from(uuid_id_from_str);
    let uuid_id_from_cass: uuid::Uuid = cass_id_from_str.into();
    assert_eq!(uuid_id_from_str, uuid_id_from_cass);
    assert_eq!(cass_id_from_str, cass_id_from_uuid);
}
