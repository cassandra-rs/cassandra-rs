extern crate cassandra;
use cassandra::Cluster;
use cassandra::Session;
use cassandra::Statement;
use cassandra::Batch;
use cassandra::BatchType;
use cassandra::PreparedStatement;
use cassandra::CassandraError;

// use cql_ffi::AsContactPoints;

struct Pair<'a> {
    key: &'a str,
    value: &'a str,
}

static CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'1\' };";
static CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.pairs (key text, value text, PRIMARY KEY \
                                     (key));";
static INSERT_QUERY: &'static str = "INSERT INTO examples.pairs (key, value) VALUES (?, ?)";
static SELECT_QUERY: &'static str = "SELECT * from examples.pairs";

fn insert_into_batch_with_prepared<'a>(session: &mut Session,
                                       pairs: Vec<Pair>)
                                       -> Result<PreparedStatement, CassandraError> {
    let prepared = session.prepare(INSERT_QUERY).unwrap().wait().unwrap();
    let mut batch = Batch::new(BatchType::LOGGED);
    for pair in pairs {
        let mut statement = prepared.bind();
        try!(statement.bind_string(0, pair.key));
        try!(statement.bind_string(1, pair.value));
        match batch.add_statement(statement) {
            Ok(_) => {}
            Err(err) => return Err(CassandraError::build(err)),
        }
    }
    try!(session.execute_batch(batch).wait());
    Ok(prepared)
}

pub fn verify_batch(session: &mut Session) {
    let result = session.execute(SELECT_QUERY, 0).wait().unwrap();
    println!("{:?}", result);
}

fn main() {
    let mut cluster = Cluster::new();
    cluster.set_contact_points("127.0.0.1").unwrap();
    let mut session = Session::new().connect(&mut cluster).wait().unwrap();

    let pairs = vec!(
        Pair{key:"a", value:"1"},
        Pair{key:"b", value:"2"},
        Pair{key:"c", value:"3"},
        Pair{key:"d", value:"4"},
    );

    session.execute(CREATE_KEYSPACE, 0).wait().unwrap();
    session.execute_statement(&Statement::new(CREATE_TABLE, 0)).wait().unwrap();
    insert_into_batch_with_prepared(&mut session, pairs).unwrap();
    verify_batch(&mut session);
    session.close();
}
