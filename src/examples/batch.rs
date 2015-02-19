extern crate cql_ffi;
use cql_ffi::CassCluster;
use cql_ffi::CassSession;
use cql_ffi::CassStatement;
use cql_ffi::CassBatch;
use cql_ffi::CassBatchType;
use cql_ffi::CassPrepared;
use cql_ffi::CassError;

struct Pair<'a> {
    key:&'a str,
    value:&'a str
}

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.pairs (key text, value text, PRIMARY KEY (key));";
static INSERT_QUERY:&'static str = "INSERT INTO examples.pairs (key, value) VALUES (?, ?)";

fn create_cluster() -> Result<CassCluster,CassError> {
    CassCluster::new().set_contact_points("127.0.0.1")
}

fn insert_into_batch_with_prepared<'a>(session:&mut CassSession, pairs:Vec<Pair>, prepared:&'a CassPrepared)-> Result<&'a CassPrepared,CassError> {unsafe{
    let batch = &mut CassBatch::new(CassBatchType::LOGGED);
    for pair in pairs.iter() {
        let statement = prepared.bind();
        statement.bind_string(0, pair.key).unwrap()
            .bind_string(1, pair.value).unwrap();
        batch.add_statement(&statement);
        try!(session.execute_batch(batch).wait());
    }
    Ok(prepared)
}}

fn main() {
    let cluster = &mut create_cluster().unwrap();
    let mut session = CassSession::new().connect(cluster).wait().unwrap();
    let pairs = vec!(   Pair{key:"a", value:"1"},
                        Pair{key:"b", value:"2"}
                    );
    session.execute(CREATE_KEYSPACE,0).wait().unwrap();
    session.execute_statement(&CassStatement::new(CREATE_TABLE,0)).wait().unwrap();
    match session.prepare(INSERT_QUERY).wait() {
        Ok(ref mut prepared) => insert_into_batch_with_prepared(&mut session, pairs, prepared).unwrap(),
        Err(err) => panic!(err)
    };
    session.close();
}
