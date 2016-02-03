extern crate cassandra;
use std::str::FromStr;
use cassandra::*;

struct Pair<'a> {
    key: &'a str,
    value: i32,
}
static CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'3\' };";
static CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.maps (key text, items map<text, int>, \
                                     PRIMARY KEY (key))";
static SELECT_QUERY: &'static str = "SELECT items FROM examples.maps WHERE key = ?";
static INSERT_QUERY: &'static str = "INSERT INTO examples.maps (key, items) VALUES (?, ?);";

fn insert_into_maps(session: &mut Session, key: &str, items: Vec<Pair>) -> Result<(), CassError> {
    let mut statement = Statement::new(INSERT_QUERY, 2);
    statement.bind(0, key).unwrap();

    let mut map = Map::new(5);
    for item in items {
        map.append_string(item.key).unwrap();
        map.append_int32(item.value).unwrap();
    }
    try!(statement.bind(1, map));
    try!(session.execute_statement(&statement).wait());
    Ok(())
}

fn select_from_maps(session: &mut Session, key: &str) -> Result<(), CassError> {
    let mut statement = Statement::new(SELECT_QUERY, 1);
    try!(statement.bind(0, key));
    let result = try!(session.execute_statement(&statement).wait());
    // println!("{:?}", result);
    for row in result.iter() {
        let column = row.get_column(0).unwrap(); //FIXME
        let items_iterator: MapIterator = column.map_iter().unwrap();
        for item in items_iterator {
            println!("item: {:?}", item);
        }
    }
    Ok(())
}

fn main() {
    match foo() {
        Ok(()) => {}
        Err(err) => println!("Error: {:?}", err),
    }
}

fn foo() -> Result<(), CassError> {
    let mut cluster = Cluster::new();
    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();
    cluster.set_load_balance_round_robin();

    let items: Vec<Pair> = vec![Pair {
                                    key: "apple",
                                    value: 1,
                                },
                                Pair {
                                    key: "orange",
                                    value: 2,
                                },
                                Pair {
                                    key: "banana",
                                    value: 3,
                                },
                                Pair {
                                    key: "mango",
                                    value: 4,
                                }];
    match cluster.connect() {
        Ok(ref mut session) => {
            try!(session.execute(CREATE_KEYSPACE, 0).wait());
            try!(session.execute(CREATE_TABLE, 0).wait());
            try!(insert_into_maps(session, "test", items));
            try!(select_from_maps(session, "test"));
            Ok(())
        }
        Err(err) => Err(err),
    }

}
