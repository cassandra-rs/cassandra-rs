#[macro_use(stmt)]
extern crate cassandra;

mod help;

use cassandra::*;
use errors::*;


#[derive(Debug,PartialEq,Clone,Copy)]
struct Basic {
    bln: bool,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn insert_into_basic(session: &Session, key: &str, basic: Basic) -> Result<CassResult> {
    let mut statement = stmt!("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
    statement.bind(0, key)?;
    statement.bind(1, basic.bln)?;
    statement.bind(2, basic.flt)?;
    statement.bind(3, basic.dbl)?;
    statement.bind(4, basic.i32)?;
    statement.bind(5, basic.i64)?;
    session.execute(&statement).wait()
}

fn select_from_basic(session: &Session, key: &str) -> Result<Option<Basic>> {
    let mut statement = stmt!("SELECT * FROM examples.basic WHERE key = ?");
    statement.bind_string(0, key)?;
    let result = session.execute(&statement).wait()?;
    println!("Result: \n{:?}\n", result);
    match result.first_row() {
        None => Ok(None),
        Some(row) => {
            Ok(Some(Basic {
                bln: row.get_col(1)?,
                dbl: row.get_col(2)?,
                flt: row.get_col(3)?,
                i32: row.get_col(4)?,
                i64: row.get_col(5)?,
            }))
        }
    }
}

/// Create the table for basic testing.
fn create_basic_table(session: &Session) {
    let table_statement = &stmt!("CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt \
                                  float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));");

    session.execute(table_statement).wait().unwrap();
}

#[test]
fn test_basic_round_trip() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);

    let input = Basic {
        bln: true,
        flt: 0.001f32,
        dbl: 0.0002f64,
        i32: 1,
        i64: 2,
    };

    insert_into_basic(&session, "test", input).unwrap();
    let output = select_from_basic(&session, "test").unwrap().expect("no output from select");

    println!("{:?}", input);
    println!("{:?}", output);

    assert!(input == output);
}
