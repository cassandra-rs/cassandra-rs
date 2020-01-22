mod help;

use cassandra_cpp::*;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pair {
    key: String,
    value: String,
}

fn insert_into_batch_with_prepared(
    session: &Session,
    pairs: &Vec<Pair>,
) -> Result<PreparedStatement> {
    let insert_query = "INSERT INTO examples.pairs (key, value) VALUES (?, ?)";
    let prepared = session.prepare(insert_query).unwrap().wait().unwrap();
    let mut batch = Batch::new(BatchType::LOGGED);
    batch.set_consistency(Consistency::ONE)?;
    for pair in pairs {
        let mut statement = prepared.bind();
        statement.bind(0, pair.key.as_ref() as &str)?;
        statement.bind(1, pair.value.as_ref() as &str)?;
        match batch.add_statement(&statement) {
            Ok(_) => {}
            Err(err) => panic!("{:?}", err),
        }
    }
    session.execute_batch(batch).wait()?;
    Ok(prepared)
}

fn retrieve_batch(session: &Session) -> Vec<Pair> {
    let select_query = stmt!("SELECT * from examples.pairs");

    let result = session.execute(&select_query).wait().unwrap();
    let v: Vec<Pair> = result
        .iter()
        .map(|r| Pair {
            key: r.get(0).expect("Key"),
            value: r.get(1).expect("Value"),
        })
        .collect();
    v
}

#[test]
fn test_batch() {
    let create_table =
        "CREATE TABLE IF NOT EXISTS examples.pairs (key text, value text, PRIMARY KEY (key));";

    let pairs = vec![
        Pair {
            key: "a".to_string(),
            value: "1".to_string(),
        },
        Pair {
            key: "b".to_string(),
            value: "2".to_string(),
        },
        Pair {
            key: "c".to_string(),
            value: "3".to_string(),
        },
        Pair {
            key: "d".to_string(),
            value: "4".to_string(),
        },
    ];

    let session = help::create_test_session();
    help::create_example_keyspace(&session);

    session.execute(&stmt!(create_table)).wait().unwrap();
    insert_into_batch_with_prepared(&session, &pairs).unwrap();
    let result = retrieve_batch(&session);

    let set0: HashSet<_> = pairs.iter().collect();
    let set1: HashSet<_> = result.iter().collect();
    assert_eq!(set0, set1, "expected {:?} but got {:?}", &pairs, &result);
}
