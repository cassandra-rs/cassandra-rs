extern crate cql_ffi;
use cql_ffi::CassCluster;
use cql_ffi::CassSession;
use cql_ffi::CassStatement;
use cql_ffi::CassBatch;
use cql_ffi::CassBatchType;
use cql_ffi::CassPrepared;
use cql_ffi::CassError;
use cql_ffi::str2cass_string;

struct Pair<'a> {
    key:&'a str,
    value:&'a str
}

fn create_cluster() -> Result<CassCluster,CassError> {unsafe{
    CassCluster::new().set_contact_points("127.0.0.1")
}}

fn prepare_insert_into_batch(session:&mut CassSession) -> Result<CassPrepared,CassError> {unsafe{
    let query = "INSERT INTO examples.pairs (key, value) VALUES (?, ?)";
    let mut future = session.prepare(query).wait().unwrap();
    let prepared = future.get_prepared();
    Ok(prepared)
}}

fn insert_into_batch_with_prepared<'a>(session:&mut CassSession, pairs:Vec<Pair>, prepared:&'a CassPrepared)-> Result<&'a CassPrepared,CassError> {unsafe{
    let batch = &mut CassBatch::new(CassBatchType::LOGGED);
    for pair in pairs.iter() {
        let statement = prepared.bind();
        let _ = statement.bind_string(0, str2cass_string(pair.key));
        let _ = statement.bind_string(1, str2cass_string(pair.value));
        batch.add_statement(statement);
    }
    let statement = CassStatement::new("INSERT INTO examples.pairs (key, value) VALUES ('c', '3')", 0);
    batch.add_statement(statement);
    let statement = CassStatement::new("INSERT INTO examples.pairs (key, value) VALUES (?, ?)",2);
    let _ = statement.bind_string(0, str2cass_string("d"));
    let _ = statement.bind_string(1, str2cass_string("4"));
    batch.add_statement(statement);
    try!(session.execute_batch(batch).wait());
    Ok(prepared)
}}

fn main() {unsafe{
    let cluster = &mut create_cluster().unwrap();
    let mut session = CassSession::new();
    let pairs = vec!(Pair{key:"a", value:"1"}, Pair{key:"b", value:"2"});
    let _ = session.connect(cluster);
    let _ = session.execute_statement(&CassStatement::new("CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };",0));
    let _ = session.execute_statement(&CassStatement::new("CREATE TABLE examples.pairs (key text, value text, PRIMARY KEY (key));",0));
    match prepare_insert_into_batch(&mut session) {
        Ok(ref mut prepared) => insert_into_batch_with_prepared(&mut session, pairs, prepared).unwrap(),
        Err(err) => panic!(err)
    };
    session.close().wait().unwrap();
}}
