#[macro_use(stmt)]
extern crate cassandra;
use cassandra::*;
use errors::*;
use std::str::FromStr;

static CREATE_KEYSPACE: &'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                        \'SimpleStrategy\', \'replication_factor\': \'1\' };";
static CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl \
                                     double, i32 int, i64 bigint, PRIMARY KEY (key));";
static INSERT_QUERY: &'static str = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, \
                                     ?);";
static SELECT_QUERY: &'static str = "SELECT * FROM examples.basic WHERE key = ?";

#[derive(Debug,PartialEq)]
struct Basic {
    bln: bool,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn insert_into_basic(session: &mut Session, key: &str, basic: &mut Basic) -> Result<()> {
    println!("Creating statement");
    let mut statement = stmt!(INSERT_QUERY);
    statement.bind(0, key)?;
    statement.bind(1, basic.bln)?;
    statement.bind(2, basic.flt)?;
    statement.bind(3, basic.dbl)?;
    statement.bind(4, basic.i32)?;
    statement.bind(5, basic.i64)?;

    println!("Executing insert statement");
    session.execute(&statement).wait()?;
    println!("Insert execute OK");
    Ok(())
}


fn select_from_basic(session: &mut Session, prepared: &PreparedStatement, key: &str, basic: &mut Basic) -> Result<()> {
    let mut statement = prepared.bind();
    statement.bind_string(0, key)?;
    let mut future = session.execute(&statement);
    match future.wait() {
        Ok(result) => {
            println!("{:?}", result);
            for row in result.iter() {
                basic.bln = row.get_col(1)?;
                basic.dbl = row.get_col(2)?;
                basic.flt = row.get_col(3)?;
                basic.i32 = row.get_col(4)?;
                basic.i64 = row.get_col(5)?;
            }
            Ok(())
        }
        Err(err) => panic!("{:?}", err),
    }
}

fn main() {
    let mut cluster = Cluster::default();
    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();
    cluster.set_protocol_version(CqlProtocol::THREE).unwrap();

    match cluster.connect() {
        Ok(ref mut session) => {

            let mut input = Basic {
                bln: true,
                flt: 0.001f32,
                dbl: 0.0002f64,
                i32: 1,
                i64: 2,
            };
            let mut output = Basic {
                bln: false,
                flt: 0f32,
                dbl: 0f64,
                i32: 0,
                i64: 0,
            };
            println!("Executing create keyspace");
            session.execute(&stmt!(CREATE_KEYSPACE)).wait().unwrap();
            println!("Creating table");
            session.execute(&stmt!(CREATE_TABLE)).wait().unwrap();

            println!("Basic insertions");
            insert_into_basic(session, "prepared_test", &mut input).unwrap();
            println!("Preparing");
            match session.prepare(SELECT_QUERY).unwrap().wait() {
                Ok(prepared) => {
                    select_from_basic(session, &prepared, "prepared_test", &mut output).unwrap();
                    println!("input: {:?}\nouput: {:?}", input, output);
                    assert_eq!(input, output);
                }
                Err(err) => panic!(err),
            }
        }
        err => println!("{:?}", err),
    }
}
