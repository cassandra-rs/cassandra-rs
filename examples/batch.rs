#[macro_use(stmt)]
extern crate cassandra;
use cassandra::*;
use errors::*;
use std::str::FromStr;

struct Pair<'a> {
    key: &'a str,
    value: &'a str,
}


fn insert_into_batch_with_prepared(session: &mut Session, pairs: Vec<Pair>) -> Result<PreparedStatement> {
    let insert_query = "INSERT INTO examples.pairs (key, value) VALUES (?, ?)";
    let prepared = session.prepare(insert_query).unwrap().wait().unwrap();
    let mut batch = Batch::new(CASS_BATCH_TYPE_LOGGED);
    for pair in pairs {
        let mut statement = prepared.bind();
        statement.bind(0, pair.key)?;
        statement.bind(1, pair.value)?;
        match batch.add_statement(&statement) {
            Ok(_) => {}
            Err(err) => panic!("{:?}", err),
        }
    }
    session.execute_batch(batch).wait()?;
    Ok(prepared)
}

pub fn verify_batch(session: &mut Session) {
    let select_query = stmt!("SELECT * from examples.pairs");

    let result = session.execute(&select_query).wait().unwrap();
    println!("{:?}", result);
}

fn main() {
    let create_ks = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \'SimpleStrategy\', \
                     \'replication_factor\': \'1\' };";
    let create_table = "CREATE TABLE IF NOT EXISTS examples.pairs (key text, value text, PRIMARY KEY (key));";

    let pairs = vec!(
        Pair{key:"a", value:"1"},
        Pair{key:"b", value:"2"},
        Pair{key:"c", value:"3"},
        Pair{key:"d", value:"4"},
    );

    let mut cluster = Cluster::default();
    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();

    match cluster.connect() {
        Ok(ref mut session) => {
            session.execute(&stmt!(create_ks)).wait().unwrap();
            session.execute(&stmt!(create_table)).wait().unwrap();
            insert_into_batch_with_prepared(session, pairs).unwrap();
            verify_batch(session);
        }
        err => println!("{:?}", err),
    }
}
