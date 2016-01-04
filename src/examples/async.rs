extern crate num;
extern crate cassandra;
use std::str::FromStr;

use cassandra::*;

static NUM_CONCURRENT_REQUESTS: usize = 100;
static CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'1\' };";
static CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl \
                                     double, i32 int, i64 bigint, PRIMARY KEY (key));";
static INSERT_QUERY: &'static str = "INSERT INTO examples.async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, \
                                     ?);";

fn insert_into_async(session: &mut Session, key: String) -> Result<Vec<ResultFuture>, CassError> {
    let mut futures = Vec::<ResultFuture>::new();
    for i in 0..NUM_CONCURRENT_REQUESTS {
        let key: String = key.clone() + &i.to_string();
        let mut statement = Statement::new(INSERT_QUERY, 6);

        try!(statement.bind_string(0, &key));
        try!(statement.bind_bool(1, i % 2 == 0));
        try!(statement.bind_float(2, i as f32 / 2.0f32));
        try!(statement.bind_double(3, i as f64 / 200.0));
        try!(statement.bind_int32(4, i as i32 * 10));
        try!(statement.bind_int64(5, i as i64 * 100));

        let future = session.execute_statement(&statement);
        futures.push(future);
    }
    Ok(futures)
}

pub fn main() {
    let mut cluster = Cluster::new();
    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();
    match cluster.connect() {
        Ok(mut session) => {
            session.execute(CREATE_KEYSPACE, 0).wait().unwrap();
            session.execute(CREATE_TABLE, 0).wait().unwrap();
            session.execute("USE examples", 0).wait().unwrap();
            let futures = insert_into_async(&mut session, "test".to_owned()).unwrap();
            for mut future in futures {
                println!("insert result={:?}", future.wait());
            }
            session.close().wait().unwrap();
        }
        Err(err) => panic!("couldn't connect: {:?}", err)
    }
}
