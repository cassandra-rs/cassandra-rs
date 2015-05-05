extern crate cql_ffi;

use cql_ffi::*;

static NUM_CONCURRENT_REQUESTS:usize = 1000;
static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";

fn insert_into_async(session: &mut CassSession, key:String) {
    let query="INSERT INTO examples.async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
    let mut futures = Vec::<ResultFuture>::new();
    for i in (0..NUM_CONCURRENT_REQUESTS) {
        let statement = &CassStatement::new(query, 6);
        let key = key.clone() + &i.to_string();
        CassStatement::new(query, 6)
            .bind_string(0, &key).unwrap()
            .bind_bool(1, if i % 2 == 0 {true} else {false}).unwrap()
            .bind_float(2, i as f32 / 2.0f32).unwrap()
            .bind_double(3, i as f64 / 200.0).unwrap()
            .bind_int32(4, i as i32 * 10).unwrap()
            .bind_int64(5, i as i64* 100).unwrap();
        let future = session.execute_statement(statement);
        futures.push(future);
    }
    //~ for mut result in futures.iter()  {
        //FIXME
       //~ result.wait();
    //~ }
}

pub fn main() {
    let cluster = &mut CassCluster::new().set_contact_points("127.0.0.1").unwrap();
    match CassSession::new().connect(cluster).wait() {
        Ok(mut session) => {
            let _ = session.execute(CREATE_KEYSPACE,0).wait().unwrap();
            let _ = session.execute(CREATE_TABLE,0).wait().unwrap();
            let _ = session.execute("USE examples",0);
            insert_into_async(&mut session, "test".to_string());
            session.close().wait().unwrap();
        },
        _ => {
            panic!("couldn't connect");
        }
    }
}
