#![feature(core)]

extern crate cql_ffi;

use cql_ffi::*;

use std::num::ToPrimitive;

static NUM_CONCURRENT_REQUESTS:usize = 1000;

fn print_error(future:&mut CassFuture) {unsafe{
    let message = future.error_message();
    let message = String::from_raw_parts(message.0.data as *mut u8,message.0.length as usize, message.0.length as usize);    
    println!("Error: {:?}", message);
}}

fn create_cluster() -> CassCluster {unsafe{
    let mut cluster = CassCluster::new();
    let _ = cluster.set_contact_points("127.0.0.1".as_ptr() as *const i8);
    cluster 
}}

fn connect_session(session:&mut CassSession, cluster:&mut CassCluster) -> Result<(),CassError> {unsafe{
    let mut future = session.connect(cluster);
    future.wait();
    let err = future.error_code();
    future.free();
    err
}}

fn execute_query(session: &mut CassSession, query: &str) -> Result<(),CassError> {unsafe{
    let query=str2cass_string(query);
    println!("{:?}",query.0.length);
   // println!("{:?}", query);
    let statement = CassStatement::new(query, 0);
    let mut future = session.execute(statement);
    future.wait();
    let _ = future.error_code();
    let result = future.error_code();
    future.free();
    result
}}

fn insert_into_async(session: &mut CassSession, key:&str) {unsafe{
    let query=str2cass_string("INSERT INTO examples.async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
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
        futures.push(session.execute(statement));
        statement.free();
    }
    for mut future in futures.iter_mut() {
        future.wait();
        match future.error_code() {
            Ok(()) => {},
            Err(_) => print_error(future),
        }
        future.free();
    }
}}

pub fn main() {unsafe{
    let (ref mut cluster,ref mut session) = (create_cluster(),CassSession::new());
    match connect_session(session, cluster) {
        Ok(()) => {},
        _ => {
            cluster.free();
            session.free();
            panic!("couldn't connect");
        }
    }
    let _ = execute_query(session, "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };");
    let _ = execute_query(session, "CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));");
    let _ = execute_query(session, "USE examples");
    insert_into_async(session, "test");
    let mut close_future = session.close();
    close_future.wait();
    close_future.free();
    cluster.free();
    session.free();
}}
