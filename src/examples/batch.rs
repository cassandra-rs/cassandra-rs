extern crate cassandra;
use cassandra::CassCluster;
use cassandra::CassSession;
use cassandra::CassStatement;
use cassandra::CassBatch;
use cassandra::CassBatchType;
use cassandra::CassPrepared;
use cassandra::CassError;

//use cql_ffi::AsContactPoints;

struct Pair<'a> {
    key:&'a str,
    value:&'a str
}

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.pairs (key text, value text, PRIMARY KEY (key));";
static INSERT_QUERY:&'static str = "INSERT INTO examples.pairs (key, value) VALUES (?, ?)";
static SELECT_QUERY:&'static str = "SELECT * from examples.pairs";

fn insert_into_batch_with_prepared<'a>(session:&mut CassSession, pairs:Vec<Pair>)-> Result<CassPrepared,CassError> {
    let prepared = session.prepare(INSERT_QUERY).unwrap().wait().unwrap();
    let mut batch = CassBatch::new(CassBatchType::LOGGED);
    for pair in pairs.iter() {
	    let mut statement = prepared.bind();
        try!(statement.bind_string(0, pair.key));
        try!(statement.bind_string(1, pair.value));
        batch.add_statement(statement);
    }
    try!(session.execute_batch(batch).wait());
    Ok(prepared)
}

pub fn verify_batch(session:&mut CassSession) {
    let result = session.execute(SELECT_QUERY,0).wait().unwrap();
    println!("{:?}",result);
}

fn main() {
    let cluster = &CassCluster::new().set_contact_points("127.0.0.1").unwrap();
    let session = &mut CassSession::new().connect(cluster).wait().unwrap();
    
    let pairs = vec!(   
        Pair{key:"a", value:"1"},
        Pair{key:"b", value:"2"},
        Pair{key:"c", value:"3"},
        Pair{key:"d", value:"4"},
    );
    
    session.execute(CREATE_KEYSPACE,0).wait().unwrap();
    session.execute_statement(CassStatement::new(CREATE_TABLE,0)).wait().unwrap();
    insert_into_batch_with_prepared(session, pairs).unwrap();
   	verify_batch(session);
    session.close();
}
