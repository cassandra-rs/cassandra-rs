extern crate cql_ffi;
use cql_ffi::*;

#[derive(Copy,Debug,PartialEq)]
struct Basic {
    bln:bool,
    flt:f32,
    dbl:f64,
    i32:i32,
    i64:i64
}

fn create_cluster() -> Result<CassCluster,CassError> {
    let cluster = CassCluster::new();
    cluster.set_contact_points("127.0.0.1".as_contact_points())
}

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double,i32 int, i64 bigint, PRIMARY KEY (key));";
static INSERT_QUERY:&'static str = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
static SELECT_QUERY:&'static str = "SELECT * FROM examples.basic WHERE key = ?";

//fixme row key sent is null?
fn insert_into_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:Basic) -> Result<CassResult,CassError> {
    println!("key={:?}",key);
    let statement = &prepared.bind();
    statement.bind_string_by_name("key", key.as_cass_str()).unwrap()
        .bind_bool_by_name("BLN", basic.bln).unwrap()
        .bind_float_by_name("FLT", basic.flt).unwrap()
        .bind_double_by_name("\"dbl\"", basic.dbl).unwrap()
        .bind_int32_by_name("i32", basic.i32).unwrap()
        .bind_int64_by_name("I64", basic.i64).unwrap();

    session.execute_statement(statement).wait()
}

unsafe fn select_from_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:&mut Basic) -> Result<CassResult,CassError> {
    let statement = prepared.bind();
    let statement = statement.bind_string_by_name("key", key.as_cass_str()).unwrap();
    match session.execute_statement(&statement).wait() {
        Ok(result) => {
            for row in result.iter() {
                basic.bln = try!(row.get_column_by_name("BLN").get_bool());
                basic.dbl = try!(row.get_column_by_name("dbl").get_double());
                basic.flt = try!(row.get_column_by_name("flt").get_float());
                basic.i32 = try!(row.get_column_by_name("\"i32\"").get_int32());
                basic.i64 = try!(row.get_column_by_name("i64").get_int64());
            }
            Ok(result)
        }
        Err(_) => panic!("err")
    }
}

fn main() {unsafe{
    let cluster = &mut create_cluster().unwrap();
    match CassSession::new().connect(cluster).wait() {
        Ok(mut session) => {
            let input = Basic{bln:true, flt:0.001f32, dbl:0.0002, i32:1, i64:2 };
            let mut output = Basic{bln:false, flt:0f32, dbl:0.0, i32:0, i64:0 };
            let _ = session.execute(CREATE_KEYSPACE,0).wait().unwrap();
            let _ = session.execute(CREATE_TABLE,0).wait().unwrap();
            match session.prepare(INSERT_QUERY).unwrap().wait() {
                Ok(mut insert_prepared) => {
                    insert_into_basic(&mut session, &mut insert_prepared, "prepared_test", input).unwrap();
                },
                Err(err) => println!("error: {:?}",err)
            }
            match session.prepare(SELECT_QUERY).unwrap().wait() {
                Ok(ref mut select_prepared) => {
                    select_from_basic(&mut session, &select_prepared, "prepared_test", &mut output).unwrap();
                    assert_eq!(input,output);
                    println!("results matched: {:?}", output);
                },
                Err(err) => println!("err: {:?}",err)
            }
            session.close().wait().unwrap();
        }
        Err(err) => println!("couldn't connect: {:?}",err)
    }    
}}
