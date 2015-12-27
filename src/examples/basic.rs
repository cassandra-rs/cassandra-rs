extern crate cassandra;

use cassandra::*;

const CONTACT_POINTS: &'static str = "127.0.0.1";

const CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = \
                                       { \'class\': \'SimpleStrategy\', \'replication_factor\': \
                                       \'1\' };";
const CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln \
                                    boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY \
                                    KEY (key));";
const INSERT_QUERY: &'static str = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) \
                                    VALUES (?, ?, ?, ?, ?, ?);";
const SELECT_QUERY: &'static str = "SELECT * FROM examples.basic WHERE key = ?";

#[derive(Debug,PartialEq,Clone,Copy)]
struct Basic {
    bln: bool,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn insert_into_basic(session: &mut Session,
                     key: &str,
                     basic: &Basic)
                     -> Result<CassResult, CassError> {
    let mut statement = Statement::new(INSERT_QUERY, 6);
    try!(statement.bind_string(0, key));
    try!(statement.bind_bool(1, basic.bln));
    try!(statement.bind_float(2, basic.flt));
    try!(statement.bind_double(3, basic.dbl));
    try!(statement.bind_int32(4, basic.i32));
    try!(statement.bind_int64(5, basic.i64));
    Ok(try!(session.execute_statement(&statement).wait()))
}

fn select_from_basic(session: &mut Session, key: &str) -> Result<Basic, CassError> {
    let mut statement = Statement::new(SELECT_QUERY, 1);
    try!(statement.bind_string(0, key));
    let result = try!(session.execute_statement(&statement).wait());
    println!("Result: \n{:?}\n", result);
    match result.first_row() {
        None => Err(CassError::build(1)),
        Some(row) => {
            Ok(Basic {
                bln: try!(try!(row.get_column(1)).get_bool()),
                dbl: try!(try!(row.get_column(2)).get_double()),
                flt: try!(try!(row.get_column(3)).get_float()),
                i32: try!(try!(row.get_column(4)).get_i32()),
                i64: try!(try!(row.get_column(5)).get_i64()),
            })
        }
    }
}


fn main() {
    let input = Basic {
        bln: true,
        flt: 0.001f32,
        dbl: 0.0002f64,
        i32: 1,
        i64: 2,
    };

    let mut cluster = Cluster::new();
    let contact_points: Vec<&str> = vec![CONTACT_POINTS];
    cluster.set_contact_points(contact_points)
           .unwrap()
           .set_load_balance_round_robin()
           .unwrap();

    let session_future = Session::new().connect(&cluster).wait();

    match session_future {
        Ok(mut session) => {
            session.execute(CREATE_KEYSPACE, 0).wait().unwrap();
            session.execute(CREATE_TABLE, 0).wait().unwrap();

            insert_into_basic(&mut session, "test", &input).unwrap();
            let output = select_from_basic(&mut session, "test").unwrap();

            println!("{:?}", input);
            println!("{:?}", output);

            assert!(input == output);

            session.close().wait().unwrap();
        }
        _ => {}
    }
}
