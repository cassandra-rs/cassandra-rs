#![allow(unstable)]
extern crate cql_ffi;

use std::slice;

use cql_ffi::*;

struct Pair<'a> {
    key:&'a str,
    value:i32
}


unsafe fn print_error(future:&mut CassFuture) {
    let message = cass_future_error_message(future);
    let message = slice::from_raw_buf(&message.data,message.length as usize);
    println!("Error: {:?}", message);
}

unsafe fn create_cluster() -> *mut CassCluster {
    let cluster = cass_cluster_new();
    cass_cluster_set_contact_points(cluster, str2ref("127.0.0.1,127.0.0.2,127.0.0.3"));
    cluster 
}

unsafe fn connect_session(session:&mut CassSession, cluster:&mut CassCluster) -> CassError {
    let future = &mut *cass_session_connect(session, cluster);
    cass_future_wait(future);
    let rc = match cass_future_error_code(future) {
        CassError::CASS_OK => {CassError::CASS_OK},
        _=> panic!("{:?}",future)
    };
    cass_future_free(future);
    rc
}

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

unsafe fn insert_into_maps(session:&mut CassSession, key:&str, items:Vec<Pair>) -> CassError {
    let query = str2cass_string("INSERT INTO examples.maps (key, items) VALUES (?, ?);");
    let statement = cass_statement_new(query, 2);
    cass_statement_bind_string(statement, 0, str2cass_string(key));
    let collection = cass_collection_new(CassCollectionType::MAP, 5);
    for item in items.iter() {
        cass_collection_append_string(collection, str2cass_string(item.key));
        cass_collection_append_int32(collection, item.value);
    }
    cass_statement_bind_collection(statement, 1, collection);
    cass_collection_free(collection);
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    if rc != CassError::CASS_OK {
        print_error(&mut*future);
    }
    cass_future_free(future);
    cass_statement_free(statement);
    rc
}

unsafe fn select_from_maps(session: &mut CassSession, key:&str) {
    let query = str2cass_string("SELECT items FROM examples.maps WHERE key = ?");
    let statement = cass_statement_new(query, 1);
    cass_statement_bind_string(statement, 0, str2cass_string(key));
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    if rc != CassError::CASS_OK {
        print_error(&mut*future);
    } else {
        let result = cass_future_get_result(future);
        if cass_result_row_count(result) > 0 {
        let row = cass_result_first_row(result);
        let iterator = cass_iterator_from_map(
            cass_row_get_column(row, 0));
            let mut value = 0;
            while cass_iterator_next(iterator) > 0 {
                match cassvalue2cassstring(&*cass_iterator_get_map_key(iterator)) {
                    Ok(key) => {
                        cass_value_get_int32(cass_iterator_get_map_value(iterator), &mut value);
                        println!("item: '{:?}' : {:?}", key, value);
                    }
                    Err(err) => panic!(err)
                }
            }
            cass_iterator_free(iterator);
        }
        cass_result_free(result);
    }
    cass_future_free(future);
    cass_statement_free(statement);
}

fn main() {unsafe{
    let cluster = create_cluster();
    let session = cass_session_new();
    let items:Vec<Pair> = vec!(
        Pair{key:"apple", value:1 },
        Pair{key:"orange", value:2 },
        Pair{key:"banana", value:3 },
        Pair{key:"mango", value:4 }
    );
    if connect_session(&mut*session, &mut*cluster) != CassError::CASS_OK {
        cass_cluster_free(cluster);
        cass_session_free(session);
        panic!();
    }

    execute_query(&mut*session, "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
    execute_query(&mut*session, "CREATE TABLE examples.maps (key text, items map<text, int>, PRIMARY KEY (key))");
    insert_into_maps(&mut*session, "test", items);
    select_from_maps(&mut*session, "test");
    let close_future = cass_session_close(&mut*session);
    cass_future_wait(close_future);
    cass_future_free(close_future);
    cass_cluster_free(cluster);
    cass_session_free(&mut*session);
}}
