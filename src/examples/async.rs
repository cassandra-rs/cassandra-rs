#![feature(core)]

extern crate cql_ffi;

use cql_ffi::*;

use std::num::ToPrimitive;

static NUM_CONCURRENT_REQUESTS:usize = 1000;

fn insert_into_async(session: &mut CassSession, key:&str) {unsafe{
    let query="INSERT INTO examples.async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
    let mut futures = Vec::<CassFuture>::new();
    for i in (0..NUM_CONCURRENT_REQUESTS) {
        let statement = CassStatement::new(query, 6);
        let key = format!("{}{}", key, i).as_ptr() as *const i8;
        let _ = statement.bind_string(0, CassString::init(key));
        let _ = statement.bind_bool(1, if i % 2 == 0 {cass_true} else {cass_false});
        let _ = statement.bind_float(2, i.to_f32().unwrap() / 2.0f32);
        let _ = statement.bind_double(3, i.to_f64().unwrap() / 200.0);
        let _ = statement.bind_int32(4, i.to_i32().unwrap() * 10);
        let _ = statement.bind_int64(5, i.to_i64().unwrap() * 100);
        let future = session.execute_statement(&statement);
        futures.push(future);
    }
    //~ for future in futures.iter() {
        //FIXME
        //future.wait().unwrap().get_result();
    //~ }
}}

pub fn main() {unsafe{
    let cluster = &mut CassCluster::new().set_contact_points("127.0.0.1").unwrap();
    let mut session = CassSession::new();
    match session.connect(cluster).wait() {
        Ok(_) => {
            let _ = session.execute_statement(&CassStatement::new("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };",0));
            let _ = session.execute_statement(&CassStatement::new("CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));",0));
            let _ = session.execute_statement(&CassStatement::new("USE examples",0));
            insert_into_async(&mut session, "test");
            session.close().wait().unwrap();
        },
        _ => {
            panic!("couldn't connect");
        }
    }
}}
