#![feature(core)]

extern crate cql_ffi;

use std::slice;

use cql_ffi::*;

#[derive(Debug)]
struct Basic {
    bln:u32,
    flt:f32,
    dbl:f64,
    i32:i32,
    i64:i64
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
        CassError::CASS_OK => {},
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
        CassError::CASS_OK => {},
        _ => print_error(future)
    }
    cass_future_free(future);
    cass_statement_free(statement);
    return rc;
}}

fn insert_into_basic(session:&mut CassSession, key:&str, basic:&mut Basic) -> CassError {unsafe{
    let query=str2cass_string("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
    let statement = cass_statement_new(query, 6);
    cass_statement_bind_string(statement, 0, str2cass_string(key));
    cass_statement_bind_bool(statement, 1, basic.bln);
    cass_statement_bind_float(statement, 2, basic.flt);
    cass_statement_bind_double(statement, 3, basic.dbl);
    cass_statement_bind_int32(statement, 4, basic.i32);
    cass_statement_bind_int64(statement, 5, basic.i64);
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CassError::CASS_OK => {
            print_error(&mut*future);
        },
        _=> {panic!()}
    }
    cass_future_free(future);
    cass_statement_free(statement);
    rc
}}



unsafe fn prepare_select_from_basic(session:&mut CassSession) -> Result<&CassPrepared,CassError> {
    let query=str2cass_string("SELECT * FROM examples.basic WHERE key = ?");
    let future = cass_session_prepare(session, query);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);

     match rc {
        CassError::CASS_OK => {
            print_error(&mut*future);
            Ok(&*cass_future_get_prepared(future))
        },
        _=> {
            cass_future_free(future);
            Err(rc)
        }
    }
}

unsafe fn select_from_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:&mut Basic) {
    let statement = cass_prepared_bind(prepared);
    cass_statement_bind_string(statement, 0, str2cass_string(key));
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    if rc != CassError::CASS_OK {
        print_error(&mut*future);
    } else {
        let result = cass_future_get_result(future);
        let iterator = cass_iterator_from_result(result);
        if cass_iterator_next(iterator) > 0 {
            let row = cass_iterator_get_row(iterator);
            cass_value_get_bool(cass_row_get_column(row, 1), &mut basic.bln);
            cass_value_get_double(cass_row_get_column(row, 2), &mut basic.dbl);
            cass_value_get_float(cass_row_get_column(row, 3), &mut basic.flt);
            cass_value_get_int32(cass_row_get_column(row, 4), &mut basic.i32);
            cass_value_get_int64(cass_row_get_column(row, 5), &mut basic.i64);
        }
        cass_result_free(result);
        cass_iterator_free(iterator);
    }
    cass_future_free(future);
    cass_statement_free(statement);
}

fn main() {unsafe{
    let cluster = create_cluster();
    let session = cass_session_new();
    let input = &mut Basic{bln:1, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };
    let output = &mut Basic{bln:0, flt:0f32, dbl:0f64, i32:0, i64:0};
    if connect_session(&mut*session, &mut*cluster) != CassError::CASS_OK {
        cass_cluster_free(cluster);
        cass_session_free(session);
        panic!();
    }
    execute_query(&mut*session, "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
    execute_query(&mut*session, "CREATE TABLE examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));");
    insert_into_basic(&mut*session, "prepared_test", input);
    match prepare_select_from_basic(&mut*session) {
        Ok(prepared) => {
            select_from_basic(&mut*session, prepared, "prepared_test", output);
            println!("i: {:?}, o: {:?}", input,output);
            assert!(input.bln == output.bln);
            assert!(input.flt == output.flt);
            assert!(input.dbl == output.dbl);
            assert!(input.i32 == output.i32);
            assert!(input.i64 == output.i64);
            cass_prepared_free(prepared);
        },
        Err(err) => panic!(err)
    }
    let close_future = cass_session_close(session);
    cass_future_wait(close_future);
    cass_future_free(close_future);
    cass_cluster_free(cluster);
    cass_session_free(session);
}}
