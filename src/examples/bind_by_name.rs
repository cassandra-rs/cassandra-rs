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

fn create_cluster() -> Result<CassCluster,CassError> {unsafe{
    let cluster = CassCluster::new();
    cluster.set_contact_points("127.0.0.1")
}}

//fixme row key sent is null?
unsafe fn insert_into_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:Basic) -> Result<CassFuture,CassError> {
    let statement = prepared.bind();
    println!("key={:?}",key);
    let _ = statement.bind_string_by_name(str2ref("key"), str2cass_string(key));
    let _ = statement.bind_bool_by_name("BLN".as_ptr() as *const i8, basic.bln);
    let _ = statement.bind_float_by_name("FLT".as_ptr() as *const i8, basic.flt);
    let _ = statement.bind_double_by_name("\"dbl\"".as_ptr() as *const i8, basic.dbl);
    let _ = statement.bind_int32_by_name("i32".as_ptr() as *const i8, basic.i32);
    let _ = statement.bind_int64_by_name("I64".as_ptr() as *const i8, basic.i64);
    session.execute_statement(&statement).wait()
}

unsafe fn select_from_basic(session:&mut CassSession, prepared:&CassPrepared, key:&str, basic:&mut Basic) -> Result<CassFuture,CassError> {
    let statement = prepared.bind();
    let _ = statement.bind_string_by_name("key".as_ptr() as *const i8, str2cass_string(key));
    match session.execute_statement(&statement).wait() {
        Ok(mut future) => {
            let result = future.get_result();
            let mut iterator = result.iter();
            if iterator.next() {
                let row = iterator.get_row();
                let _ = row.get_column_by_name("BLN").get_bool(&mut basic.bln);
                let _ = row.get_column_by_name("dbl").get_double(&mut basic.dbl);
                let _ = row.get_column_by_name("flt").get_float(&mut basic.flt);
                let _ = row.get_column_by_name("\"i32\"").get_int32(&mut basic.i32);
                let _ = row.get_column_by_name("i64").get_int64(&mut basic.i64);
            }
            Ok(future)
        }
        Err(_) => panic!("err")
    }
}

fn main() {unsafe{
    let cluster = &mut create_cluster().unwrap();
    let mut session = CassSession::new();
    match session.connect(cluster).wait() {
        Ok(_) => {
                let input = Basic{bln:cass_true, flt:0.001f32, dbl:0.0002, i32:1, i64:2 };
                let mut output = Basic{bln:cass_false, flt:0f32, dbl:0.0, i32:0, i64:0 };
                let insert_query = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
                let select_query = "SELECT * FROM examples.basic WHERE key = ?";
                session.execute_statement(&CassStatement::new("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };",0));
                session.execute_statement(&CassStatement::new("CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double,i32 int, i64 bigint, PRIMARY KEY (key));",0));
                match session.prepare(insert_query).wait() {
                    Ok(ref mut insert_prepared) => {
                        let _ = insert_into_basic(&mut session, &mut insert_prepared.get_prepared(), "prepared_test", input);
                    },
                    Err(_) => {panic!("error")}
                }
                match session.prepare(select_query).wait() {
                    Ok(ref mut select_prepared) => {
                        let _ = select_from_basic(&mut session, &select_prepared.get_prepared(), "prepared_test", &mut output);
                        assert!(input.bln == output.bln);
                        assert!(input.flt == output.flt);
                        assert!(input.dbl == output.dbl);
                        assert!(input.i32 == output.i32);
                        assert!(input.i64 == output.i64);
                    },
                    Err(_) => {panic!("err")}
                }
                session.close().wait().unwrap();
            },
        Err(err) => {
            panic!("couldn't connect: {:?}",err);
        }
    }    
}}
