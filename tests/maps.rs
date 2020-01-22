mod help;

use cassandra_cpp::*;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pair {
    key: String,
    value: i32,
}

static CREATE_TABLE: &'static str =
    "CREATE TABLE IF NOT EXISTS examples.maps (key text, items map<text, int>, \
     PRIMARY KEY (key))";
static SELECT_QUERY: &'static str = "SELECT items FROM examples.maps WHERE key = ?";

fn insert_into_maps(session: &Session, key: &str, items: &Vec<Pair>) -> Result<()> {
    let mut insert_statement = stmt!("INSERT INTO examples.maps (key, items) VALUES (?, ?);");
    insert_statement.bind(0, key).unwrap();

    let mut map = Map::new(items.len());
    for item in items {
        map.append_string(item.key.as_ref()).unwrap();
        map.append_int32(item.value).unwrap();
    }
    insert_statement.bind(1, map)?;
    session.execute(&insert_statement).wait()?;
    Ok(())
}

fn select_from_maps(session: &Session, key: &str) -> Result<Vec<Pair>> {
    let mut statement = Statement::new(SELECT_QUERY, 1);
    statement.bind(0, key)?;
    let result = session.execute(&statement).wait()?;
    // println!("{:?}", result);
    let mut res = vec![];
    for row in result.iter() {
        let column = row.get_column(0).unwrap(); //FIXME
        let items_iterator: MapIterator = column.get_map().unwrap();
        for item in items_iterator {
            println!("item: {:?}", item);
            res.push(Pair {
                key: item.0.get_string().expect("key").to_string(),
                value: item.1.get_i32().expect("value"),
            })
        }
    }
    Ok(res)
}

#[test]
fn test_maps() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);

    let items: Vec<Pair> = vec![
        Pair {
            key: "apple".to_string(),
            value: 1,
        },
        Pair {
            key: "orange".to_string(),
            value: 2,
        },
        Pair {
            key: "banana".to_string(),
            value: 3,
        },
        Pair {
            key: "mango".to_string(),
            value: 4,
        },
    ];

    session.execute(&stmt!(CREATE_TABLE)).wait().unwrap();
    insert_into_maps(&session, "test", &items).unwrap();
    let result = select_from_maps(&session, "test").unwrap();

    let set0: HashSet<_> = items.iter().collect();
    let set1: HashSet<_> = result.iter().collect();
    assert_eq!(set0, set1, "expected {:?} but got {:?}", &items, &result);
}
