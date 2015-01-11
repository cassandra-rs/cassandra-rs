#![allow(unstable)]

extern crate cql_ffi;

use std::slice;

use cql_ffi::*;

#[derive(Show)]
struct Basic {
    bln:cass_bool_t,
    flt:cass_float_t,
    dbl:cass_double_t,
    i32:cass_int32_t,
    i64:cass_int64_t,
}

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
        CASS_OK => {},
        _=> print_error(future)
    }
    cass_future_free(future);
    rc
}}


fn execute_query(session: &mut CassSession, query: &str) -> CassError {unsafe{
        let query=str2cass_string(query);

    let statement = cass_statement_new(query, 0);
    let future = &mut *cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CASS_OK => {},
        _ => print_error(future)
    }
    cass_future_free(future);
    cass_statement_free(statement);
    return rc;
}}


fn insert_into_basic(session:&mut CassSession, key:&str, basic:&mut Basic) -> CassError {unsafe{
    let query=str2cass_string("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
    let statement = cass_statement_new(query, 6);
    let key = key.as_ptr() as *const i8;
    cass_statement_bind_string(statement, 0, cass_string_init(key));
    cass_statement_bind_bool(statement, 1, basic.bln);
    cass_statement_bind_float(statement, 2, basic.flt);
    cass_statement_bind_double(statement, 3, basic.dbl);
    cass_statement_bind_int32(statement, 4, basic.i32);
    cass_statement_bind_int64(statement, 5, basic.i64);
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CASS_OK => {
            print_error(&mut*future);
        },
        _=> {panic!()}
    }
    cass_future_free(future);
    cass_statement_free(statement);
    rc
}}

fn select_from_basic(session:&mut CassSession, key:&str, basic:&mut Basic) -> CassError {unsafe{
    let query = str2cass_string("SELECT * FROM examples.basic WHERE key = ?");
    let statement = cass_statement_new(query, 1);
    let key = key.as_ptr() as *const i8;
    cass_statement_bind_string(statement, 0, cass_string_init(key));
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CASS_OK => {
            let result = cass_future_get_result(future);
            let iterator = cass_iterator_from_result(result);
            if cass_iterator_next(iterator) > 0 {
                let row = cass_iterator_get_row(iterator);
                let ref mut b_bln = basic.bln;
                let ref mut b_dbl = basic.dbl;
                let ref mut b_flt = basic.flt;
                let ref mut b_i32 = basic.i32;
                let ref mut b_i64 = basic.i64;
                cass_value_get_bool(cass_row_get_column(row, 1), b_bln );
                cass_value_get_double(cass_row_get_column(row, 2), b_dbl);
                cass_value_get_float(cass_row_get_column(row, 3), b_flt);
                cass_value_get_int32(cass_row_get_column(row, 4), b_i32);
                cass_value_get_int64(cass_row_get_column(row, 5), b_i64);
                cass_result_free(result);
                cass_iterator_free(iterator);
            }
        },
        _ => {
            print_error(&mut*future);
            panic!();
        }
    }
    cass_future_free(future);
    cass_statement_free(statement);
    rc
}}

fn main() {unsafe{
    let cluster = create_cluster();
    let session = cass_session_new();
    let input = &mut Basic{bln:cass_true, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };

    match connect_session(&mut*session, &mut*cluster) {
        CASS_OK => {
            let output = &mut Basic{bln:0,flt:0f32,dbl:0f64,i32:0,i64:0};
            execute_query(&mut*session, "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
            execute_query(&mut*session, "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));");
            insert_into_basic(&mut*session, "test", input);
            select_from_basic(&mut*session, "test", output);
            println!("{:?}",input);
            println!("{:?}",output);
            assert!(input.bln == output.bln);
            assert!(input.flt == output.flt);
            assert!(input.dbl == output.dbl);
            assert!(input.i32 == output.i32);
            assert!(input.i64 == output.i64);
            let close_future = cass_session_close(session);
            cass_future_wait(close_future);
            cass_future_free(close_future);
        },
        _ => {}
    }
    cass_cluster_free(cluster);
    cass_session_free(session);
}}
