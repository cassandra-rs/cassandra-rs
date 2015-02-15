extern crate cql_ffi;
use cql_ffi::*;

#[derive(Copy)]
struct Basic {
    bln:cass_bool_t,
    flt:cass_float_t,
    dbl:cass_double_t,
    i32:cass_int32_t,
    i64:cass_int64_t
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

unsafe fn prepare_query(session:&mut CassSession, query:CassString) -> Result<CassPrepared,CassError> {

    let future = &mut session.prepare(query);
    future.wait();
    let rc = match future.error_code() {
        Ok(()) => future.get_prepared(),
        _ => {print_error(future);panic!();}
    };
    future.free();
    Ok(rc)
}

//fixme row key sent is null?
unsafe fn insert_into_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:Basic) -> Result<(),CassError> {
    let mut statement = prepared.bind();
    println!("key={:?}",str2cass_string(key));
    let _ = statement.bind_string_by_name(str2ref("key"), str2cass_string(key));
    let _ = statement.bind_bool_by_name("BLN".as_ptr() as *const i8, basic.bln);
    let _ = statement.bind_float_by_name("FLT".as_ptr() as *const i8, basic.flt);
    let _ = statement.bind_double_by_name("\"dbl\"".as_ptr() as *const i8, basic.dbl);
    let _ = statement.bind_int32_by_name("i32".as_ptr() as *const i8, basic.i32);
    let _ = statement.bind_int64_by_name("I64".as_ptr() as *const i8, basic.i64);
    let mut future = session.execute(statement);
    future.wait();
    let result = future.error_code();
    future.free();
    statement.free();
    result
}

unsafe fn select_from_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:&mut Basic) -> Result<(),CassError> {
    let mut statement = prepared.bind();
    let _ = statement.bind_string_by_name("key".as_ptr() as *const i8, str2cass_string(key));
    let mut future = session.execute(statement);
    future.wait();
    match future.error_code() {
        Ok(()) => {
            let mut result = future.get_result();
            let mut iterator = result.iter();
            if iterator.next() {
                let row = iterator.get_row();
                let _ = row.get_column_by_name("BLN".as_ptr() as *const i8).get_bool(&mut basic.bln);
                let _ = row.get_column_by_name("dbl".as_ptr() as *const i8).get_double(&mut basic.dbl);
                let _ = row.get_column_by_name("flt".as_ptr() as *const i8).get_float(&mut basic.flt);
                let _ = row.get_column_by_name("\"i32\"".as_ptr() as *const i8).get_int32(&mut basic.i32);
                let _ = row.get_column_by_name("i64".as_ptr() as *const i8).get_int64(&mut basic.i64);
            }
            result.free();
            iterator.free();
        }
        Err(_) => panic!("err")
    }
    future.free();
    statement.free();
    Ok(())
}

fn main() {unsafe{
    let (ref mut cluster,ref mut session) = (create_cluster(),CassSession::new());
    match connect_session(session, cluster) {
        Ok(()) => {},
        Err(err) => {
            cluster.free();
            session.free();
            panic!("couldn't connect: {:?}",err);
        }
    }    
    let input = Basic{bln:cass_true, flt:0.001f32, dbl:0.0002, i32:1, i64:2 };
    let mut output = Basic{bln:cass_false, flt:0f32, dbl:0.0, i32:0, i64:0 };
    let insert_query = CassString::init("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);".as_ptr() as *const i8);
    let select_query = CassString::init("SELECT * FROM examples.basic WHERE key = ?".as_ptr() as *const i8);
    let _ = execute_query(session, "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
    let _ = execute_query(session, "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double,i32 int, i64 bigint, PRIMARY KEY (key));");
    match prepare_query(session, insert_query) {
        Ok(ref mut insert_prepared) => {
            let _ = insert_into_basic(session, insert_prepared, "prepared_test", input);
            insert_prepared.free();
        },
        Err(_) => {panic!("error")}
    }
    match prepare_query(session, select_query) {
        Ok(ref mut select_prepared) => {
            let _ = select_from_basic(session, select_prepared, "prepared_test", &mut output);
            assert!(input.bln == output.bln);
            assert!(input.flt == output.flt);
            assert!(input.dbl == output.dbl);
            assert!(input.i32 == output.i32);
            assert!(input.i64 == output.i64);
            select_prepared.free();
        },
        Err(_) => {panic!("err")}
    }
    let mut close_future = session.close();
    close_future.wait();
    close_future.free();
    cluster.free();
    session.free();
}}
