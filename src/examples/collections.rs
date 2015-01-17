#![allow(unstable)]
extern crate cql_ffi;
use std::ffi::CString;
use std::slice;
use cql_ffi::*;

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

unsafe fn execute_query(session: &mut CassSession, query: &str) -> CassError {
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
}


unsafe fn insert_into_collections(session:&mut CassSession, key:&str, items:Vec<String>) -> CassError {
    let query = cass_string_init("INSERT INTO examples.collections (key, items) VALUES (?, ?);".as_ptr() as *const i8);
    let statement = cass_statement_new(query, 2);
    cass_statement_bind_string(statement, 0, cass_string_init(key.as_ptr() as *const i8));
    let collection = cass_collection_new(CASS_COLLECTION_TYPE_SET, 2);
    for item in items.iter() {
        cass_collection_append_string(collection, cass_string_init(item.as_bytes().as_ptr() as *const i8));
    }
    cass_statement_bind_collection(statement, 1, collection);
    cass_collection_free(collection);
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc  {
        CassError::CASS_OK => {print_error(&mut*future)},
        _ => panic!(rc)
    }
    cass_future_free(future);
    cass_statement_free(statement);
    return rc;
}

unsafe fn select_from_collections(session:&mut CassSession, key:&str) {
    let query = str2cass_string("SELECT items FROM examples.collections WHERE key = ?");
    let statement = cass_statement_new(query, 1);
    cass_statement_bind_string(statement, 0, str2cass_string(key));
    let future = cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CassError::CASS_OK => {
            print_error(&mut *future);
            let result = cass_future_get_result(future);
            let iterator = cass_iterator_from_result(result);
            if cass_iterator_next(iterator) > 0 {
                let row = cass_iterator_get_row(iterator);
                let value = cass_row_get_column(row, 0);
                let items_iterator = cass_iterator_from_collection(value);
                while cass_iterator_next(items_iterator) > 0 {
                    let  value = cass_iterator_get_value(items_iterator);
                    let item_string = cassvalue2cassstring(&*value);
                    match item_string {
                        Ok(item) => println!("item: {:?}", item),
                        Err(err) => panic!(err)
                    }
                }
            cass_iterator_free(items_iterator);
            }
        cass_result_free(result);
        cass_iterator_free(iterator);
        }
        _ => {panic!(rc)}
    }
    cass_future_free(future);
    cass_statement_free(statement);
}

fn main() {unsafe{
    println!("argle:{:?}",str2cass_string("foo"));
    
    let cluster = create_cluster();
    let session = cass_session_new();
    let items = vec!("apple".to_string(), "orange".to_string(), "banana".to_string(), "mango".to_string());
    match connect_session(&mut*session, &mut*cluster) {
        CassError::CASS_OK => {
            execute_query(&mut*session,"CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };");
            execute_query(&mut*session,"CREATE TABLE examples.collections (key text, items set<text>, PRIMARY KEY (key))");
            insert_into_collections(&mut*session, "test", items);
            select_from_collections(&mut*session, "test");
            let close_future = cass_session_close(&mut*session);
            cass_future_wait(close_future);
            cass_future_free(close_future);
        },
        _ => {panic!();}
    }
    cass_cluster_free(cluster);
    cass_session_free(session);
}}
