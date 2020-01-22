mod help;

use cassandra_cpp::*;
use std::collections::HashSet;

fn insert_into_collections(
    session: &Session,
    key: &str,
    items: &Vec<String>,
) -> Result<CassResult> {
    let mut statement = stmt!("INSERT INTO examples.collections (key, items) VALUES (?, ?);");
    statement.bind(0, key)?;
    let mut set = Set::new(2);
    for item in items {
        set.append_string(item)?;
    }
    statement.bind_set(1, set)?;
    session.execute(&statement).wait()
}

fn insert_null_into_collections(session: &Session, key: &str) -> Result<CassResult> {
    let mut statement = stmt!("INSERT INTO examples.collections (key) VALUES (?);");
    statement.bind(0, key)?;
    session.execute(&statement).wait()
}

fn select_from_collections(session: &Session, key: &str) -> Result<Vec<String>> {
    let mut statement = stmt!("SELECT items FROM examples.collections WHERE key = ?");
    statement.bind(0, key)?;
    let result = session.execute(&statement).wait()?;
    println!("{:?}", result);
    let mut res = vec![];
    for row in result.iter() {
        let column = row.get_column(0);
        let items_iterator: SetIterator = column?.get_set()?;
        for item in items_iterator {
            println!("item: {:?}", item);
            res.push(item.get_string().expect("Should exist").to_string());
        }
    }
    Ok(res)
}

#[test]
fn test_collections() {
    let create_table = stmt!(
        "CREATE TABLE IF NOT EXISTS examples.collections (key text, items set<text>, PRIMARY \
         KEY (key))"
    );

    let items = vec![
        "apple".to_string(),
        "orange".to_string(),
        "banana".to_string(),
        "mango".to_string(),
    ];

    let session = help::create_test_session();
    help::create_example_keyspace(&session);

    session.execute(&create_table).wait().unwrap();
    insert_into_collections(&session, "test", &items).unwrap();
    let result = select_from_collections(&session, "test").unwrap();

    let set0: HashSet<_> = items.iter().collect();
    let set1: HashSet<_> = result.iter().collect();
    assert_eq!(set0, set1, "expected {:?} but got {:?}", &items, &result);

    insert_null_into_collections(&session, "empty").unwrap();
    select_from_collections(&session, "empty").expect_err("Should fail cleanly");
}
