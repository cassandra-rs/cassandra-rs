#[macro_use(stmt)]
extern crate cassandra;
use cassandra::*;
use errors::*;

use std::str::FromStr;

static INSERT_QUERY: &'static str = "INSERT INTO examples.log (key, time, entry) VALUES (?, ?, ?);";
static SELECT_QUERY: &'static str = "SELECT * FROM examples.log WHERE key = ?";
static CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'3\' };";
static CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.log (key text, time timeuuid, entry text, \
                                     PRIMARY KEY (key, time));";

fn insert_into_log(session: &mut Session, key: &str, time: Uuid, entry: &str) -> Result<CassResult> {
    let mut statement = stmt!(INSERT_QUERY);
    statement.bind(0, key)?;
    statement.bind(1, time)?;
    statement.bind(2, entry)?;
    let mut future = session.execute(&statement);
    future.wait()
}

fn select_from_log(session: &mut Session, key: &str) -> Result<CassResult> {
    let mut statement = stmt!(SELECT_QUERY);
    statement.bind(0, key)?;
    let mut future = session.execute(&statement);
    let results = future.wait()?;
    Ok(results)
}

fn main() {
    let uuid_gen = UuidGen::default();
    let mut cluster = Cluster::default();
    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();
    match cluster.connect() {
        Ok(ref mut session) => {
            session.execute(&stmt!(CREATE_KEYSPACE)).wait().unwrap();
            session.execute(&stmt!(CREATE_TABLE)).wait().unwrap();
            println!("uuid_gen = {}", uuid_gen.gen_time());
            insert_into_log(session, "test", uuid_gen.gen_time(), "Log entry #1").unwrap();
            insert_into_log(session, "test", uuid_gen.gen_time(), "Log entry #2").unwrap();
            insert_into_log(session, "test", uuid_gen.gen_time(), "Log entry #3").unwrap();
            insert_into_log(session, "test", uuid_gen.gen_time(), "Log entry #4").unwrap();
            let results = select_from_log(session, "test").unwrap();
            println!("{}", results);
        }
        err => println!("{:?}", err),
    }
}
