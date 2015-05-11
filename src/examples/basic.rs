#![feature(custom_attribute)]

extern crate cql_ffi;

use cql_ffi::*;

const CONTACT_POINTS:&'static str = "127.0.0.1";

const CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";
const CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";
const INSERT_QUERY:&'static str = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
const SELECT_QUERY:&'static str = "SELECT * FROM examples.basic WHERE key = ?";

#[derive(Debug)]
struct Basic {
    bln:bool,
    flt:f32,
    dbl:f64,
    i32:i32,
    i64:i64,
}

fn insert_into_basic(session: &mut CassSession, key:&str, basic:&Basic) -> Result<CassResult,CassError> {
    let statement = CassStatement::new(INSERT_QUERY, 6);
    try!(statement.bind_string(0, key));
    try!(statement.bind_bool(1, basic.bln));
    try!(statement.bind_float(2, basic.flt));
    try!(statement.bind_double(3, basic.dbl));
    try!(statement.bind_int32(4, basic.i32));
    try!(statement.bind_int64(5, basic.i64));
    Ok(try!(session.execute_statement(&statement).wait()))
}

fn select_from_basic(session:&mut CassSession, key:&str, basic:&mut Basic) -> Result<CassResult,CassError> {
    let statement = CassStatement::new(SELECT_QUERY, 1);
    let statement = try!(statement.bind_string(0, key));
    match session.execute_statement(statement).wait() {
        Ok(result) => {
            println!("Result: \n{:?}\n",result);
            for row in result.iter() {
                basic.bln = try!(try!(row.get_column(1)).get_bool());
                basic.dbl = try!(try!(row.get_column(2)).get_double());
                basic.flt = try!(try!(row.get_column(3)).get_float());
                basic.i32 = try!(try!(row.get_column(4)).get_int32());
                basic.i64 = try!(try!(row.get_column(5)).get_int64());
            }
            Ok(result)
        }
        Err(_) => panic!("error")
    }
}

fn main() {
    let input = Basic{bln:true, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };

    let cluster = &CassCluster::new()
                        .set_contact_points(CONTACT_POINTS).unwrap()
                        .set_load_balance_round_robin().unwrap();

    let session_future = CassSession::new().connect(cluster).wait();

    match session_future {
        Ok(mut session) => {
            let mut output = Basic{bln:false,flt:0f32,dbl:0f64,i32:0,i64:0};
            session.execute(CREATE_KEYSPACE,0);
            session.execute(CREATE_TABLE,0);
            insert_into_basic(&mut session, "test", &input).unwrap();
            select_from_basic(&mut session, "test", &mut output).unwrap();
            println!("{:?}",input);
            println!("{:?}",output);
            assert!(input.bln == output.bln);
            assert!(input.flt == output.flt);
            assert!(input.dbl == output.dbl);
            assert!(input.i32 == output.i32);
            assert!(input.i64 == output.i64);
            session.close().wait().unwrap();
        },
        _ => {}
    }
}
