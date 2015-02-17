#![feature(collections)]

extern crate collections;
extern crate cql_ffi;

use cql_ffi::*;

#[derive(Debug)]
struct Basic {
    bln:cass_bool_t,
    flt:cass_float_t,
    dbl:cass_double_t,
    i32:cass_int32_t,
    i64:cass_int64_t,
}

fn create_cluster() -> Result<CassCluster,CassError> {unsafe{
    let cluster = CassCluster::new();
    cluster.set_contact_points("127.0.0.1")
}}

fn insert_into_basic(mut session: CassSession, key:&str, basic:&mut Basic) -> Result<(CassSession,CassFuture),CassError> {unsafe{
    let query="INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
    let statement = CassStatement::new(query, 6);
    let _ = statement.bind_string(0, str2cass_string(key));
    let _ = statement.bind_bool(1, basic.bln);
    let _ = statement.bind_float(2, basic.flt);
    let _ = statement.bind_double(3, basic.dbl);
    let _ = statement.bind_int32(4, basic.i32);
    let _ = statement.bind_int64(5, basic.i64);
    let future = session.execute_statement(&statement).wait().unwrap();
    Ok((session,future))
}}

fn select_from_basic(mut session:CassSession, key:&str, basic:&mut Basic) -> Result<(CassSession,CassFuture),CassError> {unsafe{
    let query = "SELECT * FROM examples.basic WHERE key = ?";
    let statement = CassStatement::new(query, 1);
    let key = key.as_ptr() as *const i8;
    let _ = statement.bind_string(0, CassString::init(key));
    match session.execute_statement(&statement).wait() {
        Ok(mut future) => {
            let result = future.get_result();
            let mut iterator = result.iter();
            if iterator.next() {
                let row = iterator.get_row();
                let ref mut b_bln = basic.bln;
                let ref mut b_dbl = basic.dbl;
                let ref mut b_flt = basic.flt;
                let ref mut b_i32 = basic.i32;
                let ref mut b_i64 = basic.i64;
                let _ = row.get_column(1).get_bool(b_bln );
                let _ = row.get_column(2).get_double(b_dbl);
                let _ = row.get_column(3).get_float(b_flt);
                let _ = row.get_column(4).get_int32(b_i32);
                let _ = row.get_column(5).get_int64(b_i64);
            }
            Ok((session,future))
        }
        Err(_) => panic!("error")
        
    }
}}

fn main() {unsafe{
    let cluster = &mut create_cluster().unwrap();
    let input = &mut Basic{bln:cass_true, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };

    match CassSession::new().connect(cluster).wait() {
        Ok(_) => {
            let mut session = CassSession::new();
            let output = &mut Basic{bln:0,flt:0f32,dbl:0f64,i32:0,i64:0};
            session.execute_statement(&CassStatement::new("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };",0));
            session.execute_statement(&CassStatement::new("CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));",0));
            let (session,_) = insert_into_basic(session, "test", input).unwrap();
            let (mut session,_) = select_from_basic(session, "test", output).unwrap();
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
}}
