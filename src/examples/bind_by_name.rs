#![allow(unstable)]
extern crate cql_ffi;
use std::ffi::CString;
use std::slice;
use cql_ffi::*;

#[derive(Copy)]
struct Basic {
    bln:cass_bool_t,
    flt:cass_float_t,
    dbl:cass_double_t,
    i32:cass_int32_t,
    i64:cass_int64_t
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
    let statement = cass_statement_new(cass_string_init(cass_string_init(CString::from_slice(query.as_bytes()).as_ptr()).data), 0);
    let future = &mut *cass_session_execute(session, statement);
    cass_future_wait(future);
    let err_code =  cass_future_error_code(future);
    let rc = match err_code {
        CassError::CASS_OK => {CassError::CASS_OK},
        _ => panic!("CassError: {:?}", err_code)
    };
    cass_future_free(future);
    cass_statement_free(statement);
    rc
}}

unsafe fn prepare_query(session:&mut CassSession, query:CassString) -> Result<&CassPrepared,CassError> {

    let future = cass_session_prepare(session, query);
    cass_future_wait(future);
    let rc = match cass_future_error_code(future) {
        CassError::CASS_OK => cass_future_get_prepared(future),
        _ => {print_error(&mut*future);panic!();}
    };
    cass_future_free(future);
    Ok(&*rc)
}

//fixme row key sent is null?
unsafe fn insert_into_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:Basic) -> CassError {
    let statement = cass_prepared_bind(prepared);
    println!("key={:?}",str2cass_string(key));
    cass_statement_bind_string_by_name(statement, "key".as_bytes().as_ptr() as *const i8, str2cass_string(key));
    cass_statement_bind_bool_by_name(statement, "BLN".as_ptr() as *const i8, basic.bln);
    cass_statement_bind_float_by_name(statement, "FLT".as_ptr() as *const i8, basic.flt);
    cass_statement_bind_double_by_name(statement, "\"dbl\"".as_ptr() as *const i8, basic.dbl);
    cass_statement_bind_int32_by_name(statement, "i32".as_ptr() as *const i8, basic.i32);
    cass_statement_bind_int64_by_name(statement, "I64".as_ptr() as *const i8, basic.i64);
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let err_code = cass_future_error_code(future);
    let rc = match err_code {
        CassError::CASS_OK => CassError::CASS_OK,
        _=> panic!("{:?}",err_code)
    };
    cass_future_free(future);
    cass_statement_free(statement);
    rc
}

unsafe fn select_from_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:&mut Basic) -> CassError {
    let statement = cass_prepared_bind(prepared);
    cass_statement_bind_string_by_name(statement, "key".as_ptr() as *const i8, str2cass_string(key));
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
            cass_value_get_bool(cass_row_get_column_by_name(row, "BLN".as_ptr() as *const i8), &mut basic.bln);
            cass_value_get_double(cass_row_get_column_by_name(row, "dbl".as_ptr() as *const i8), &mut basic.dbl);
            cass_value_get_float(cass_row_get_column_by_name(row, "flt".as_ptr() as *const i8), &mut basic.flt);
            cass_value_get_int32(cass_row_get_column_by_name(row, "\"i32\"".as_ptr() as *const i8), &mut basic.i32);
            cass_value_get_int64(cass_row_get_column_by_name(row, "i64".as_ptr() as *const i8), &mut basic.i64);
        }
        cass_result_free(result);
        cass_iterator_free(iterator);
    }
    cass_future_free(future);
    cass_statement_free(statement);
    return rc;
}

fn main() {unsafe{
    let cluster = create_cluster();
    let session = cass_session_new();
    let input = Basic{bln:cass_true, flt:0.001f32, dbl:0.0002, i32:1, i64:2 };
    let mut output = Basic{bln:cass_false, flt:0f32, dbl:0.0, i32:0, i64:0 };
    let insert_query = cass_string_init("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);".as_ptr() as *const i8);
    let select_query = cass_string_init("SELECT * FROM examples.basic WHERE key = ?".as_ptr() as *const i8);
    match connect_session(&mut*session, &mut*cluster) {
        CassError::CASS_OK => CassError::CASS_OK,
        _ => {
            cass_cluster_free(cluster);
            cass_session_free(session);
            panic!();
        }
    };
    execute_query(&mut*session, "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
    execute_query(&mut*session, "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double,i32 int, i64 bigint, PRIMARY KEY (key));");
    match prepare_query(&mut*session, insert_query) {
        Ok(insert_prepared) => {
            insert_into_basic(&mut*session, insert_prepared, "prepared_test", input);
            cass_prepared_free(insert_prepared);
        },
        Err(err) => {panic!("{:?}",err)}
    }
    match prepare_query(&mut*session, select_query) {
        Ok(select_prepared) => {
            select_from_basic(&mut  *session, select_prepared, "prepared_test", &mut output);
            assert!(input.bln == output.bln);
            assert!(input.flt == output.flt);
            assert!(input.dbl == output.dbl);
            assert!(input.i32 == output.i32);
            assert!(input.i64 == output.i64);
            cass_prepared_free(select_prepared);
        },
        Err(err) => {panic!("{:?}",err)}
    }
    let close_future = cass_session_close(session);
    cass_future_wait(close_future);
    cass_future_free(close_future);
    cass_cluster_free(cluster);
    cass_session_free(session);
}}
