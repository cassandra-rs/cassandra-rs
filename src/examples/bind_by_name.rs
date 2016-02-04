#[macro_use(stmt)]
extern crate cassandra;
use cassandra::*;
use std::str::FromStr;

#[derive(Clone,Copy,Debug,PartialEq)]
struct Basic {
    bln: bool,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn insert_into_basic(session: &mut Session, prepared: PreparedStatement, key: &str, basic: Basic)
                     -> Result<CassResult, CassError> {
    println!("key={:?}", key);
    let mut statement = prepared.bind();

    try!(statement.bind_by_name("key", key));
    try!(statement.bind_by_name("bln", basic.bln));
    try!(statement.bind_by_name("flt", basic.flt));
    try!(statement.bind_by_name("dbl", basic.dbl));
    try!(statement.bind_by_name("i32", basic.i32));
    try!(statement.bind_by_name("i64", basic.i64));

    session.execute(&statement).wait()
}

unsafe fn select_from_basic(session: &mut Session, prepared: &PreparedStatement, key: &str) -> Result<Basic, CassError> {
    let mut statement = prepared.bind();
    statement.bind_string_by_name("key", key).unwrap();
    match session.execute(&statement).wait() {
        Ok(result) => {
            println!("{:?}", result);
            match result.iter().next() {
                Some(row) => {
                    Ok(Basic {
                        bln: try!(row.get_col_by_name("bln")),
                        dbl: try!(row.get_col_by_name("dbl")),
                        flt: try!(row.get_col_by_name("flt")),
                        i32: try!(row.get_col_by_name("i32")),
                        i64: try!(row.get_col_by_name("i64")),
                    })
                }
                None => unimplemented!(),
            }
        }
        Err(_) => unimplemented!(),
    }
}

fn main() {
    unsafe {
        let mut cluster = Cluster::new();
        cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();

        match cluster.connect() {
            Ok(ref mut session) => {
                let input = Basic {
                    bln: true,
                    flt: 0.001f32,
                    dbl: 0.0002,
                    i32: 1,
                    i64: 2,
                };
                session.execute(&stmt!("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'3\' };"))
                       .wait()
                       .unwrap();
                session.execute(&stmt!("CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt \
                                        float, dbl double,i32 int, i64 bigint, PRIMARY KEY (key));"))
                       .wait()
                       .unwrap();
                match session.prepare("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, \
                                       ?, ?, ?);")
                             .unwrap()
                             .wait() {
                    Ok(insert_prepared) => {
                        insert_into_basic(session, insert_prepared, "prepared_test", input).unwrap();
                    }
                    Err(err) => println!("error: {:?}", err),
                }
                match session.prepare("SELECT * FROM examples.basic WHERE key = ?").unwrap().wait() {
                    Ok(ref mut select_prepared) => {
                        let output = select_from_basic(session, &select_prepared, "prepared_test").unwrap();
                        assert_eq!(input, output);
                        println!("results matched: {:?}", output);
                    }
                    Err(err) => println!("err: {:?}", err),
                }
            }
            Err(err) => println!("couldn't connect: {:?}", err),
        }
    }
}
