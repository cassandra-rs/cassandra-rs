#[macro_use(stmt)]
extern crate cassandra;
use cassandra::*;
use errors::*;
use std::str::FromStr;


fn insert_into_collections(session: &mut Session, key: &str, items: Vec<&str>) -> Result<CassResult> {
    let mut statement = stmt!("INSERT INTO examples.collections (key, items) VALUES (?, ?);");
    statement.bind(0, key)?;
    let mut set = Set::new(2);
    for item in items {
        set.append_string(item)?;
    }
    statement.bind_set(1, set)?;
    session.execute(&statement).wait()
}

fn select_from_collections(session: &mut Session, key: &str) -> Result<()> {
    let mut statement = stmt!("SELECT items FROM examples.collections WHERE key = ?");
    statement.bind(0, key)?;
    let result = session.execute(&statement).wait()?;
    println!("{:?}", result);
    for row in result.iter() {
        let column = row.get_column(0);
        let items_iterator: SetIterator = column?.set_iter()?;
        for item in items_iterator {
            println!("item: {:?}", item);
        }
    }
    Ok(())
}

fn main() {
    let create_ks = stmt!("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                           \'SimpleStrategy\', \'replication_factor\': \'1\' };");
    let create_table = stmt!("CREATE TABLE IF NOT EXISTS examples.collections (key text, items set<text>, PRIMARY \
                              KEY (key))");

    let items = vec!["apple", "orange", "banana", "mango"];
    let contact_points = ContactPoints::from_str("127.0.0.1").unwrap();
    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points).unwrap();

    match cluster.connect() {
        Ok(ref mut session) => {
            session.execute(&create_ks).wait().unwrap();
            session.execute(&create_table).wait().unwrap();
            insert_into_collections(session, "test", items).unwrap();
            select_from_collections(session, "test").unwrap();
        }
        err => println!("{:?}", err),
    }
}
