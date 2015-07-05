extern crate cassandra;

use cassandra::*;

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";
static INSERT_QUERY:&'static str = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
static SELECT_QUERY:&'static str = "SELECT * FROM examples.basic WHERE key = ?";

#[derive(Debug,PartialEq)]
struct Basic {
    bln:bool,
    flt:f32,
    dbl:f64,
    i32:i32,
    i64:i64
}

fn insert_into_basic(session:&mut CassSession, key:&str, basic:&mut Basic) -> Result<(),CassError> {
    let mut statement = CassStatement::new(INSERT_QUERY, 6);
    statement
        .bind_string(0, key).unwrap()
        .bind_bool(1, basic.bln).unwrap()
        .bind_float(2, basic.flt).unwrap()
        .bind_double(3, basic.dbl).unwrap()
        .bind_int32(4, basic.i32).unwrap();
//        .bind_int64(5, basic.i64).unwrap();
    try!(session.execute_statement(&statement).wait());
    Ok(())
}


fn select_from_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:&mut Basic) -> Result<(),CassError> {
    let mut statement = prepared.bind();
    try!(statement.bind_string(0, key));
    let mut future = session.execute_statement(&statement);
    match future.wait() {
        Ok(result) => {
            println!("{:?}", result);
            for row in result.iter() {
                basic.bln = try!(try!(row.get_column(1)).get_bool());
                basic.dbl = try!(try!(row.get_column(2)).get_double());
                basic.flt = try!(try!(row.get_column(3)).get_float());
                basic.i32 = try!(try!(row.get_column(4)).get_int32());
                basic.i64 = try!(try!(row.get_column(5)).get_int64());
            }
            Ok(())
        },
        Err(err) => panic!("{:?}",err)
    }
}

fn main() {
    let mut cluster = CassCluster::new();
    cluster.set_contact_points("127.0.0.1").unwrap();
    let mut session = CassSession::new().connect(&mut cluster).wait().unwrap();
    let mut input = Basic{bln:true, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };
    let mut output = Basic{bln:false, flt:0f32, dbl:0f64, i32:0, i64:0};
    session.execute(CREATE_KEYSPACE,0);
    session.execute(CREATE_TABLE,0);
    insert_into_basic(&mut session, "prepared_test", &mut input).unwrap();
    match session.prepare(SELECT_QUERY).unwrap().wait() {
        Ok(prepared) => {
            select_from_basic(&mut session, &prepared, "prepared_test", &mut output).unwrap();
            println!("input: {:?}\nouput: {:?}", input,output);
            assert_eq!(input,output);
        },
        Err(err) => panic!(err)
    }
    let close_future = session.close();
    close_future.wait().unwrap();
}
