#![allow(unstable)]

extern crate cql_ffi;

use std::ffi::CString;
use std::slice;

use cql_ffi::*;

use std::num::ToPrimitive;

static NUM_CONCURRENT_REQUESTS:usize = 1000;

fn print_error(future:&mut CassFuture) {unsafe{
    let message = cass_future_error_message(future);
    let message = slice::from_raw_buf(&message.data,message.length as usize);
    println!("Error: {:?}", message);
}}

fn create_cluster() -> *mut CassCluster {unsafe{
    let cluster = cass_cluster_new();
    cass_cluster_set_contact_points(cluster, str2ref("127.0.0.1,127.0.0.2,127.0.0.3"));
    cluster 
}}

fn connect_session(session:&mut CassSession, cluster:&mut CassCluster) -> CassError {unsafe{
    let future = &mut *cass_session_connect(session, cluster);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CassError::CASS_OK => {},
        _=> print_error(future)
    }
    cass_future_free(future);
    rc
}}

fn execute_query(session: &mut CassSession, query: &str) -> CassError {unsafe{
    let statement = cass_statement_new(cass_string_init(cass_string_init(CString::from_slice(query.as_bytes()).as_ptr()).data), 0);
    let future = &mut *cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CassError::CASS_OK => {},
        _ => print_error(future)
    }
    cass_future_free(future);
    cass_statement_free(statement);
    return rc;
}}

fn insert_into_async(session: &mut CassSession, key:&str) {unsafe{
    let query=str2cass_string("INSERT INTO examples.async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
    let mut futures = Vec::<*mut CassFuture>::new();
    for i in (0..NUM_CONCURRENT_REQUESTS) {
        let statement = cass_statement_new(query, 6);
        let key = format!("{}{}", key, i).as_ptr() as *const i8;
        cass_statement_bind_string(statement, 0, cass_string_init(key));
        cass_statement_bind_bool(statement, 1, if i % 2 == 0 {cass_true} else {cass_false});
        cass_statement_bind_float(statement, 2, i.to_f32().unwrap() / 2.0f32);
        cass_statement_bind_double(statement, 3, i.to_f64().unwrap() / 200.0);
        cass_statement_bind_int32(statement, 4, i.to_i32().unwrap() * 10);
        cass_statement_bind_int64(statement, 5, i.to_i64().unwrap() * 100);
        futures.push(cass_session_execute(session, statement));
        cass_statement_free(statement);
    }
    for mut future in futures.iter_mut() {
        cass_future_wait(*future);
        let rc = cass_future_error_code(*future);
        if rc != CassError::CASS_OK {
            print_error(&mut**future);
        }
        cass_future_free(*future);
    }
}}

pub fn main() {unsafe{
    let (cluster,session) = (create_cluster(),cass_session_new());
    match connect_session(&mut*session, &mut*cluster) {
        CassError::CASS_OK => {},
        _ => {
            cass_cluster_free(cluster);
            cass_session_free(session);
            panic!("couldn't connect");
        }
    }
    execute_query(&mut*session, "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };");
    execute_query(&mut*session, "CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));");
    execute_query(&mut*session, "USE examples");
    insert_into_async(&mut*session, "test");
    let close_future = cass_session_close(session);
    cass_future_wait(close_future);
    cass_future_free(close_future);
    cass_cluster_free(cluster);
    cass_session_free(session);
}}
