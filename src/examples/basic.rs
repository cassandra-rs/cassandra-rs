#![feature(collections)]

extern crate collections;
extern crate cql_ffi;

use collections::string::String;

use cql_ffi::*;

#[derive(Debug)]
struct Basic {
    bln:cass_bool_t,
    flt:cass_float_t,
    dbl:cass_double_t,
    i32:cass_int32_t,
    i64:cass_int64_t,
}

fn print_error(future:&mut CassFuture) {unsafe{
    let message = future.error_message();
    let message = String::from_raw_parts(message.0.data as *mut u8,message.0.length as usize, message.0.length as usize);    
    println!("Error: {:?}", message);
}}

fn create_cluster() -> CassCluster {unsafe{
    let mut cluster = CassCluster::new();
    let _ = cluster.set_contact_points("127.0.0.1".as_ptr() as *const i8);
    cluster 
}}

fn connect_session(session:&mut CassSession, cluster:&mut CassCluster) -> Result<(),CassError> {unsafe{
    let mut future = session.connect(cluster);
    future.wait();
    let err = future.error_code();
    future.free();
    err
}}

fn execute_query(session: &mut CassSession, query: &str) -> Result<(),CassError> {unsafe{
    let query=str2cass_string(query);
    println!("{:?}",query.0.length);
   // println!("{:?}", query);
    let statement = CassStatement::new(query, 0);
    let mut future = session.execute(statement);
    future.wait();
    let _ = future.error_code();
    let result = future.error_code();
    future.free();
    result
}}

fn insert_into_basic(session: &mut CassSession, key:&str, basic:&mut Basic) -> Result<(),CassError> {unsafe{
    let query=str2cass_string("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
    let statement = CassStatement::new(query, 6);
    let _ = statement.bind_string(0, str2cass_string(key));
    let _ = statement.bind_bool(1, basic.bln);
    let _ = statement.bind_float(2, basic.flt);
    let _ = statement.bind_double(3, basic.dbl);
    let _ = statement.bind_int32(4, basic.i32);
    let _ = statement.bind_int64(5, basic.i64);
    let mut future = session.execute(statement);
    future.wait();
    let result = future.error_code();
    future.free();
    statement.free();
    result
}}

fn select_from_basic(session:&mut CassSession, key:&str, basic:&mut Basic) -> Result<(),CassError> {unsafe{
    let query = str2cass_string("SELECT * FROM examples.basic WHERE key = ?");
    let statement = CassStatement::new(query, 1);
    let key = key.as_ptr() as *const i8;
    let _ = statement.bind_string(0, CassString::init(key));
    let mut future = session.execute(statement);
    future.wait();
    let _ = match future.error_code() {
        Ok(_) => {
            let mut result = future.get_result();
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
                result.free();
                iterator.free();
            }
        },
        Err(_) => panic!("error")
        
    };
    future.free();
    statement.free();
    Ok(())
}}

fn main() {unsafe{
    let cluster = &mut create_cluster();
    let session = &mut CassSession::new();
    let input = &mut Basic{bln:cass_true, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };

    let session_future = connect_session(session,cluster);    

    match session_future {
        Ok(()) => {
            let output = &mut Basic{bln:0,flt:0f32,dbl:0f64,i32:0,i64:0};
            let _ = execute_query(session, "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };");
            let _ = execute_query(session, "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));");
            let _ = insert_into_basic(session, "test", input);
            let _ = select_from_basic(session, "test", output);
            println!("{:?}",input);
            println!("{:?}",output);
            assert!(input.bln == output.bln);
            assert!(input.flt == output.flt);
            assert!(input.dbl == output.dbl);
            assert!(input.i32 == output.i32);
            assert!(input.i64 == output.i64);
            let mut close_future = session.close();
            close_future.wait();
            close_future.free();
        },
        _ => {}
    }
    cluster.free();
    session.free();
}}
