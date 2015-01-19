extern crate cql_ffi;

use std::slice;

use cql_ffi::*;

static NUM_CONCURRENT_REQUESTS:isize=1000;

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

unsafe fn insert_into_paging(session:&mut CassSession, uuid_gen:&mut CassUuidGen) {
    let query = str2cass_string("INSERT INTO paging (key, value) VALUES (?, ?);");
    let mut futures:Vec<&mut CassFuture> = Vec::with_capacity(NUM_CONCURRENT_REQUESTS as usize);

    for i in (0..NUM_CONCURRENT_REQUESTS) {
        let statement = cass_statement_new(query, 2);
        let key = gencassuuid(&mut*uuid_gen).unwrap();
        cass_statement_bind_uuid(statement, 0, key);
        println!("{}", i);
        cass_statement_bind_string(statement, 1, str2cass_string(cassuuid2string(key).unwrap().as_slice()));

        futures.push(&mut*cass_session_execute(session, statement));
        cass_statement_free(statement);
    }
    while futures.len() > 0 {
        let future = futures.pop().unwrap();
        let rc = cass_future_error_code(future);
        if rc != CassError::CASS_OK {
            print_error(future);
        }
        cass_future_free(&mut*future);
    }
}

unsafe fn select_from_paging(session:&mut CassSession) {
   let mut has_more_pages = true;
    let query = str2cass_string("SELECT * FROM paging");
    let statement = cass_statement_new(query, 0);
    cass_statement_set_paging_size(statement, 100);
    while has_more_pages {
        let future = cass_session_execute(session, statement);
        if cass_future_error_code(future) != CassError::CASS_OK {
            print_error(&mut*future);
            panic!();
        }
        
        let result = cass_future_get_result(future);
        let iterator = cass_iterator_from_result(result);
        cass_future_free(future);
        while cass_iterator_next(iterator) > 0 {
            let row = cass_iterator_get_row(iterator);
            let key = cassvalue2cassuuid(&*cass_row_get_column(row, 0));
            let key_str = cassuuid2string(key.unwrap());
            let value = cassvalue2cassstring(&*cass_row_get_column(row, 1));
            println!("key: '{:?}' value: '{:?}'\n", key_str, value);
        }
        has_more_pages = cass_result_has_more_pages(result) > 0;
        if has_more_pages {
            cass_statement_set_paging_state(statement, result);
        }
        cass_iterator_free(iterator);
//        cass_result_free(result);
    } 
    cass_statement_free(statement);
}

fn main() {unsafe{
    let uuid_gen = cass_uuid_gen_new();
    let  cluster = create_cluster();
    let session = cass_session_new();
    if connect_session(&mut*session, &mut*cluster) != CassError::CASS_OK {
        cass_cluster_free(cluster);
        cass_session_free(session);
    }
    execute_query(&mut*session, "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
    execute_query(&mut*session, "CREATE TABLE examples.paging (key timeuuid, value text, PRIMARY KEY (key));");
    execute_query(&mut*session, "USE examples");
    insert_into_paging(&mut*session, &mut*uuid_gen);
    select_from_paging(&mut*session);    
    let close_future = cass_session_close(session);
    cass_future_wait(close_future);
    cass_future_free(close_future);
    cass_uuid_gen_free(uuid_gen);
    cass_cluster_free(cluster);
    cass_session_free(session);
}}
