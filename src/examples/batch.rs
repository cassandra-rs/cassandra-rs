extern crate cql_ffi;
use std::ffi::CString;
use std::slice;
use cql_ffi::*;

struct Pair<'a> {
    key:&'a str,
    value:&'a str
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
    let statement = cass_statement_new(cass_string_init(cass_string_init(CString::from_slice(query.as_bytes()).as_ptr()).data), 0);
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

fn prepare_insert_into_batch(session:&mut CassSession, prepared:&Option<&mut CassPrepared>) -> CassError {unsafe{
    let  query = str2cass_string("INSERT INTO examples.pairs (key, value) VALUES (?, ?)");
    let future = cass_session_prepare(session, query);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CASS_OK => {cass_future_get_prepared(future);},
        _ => print_error(&mut*future)
    }
    cass_future_free(future);
    return rc;
}}

fn insert_into_batch_with_prepared<'a>(session:&mut CassSession, pairs:Vec<Pair>, prepared:&'a mut CassPrepared)-> Result<&'a mut CassPrepared,CassError> {unsafe{
    let batch = cass_batch_new(CASS_BATCH_TYPE_LOGGED);
    for pair in pairs.iter() {
        let statement = cass_prepared_bind(prepared);
        cass_statement_bind_string(statement, 0, str2cass_string(pair.key));
        cass_statement_bind_string(statement, 1, str2cass_string(pair.value));
        cass_batch_add_statement(batch, statement);
        cass_statement_free(statement);
    }
    let statement = cass_statement_new(str2cass_string("INSERT INTO examples.pairs (key, value) VALUES ('c', '3')"), 0);
    cass_batch_add_statement(batch, statement);
    cass_statement_free(statement);

    let statement = cass_statement_new(str2cass_string("INSERT INTO examples.pairs (key, value) VALUES (?, ?)"),2);
    cass_statement_bind_string(statement, 0, str2cass_string("d"));
    cass_statement_bind_string(statement, 1, str2cass_string("4"));
    cass_batch_add_statement(batch, statement);
    cass_statement_free(statement);

    let future = cass_session_execute_batch(session, batch);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    if (rc != CASS_OK) {
        print_error(&mut*future);
    }
    cass_future_free(future);
    cass_batch_free(batch);
    Ok(prepared)
}}

fn main() {unsafe{
    let cluster = create_cluster();
    let session = cass_session_new();
    let pairs = vec!(Pair{key:"a", value:"1"}, Pair{key:"b", value:"2"});
    match connect_session(&mut*session, &mut*cluster) {
        CASS_OK => {},
        _ => {
            cass_cluster_free(cluster);
            cass_session_free(session);
            panic!();
        }
    }
    let prepared:Option<&mut CassPrepared>=None;
    execute_query(&mut*session, "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
    execute_query(&mut*session, "CREATE TABLE examples.pairs (key text, value text, PRIMARY KEY (key));");
    let rc = prepare_insert_into_batch(&mut*session, &prepared);
    let prepared = prepared.unwrap();
    if rc == CASS_OK {
        insert_into_batch_with_prepared(&mut*session, pairs, prepared);
    }
    cass_prepared_free(prepared);
    let close_future = cass_session_close(session);
    cass_future_wait(close_future);
    cass_future_free(close_future);
    cass_cluster_free(cluster);
    cass_session_free(session);

}}
