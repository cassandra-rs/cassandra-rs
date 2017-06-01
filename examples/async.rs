#[macro_use(stmt)]
extern crate cassandra;
use cassandra::*;
use errors::*;
extern crate num;

use std::str::FromStr;



static NUM_CONCURRENT_REQUESTS: usize = 1000;

fn insert_into_async(session: &mut Session, key: String) -> Result<Vec<ResultFuture>> {
    let mut futures = Vec::<ResultFuture>::new();
    for i in 0..NUM_CONCURRENT_REQUESTS {
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

pub fn main() {
    let mut cluster = Cluster::default();
    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();
    match cluster.connect() {
        Ok(ref mut session) => {
            session.execute(&stmt!("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                 \'SimpleStrategy\', \'replication_factor\': \'1\' };"))
                .wait()
                .unwrap();
            session.execute(&stmt!("CREATE TABLE IF NOT EXISTS examples.async(key text, bln boolean, flt float, dbl \
                                 double, i32 int, i64 bigint, PRIMARY KEY (key));"))
                .wait()
                .unwrap();
            session.execute(&stmt!("USE examples")).wait().unwrap();
            let futures = insert_into_async(session, "test".to_owned()).unwrap();
            for mut future in futures {
                println!("insert result={:?}", future.wait());
            }
        }
        Err(err) => panic!("couldn't connect: {:?}", err),
    }
}
