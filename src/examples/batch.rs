#![feature(std_misc)]

extern crate cql_ffi;
use std::ffi::CString;
use cql_ffi::CassCluster;
use cql_ffi::CassSession;
use cql_ffi::CassString;
use cql_ffi::CassStatement;
use cql_ffi::CassBatch;
use cql_ffi::CassBatchType;
use cql_ffi::CassPrepared;
use cql_ffi::CassError;
use cql_ffi::str2ref;
use cql_ffi::str2cass_string;

struct Pair<'a> {
    key:&'a str,
    value:&'a str
}

fn create_cluster() -> CassCluster {unsafe{
    let mut cluster = CassCluster::new();
    let _ = cluster.set_contact_points(str2ref("127.0.0.1,127.0.0.2,127.0.0.3"));
    cluster 
}}

fn connect_session(session:&mut CassSession, cluster:&mut CassCluster) -> Result<(),CassError> {unsafe{
    let future = &mut session.connect(cluster);
    future.wait();
    let rc = future.error_code();
    future.free();
    rc
}}

fn execute_query(session: &mut CassSession, query: &str) -> Result<(),CassError> {unsafe{
    let statement = CassStatement::new(CassString::init(CassString::init(CString::from_slice(query.as_bytes()).as_ptr()).0.data), 0);
    let mut future = session.execute(statement);
    future.wait();
    let rc = future.error_code();
    future.free();
    statement.free();
    return rc;
}}

fn prepare_insert_into_batch(session:&mut CassSession) -> Result<CassPrepared,CassError> {unsafe{
    let query = str2cass_string("INSERT INTO examples.pairs (key, value) VALUES (?, ?)");
    let mut future = session.prepare(query);
    future.wait();
    let _ = future.error_code();
    let prepared = future.get_prepared();
    future.free();
    Ok(prepared)
}}

fn insert_into_batch_with_prepared<'a>(session:&mut CassSession, pairs:Vec<Pair>, prepared:&'a CassPrepared)-> Result<&'a CassPrepared,CassError> {unsafe{
    let batch = &mut CassBatch::new(CassBatchType::LOGGED);
    for pair in pairs.iter() {
        let statement = prepared.bind();
        let _ = statement.bind_string(0, str2cass_string(pair.key));
        let _ = statement.bind_string(1, str2cass_string(pair.value));
        batch.add_statement(statement);
        statement.free();
    }
    let statement = CassStatement::new(str2cass_string("INSERT INTO examples.pairs (key, value) VALUES ('c', '3')"), 0);
    batch.add_statement(statement);
    statement.free();

    let statement = CassStatement::new(str2cass_string("INSERT INTO examples.pairs (key, value) VALUES (?, ?)"),2);
    let _ = statement.bind_string(0, str2cass_string("d"));
    let _ = statement.bind_string(1, str2cass_string("4"));
    batch.add_statement(statement);
    statement.free();

    let mut future = session.execute_batch(batch);
    future.wait();
    let _ = future.error_code();
    future.free();
    batch.free();
    Ok(prepared)
}}

fn main() {unsafe{
    let cluster = &mut create_cluster();
    let session = &mut CassSession::new();
    let pairs = vec!(Pair{key:"a", value:"1"}, Pair{key:"b", value:"2"});
    let _ = connect_session(session, cluster);
    let _ = execute_query(session, "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
    let _ = execute_query(session, "CREATE TABLE examples.pairs (key text, value text, PRIMARY KEY (key));");
    match prepare_insert_into_batch(session) {
        Ok(ref mut prepared) => {
            match insert_into_batch_with_prepared(session, pairs, prepared) {
                Ok(_) => prepared.free(),
                Err(_) => {panic!()}
            }
        }
        Err(err) => panic!(err)
    };
    let mut close_future = session.close();
    close_future.wait();
    close_future.free();
    cluster.free();
    session.free();
}}
